use crate::{buffer::Buffer, header::Mode};

pub const SUBTOTAL_GRANULE_MONO: u8 = 59;
pub const TOTAL_NUMBER_BITS_MONO: u8 = 136;
pub const SUBTOTAL_NOT_NORMAL_BLOCKS_MONO: u8 = 136;

const MAIN_DATA_BEGIN_BITS: u32 = 9;
const PRIVATE_BITS_MONO: u32 = 5;
const PRIVATE_BITS_DUAL: u32 = 3;

// [scfsi, part2_3_length, big_values, global_gain, scalefac_compress, windows_switching, block_type, mixed_block_flag, table_select, subblock_gain, region0_count, region1_count, preflag, scalefac_scale, count1_table_select]

const MONO_BITS: [u32; 15] = [4, 12, 9, 8, 4, 1, 2, 1, 5, 9, 4, 3, 1, 1, 1];

#[derive(Debug)]
pub struct SideInfo {
    pub main_data_begin: u16,
    pub private_bits: u8,
    pub scfsi: u8,
    pub part2_3_length: u32,
    pub big_values: u32,
    pub global_gain: u16,
    pub scalefac_compress: u8,
    pub windows_switching: u8,
    pub block_type: u8,
    pub mixed_block_flag: u8,
    pub table_select: u32,
    pub subblock_gain: u32,
    pub region0_count: u8,
    pub region1_count: u8,
    pub preflag: u8,
    pub scalefac_scale: u8,
    pub count1_table_select: u8,
}

impl SideInfo {
    pub fn create_from_buffer(buffer: &mut Buffer, mode: &Mode) -> Self {
        let bits = if *mode == Mode::SingleChannel {
            MONO_BITS
        } else {
            MONO_BITS.map(|el| el * 2)
        };

        let main_data_begin = buffer.get_bits(9).unwrap() as u16;
        let private_bits: u8 = buffer
            .get_bits(if *mode == Mode::SingleChannel { 5 } else { 3 })
            .unwrap() as u8;

        let scfsi = buffer.get_bits(bits[0]).unwrap() as u8;
        let part2_3_length = buffer.get_bits(bits[1]).unwrap();
        let big_values = buffer.get_bits(bits[2]).unwrap();
        let global_gain = buffer.get_bits(bits[3]).unwrap() as u16;
        let scalefac_compress = buffer.get_bits(bits[4]).unwrap() as u8;
        let windows_switching = buffer.get_bits(bits[5]).unwrap() as u8;
        let block_type = if windows_switching != 0 {
            buffer.get_bits(bits[6]).unwrap() as u8
        } else {
            buffer.move_pos(bits[6] as isize);
            0
        };
        let mixed_block_flag = if windows_switching != 0 {
            buffer.get_bits(bits[7]).unwrap() as u8
        } else {
            buffer.move_pos(bits[7] as isize);
            0
        };
        let table_select = buffer
            .get_bits(bits[8] * if windows_switching != 0 { 3 } else { 2 })
            .unwrap();
        let subblock_gain = if windows_switching != 0 {
            buffer.get_bits(bits[9]).unwrap()
        } else {
            buffer.move_pos(bits[9] as isize);
            0
        };
        let region0_count = buffer.get_bits(bits[10]).unwrap() as u8;
        let region1_count = buffer.get_bits(bits[11]).unwrap() as u8;
        let preflag = buffer.get_bits(bits[12]).unwrap() as u8;
        let scalefac_scale = buffer.get_bits(bits[13]).unwrap() as u8;
        let count1_table_select = buffer.get_bits(bits[14]).unwrap() as u8;

        SideInfo {
            main_data_begin,
            private_bits,
            scfsi,
            part2_3_length,
            big_values,
            global_gain,
            scalefac_compress,
            windows_switching,
            block_type,
            mixed_block_flag,
            table_select,
            subblock_gain,
            region0_count,
            region1_count,
            preflag,
            scalefac_scale,
            count1_table_select,
        }
    }
}
