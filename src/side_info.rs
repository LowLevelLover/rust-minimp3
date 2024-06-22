use crate::{buffer::Buffer, error::ErrorType, header::Mode};

#[derive(Debug)]
pub struct SideInfo {
    pub main_data_begin: u16,
    pub private_bits: u8,
    pub scfsi: u8,
    pub granule_channels: Vec<GranuleInfo>,
}

#[derive(Debug)]
pub struct GranuleInfo {
    pub part_23_length: u16,
    pub big_values: u16,
    pub global_gain: u8,
    pub scalefac_compress: u8,
    pub windows_switching: bool,
    pub block_type: u8,
    pub mixed_block_flag: bool,
    pub table_select: [u8; 3],
    pub subblock_gain: [u8; 3],
    pub region_count: [u8; 3],
    pub preflag: bool,
    pub scalefac_scale: bool,
    pub count1_table_select: bool,
}

impl GranuleInfo {
    fn new() -> Self {
        Self {
            part_23_length: 0,
            big_values: 0,
            global_gain: 0,
            scalefac_compress: 0,
            windows_switching: false,
            block_type: 0,
            mixed_block_flag: false,
            table_select: [0, 0, 0],
            subblock_gain: [0, 0, 0],
            region_count: [0, 0, 0],
            preflag: false,
            scalefac_scale: false,
            count1_table_select: false,
        }
    }
}

impl SideInfo {
    pub fn create_from_buffer(buffer: &mut Buffer, mode: &Mode) -> Result<Self, ErrorType> {
        let is_mono = *mode == Mode::SingleChannel;

        let main_data_begin = buffer.get_bits(9).unwrap() as u16;
        let private_bits: u8 = buffer.get_bits(if is_mono { 5 } else { 3 }).unwrap() as u8;
        let scfsi = buffer.get_bits(if is_mono { 4 } else { 8 }).unwrap() as u8;

        let granules_count: u8 = if is_mono { 2 } else { 4 };
        let mut granules: Vec<GranuleInfo> = Vec::new();
        let mut part_23_sum: usize = 0;

        for _ in 0..granules_count {
            let mut granule = GranuleInfo::new();

            granule.part_23_length = buffer.get_bits(12).unwrap() as u16;
            part_23_sum += granule.part_23_length as usize;

            granule.big_values = buffer.get_bits(9).unwrap() as u16;

            if granule.big_values > 288 {
                return Err(ErrorType::BigValuesOutOfRange);
            }

            granule.global_gain = buffer.get_bits(8).unwrap() as u8;
            granule.scalefac_compress = buffer.get_bits(4).unwrap() as u8;
            granule.windows_switching = buffer.get_bits(1).unwrap() == 1;

            if granule.windows_switching {
                granule.block_type = buffer.get_bits(2).unwrap() as u8;
                if granule.block_type == 0 {
                    return Err(ErrorType::BlockTypeForbidden);
                }

                granule.mixed_block_flag = buffer.get_bits(1).unwrap() == 1;
                granule.table_select[0] = buffer.get_bits(5).unwrap() as u8;
                granule.table_select[1] = buffer.get_bits(5).unwrap() as u8;

                granule.subblock_gain[0] = buffer.get_bits(3).unwrap() as u8;
                granule.subblock_gain[1] = buffer.get_bits(3).unwrap() as u8;
                granule.subblock_gain[2] = buffer.get_bits(3).unwrap() as u8;
            } else {
                granule.table_select[0] = buffer.get_bits(5).unwrap() as u8;
                granule.table_select[1] = buffer.get_bits(5).unwrap() as u8;
                granule.table_select[2] = buffer.get_bits(5).unwrap() as u8;

                granule.region_count[0] = buffer.get_bits(4).unwrap() as u8;
                granule.region_count[1] = buffer.get_bits(3).unwrap() as u8;
                granule.region_count[2] = 255;
            }

            granule.preflag = buffer.get_bits(1).unwrap() == 1;
            granule.scalefac_scale = buffer.get_bits(1).unwrap() == 1;
            granule.count1_table_select = buffer.get_bits(1).unwrap() == 1;

            granules.push(granule);
        }

        if part_23_sum + buffer.pos > buffer.total_bits + main_data_begin as usize * 8 {
            return Err(ErrorType::Overflow);
        }

        Ok(SideInfo {
            main_data_begin,
            private_bits,
            scfsi,
            granule_channels: granules,
        })
    }
}
