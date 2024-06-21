use crate::{buffer::Buffer, side_info::SideInfo, Header};

#[derive(Debug)]
pub struct Frame {
    pub header: Header,
    pub crc: Option<u16>,
    pub side_info: SideInfo,
    length_byte: usize,
}

impl Frame {
    pub fn create_from_buffer(buffer: &mut Buffer) -> Self {
        let header = Header::create_from_buffer(buffer);
        let crc = if header.error_protection {
            Some(buffer.get_bits(16).unwrap() as u16)
        } else {
            None
        };

        let side_info = SideInfo::create_from_buffer(buffer, &header.mode);
        let length_byte =
            144000 * (header.bitrate / header.frequency) as usize + header.padding_bit as usize;

        Self {
            header,
            crc,
            side_info,
            length_byte,
        }
    }

    fn check_crc(&self) {
        todo!("Implement CRC check for");
    }
}
