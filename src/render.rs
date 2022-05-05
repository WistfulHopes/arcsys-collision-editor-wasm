use std::io::Cursor;

use bevy::core_pipeline::{
    draw_3d_graph, node, AlphaMask3d, Opaque3d, RenderTargetClearColors, Transparent3d,
};
use bevy::prelude::*;
use bevy::reflect::TypeUuid;
use bevy::render::camera::{ActiveCamera, CameraTypePlugin, RenderTarget};
use bevy::render::render_asset::RenderAssets;
use bevy::render::render_graph::{NodeRunError, RenderGraph, RenderGraphContext, SlotValue};
use bevy::render::render_phase::RenderPhase;
use bevy::render::render_resource::{
    Buffer, BufferDescriptor, BufferUsages, CommandEncoderDescriptor, Extent3d, ImageCopyBuffer,
    ImageDataLayout, MapMode, TextureDescriptor, TextureDimension, TextureFormat, TextureUsages,
};
use bevy::render::renderer::{RenderContext, RenderDevice, RenderQueue};
use bevy::render::{RenderApp, RenderStage};
use image::io::Reader as ImageReader;
use bevy_egui::{egui::{epaint::{ColorImage}}};

use crate::MyApp;

#[derive(Component, Default)]
pub struct CaptureCamera;

pub const CAPTURE_IMAGE_HANDLE: HandleUntyped =
    HandleUntyped::weak_from_u64(Image::TYPE_UUID, 13373934772014884929);

// The name of the final node of the first pass.
pub const CAPTURE_DRIVER: &str = "capture_driver";

pub fn setup_capture(
    mut commands: Commands,
    mut images: ResMut<Assets<Image>>,
    mut clear_colors: ResMut<RenderTargetClearColors>,
    render_device: Res<RenderDevice>,
) {
    let size = Extent3d {
        width: 512,
        height: 512,
        ..Default::default()
    };

    // This is the texture that will be rendered to.
    let mut image = Image {
        texture_descriptor: TextureDescriptor {
            label: None,
            size,
            dimension: TextureDimension::D2,
            format: TextureFormat::Rgba8UnormSrgb,
            mip_level_count: 1,
            sample_count: 1,
            usage: TextureUsages::TEXTURE_BINDING
                | TextureUsages::COPY_DST
                | TextureUsages::COPY_SRC
                | TextureUsages::RENDER_ATTACHMENT,
        },
        ..Default::default()
    };
    image.resize(size);

    let image_handle = images.set(CAPTURE_IMAGE_HANDLE, image);

    let padded_bytes_per_row = RenderDevice::align_copy_bytes_per_row(512) * 4;

    let size = padded_bytes_per_row as u64 * 512;

    let output_cpu_buffer = render_device.create_buffer(&BufferDescriptor {
        label: Some("Output Buffer"),
        size,
        usage: BufferUsages::MAP_READ | BufferUsages::COPY_DST,
        mapped_at_creation: false,
    });

    let render_target = RenderTarget::Image(image_handle);
    clear_colors.insert(render_target.clone(), Color::GRAY);
    commands
        .spawn_bundle(PerspectiveCameraBundle::<CaptureCamera> {
            camera: Camera {
                target: render_target,
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(0.0, 1.0, 3.0))
            .looking_at(Vec3::new(0.0, 1.0, 0.0), Vec3::Y),
            ..PerspectiveCameraBundle::new()
        })
        .insert(Capture {
            buf: output_cpu_buffer,
        });
}

// Add 3D render phases for CAPTURE_CAMERA.
pub fn extract_camera_phases(
    mut commands: Commands,
    cap: Query<&Capture>,
    active: Res<ActiveCamera<CaptureCamera>>,
    ui_state: ResMut<MyApp>,
) {
    if ui_state.boxes_window.reset_image {
        if let Some(entity) = active.get() {
            if let Some(cap) = cap.iter().next() {
                commands
                    .get_or_spawn(entity)
                    .insert_bundle((
                        RenderPhase::<Opaque3d>::default(),
                        RenderPhase::<AlphaMask3d>::default(),
                        RenderPhase::<Transparent3d>::default(),
                    ))
                    .insert(Capture {
                        buf: cap.buf.clone(),
                    });
            }
        }
    }
}

// A node for the first pass camera that runs draw_3d_graph with this camera.
pub struct CaptureCameraDriver {
    pub buf: Option<Buffer>,
}

