use crate::pixel_info::Pixel;
use crate::ImageDecoder;
use byteorder::ReadBytesExt;

pub struct R8;

impl ImageDecoder for R8 {
    fn decode_step(data: &mut &[u8]) -> std::io::Result<Pixel> {
        Ok(Pixel::builder().rad(data.read_u8()?).build())
    }
}
