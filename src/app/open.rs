use arcsys::{ggst::pac::{GGSTPac}};

pub fn open_file(file_buf: Vec<u8>) -> Result<GGSTPac, String> {
    match GGSTPac::parse(&file_buf)
    {
        Ok(file) => return Ok(file),
        Err(e) => return Err(format!("{}", e)),
    };
}