impl bevy::render::render_graph::Node for CaptureCameraDriver {
    fn run(
        &self,
        graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let render = world.get_resource::<MyApp>();
        if render.is_some() {
            let render = render.unwrap().boxes_window.reset_image;
            if render {
                let gpu_images = world.get_resource::<RenderAssets<Image>>().unwrap();
    
                if let Some(camera_3d) = world.resource::<ActiveCamera<CaptureCamera>>().get() {
                    graph.run_sub_graph(draw_3d_graph::NAME, vec![SlotValue::Entity(camera_3d)])?;
        
                    let gpu_image = gpu_images.get(&CAPTURE_IMAGE_HANDLE.typed()).unwrap();
                    let mut encoder = render_context
                        .render_device
                        .create_command_encoder(&CommandEncoderDescriptor::default());
                    let padded_bytes_per_row =
                        RenderDevice::align_copy_bytes_per_row((gpu_image.size.width) as usize) * 4;
        
                    let texture_extent = Extent3d {
                        width: gpu_image.size.width as u32,
                        height: gpu_image.size.height as u32,
                        depth_or_array_layers: 1,
                    };
        
                    if let Some(buf) = &self.buf {
                        encoder.copy_texture_to_buffer(
                            gpu_image.texture.as_image_copy(),
                            ImageCopyBuffer {
                                buffer: buf,
                                layout: ImageDataLayout {
                                    offset: 0,
                                    bytes_per_row: Some(
                                        std::num::NonZeroU32::new(padded_bytes_per_row as u32).unwrap(),
                                    ),
                                    rows_per_image: None,
                                },
                            },
                            texture_extent,
                        );
                        let render_queue = world.get_resource::<RenderQueue>().unwrap();
                        render_queue.submit(std::iter::once(encoder.finish()));
                    }
                }    
            }    
        };
        Ok(())
    }
    fn update(&mut self, world: &mut World) {
        let render = world.get_resource::<MyApp>();
        if render.is_some() {
            for cap in world.query::<&mut Capture>().iter_mut(world) {
                self.buf = Some(cap.buf.clone());
            }
        }
    }
}

pub fn save_img(
    cap: Query<&Capture>, 
    render_device: Res<RenderDevice>,
    mut ui_state: ResMut<MyApp>,
) {
    if ui_state.boxes_window.reset_image {
        if let Some(cap) = cap.iter().next() {
            let large_buffer_slice = cap.buf.slice(..);
            render_device.map_buffer(&large_buffer_slice, MapMode::Read);
            {
                let large_padded_buffer = large_buffer_slice.get_mapped_range();

                let reader = ImageReader::new(
                    Cursor::new(large_padded_buffer)
                )
                .with_guessed_format().expect("Failed to read image!");
                let image = match reader.decode() {
                    Ok(image) => {
                        let size = [image.width() as _, image.height() as _];
                        let image_buffer = image.to_rgba8();
                        let pixels = image_buffer.as_flat_samples();
                        Ok(ColorImage::from_rgba_unmultiplied(
                            size,
                            pixels.as_slice(),
                        ))
                    }
                    Err(e) => Err(e),
                };
                if image.is_ok() {
                    ui_state.boxes_window.image = Some(image.unwrap());
                }
            }
            cap.buf.unmap();
        }    
    }
}

#[derive(Component)]
pub struct Capture {
    pub buf: Buffer,
}

pub struct CapturePlugin;
impl Plugin for CapturePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(CameraTypePlugin::<CaptureCamera>::default())
            .add_startup_system(setup_capture)
            .add_system(save_img);

        let render_app = app.sub_app_mut(RenderApp);

        // This will add 3D render phases for the capture camera.
        render_app.add_system_to_stage(RenderStage::Extract, extract_camera_phases);

        let mut graph = render_app.world.get_resource_mut::<RenderGraph>().unwrap();

        // Add a node for the capture.
        graph.add_node(CAPTURE_DRIVER, CaptureCameraDriver { buf: None });

        // The capture's dependencies include those of the main pass.
        graph
            .add_node_edge(node::MAIN_PASS_DEPENDENCIES, CAPTURE_DRIVER)
            .unwrap();

        // Insert the capture node: CLEAR_PASS_DRIVER -> CAPTURE_DRIVER -> MAIN_PASS_DRIVER
        graph
            .add_node_edge(node::CLEAR_PASS_DRIVER, CAPTURE_DRIVER)
            .unwrap();
        graph
            .add_node_edge(CAPTURE_DRIVER, node::MAIN_PASS_DRIVER)
            .unwrap();
    }
}