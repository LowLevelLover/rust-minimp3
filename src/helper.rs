use crate::error;
use std::{fs, io::Read};

pub fn get_buffer_from_file(path: &str) -> Vec<u8> {
    let mut file = fs::File::open(path).expect("Cannot open mp3 file.");
    let mut data: Vec<u8> = Vec::new();

    file.read_to_end(&mut data)
        .expect("Cannot read data from file");

    data
}

pub fn get_32bits(buffer: &Vec<u8>, index: u32, n: u32) -> Result<u32, error::ErrorType> {
    if (index + n) / 8 > buffer.len() as u32 {
        return Err(error::ErrorType::OutOfIndex);
    }

    if n > 32 {
        return Err(error::ErrorType::Overflow);
    }

    if n == 0 {
        return Ok(0);
    }

    let start_byte_index = index / 8;
    let end_byte_index = (index + n - 1) / 8;

    let start_offset: u32 = ((0xff >> (index as u8 % 8)) as u32) << ((n - 1) / 8) * 8;
    let end_offset: u32 = 0xff << (7 - ((index + n - 1) as u8 % 8));

    let mask: u32 = if start_byte_index == end_byte_index {
        (start_offset) & (end_offset)
    } else {
        (start_offset) | (end_offset)
    };

    let mut result: u32 = 0;

    for i in 0..=(n as usize - 1) / 8 {
        result |= (buffer[i + start_byte_index as usize] as u32) << (((n as usize - 1) / 8 - i) * 8)
    }

    Ok((result & mask) >> (7 - ((index + n - 1) as u8 % 8)))
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_buffer_from_file_1mb() {
        let buffer = get_buffer_from_file("mp3-examples/test_data_1mb.mp3");

        assert_eq!(buffer.len() / (1024 * 1024), 1);
    }

    #[test]
    fn test_get_32bits() {
        let buffer = vec![255, 255, 230, 210];

        assert_eq!(get_32bits(&buffer, 16, 8).unwrap(), 230);
    }
}
