use crate::{buffer::Buffer, header::Mode, side_info::SideInfo, Header};

const SLEN_TABLE: [[u8; 2]; 16] = [
    [0, 0],
    [0, 1],
    [0, 2],
    [0, 3],
    [3, 0],
    [1, 1],
    [1, 2],
    [1, 3],
    [2, 1],
    [2, 2],
    [2, 3],
    [3, 1],
    [3, 2],
    [3, 3],
    [4, 2],
    [4, 3],
];

#[derive(Debug)]
pub struct Frame {
    pub header: Header,
    crc: Option<u16>,
    pub side_info: SideInfo,
    length_byte: usize,
    pub granules_data: Vec<GranuleData>,
}

#[derive(Debug)]
pub struct GranuleData {
    scale_factor: Vec<u8>,
    huffman_code: Vec<u8>,
}

impl GranuleData {
    fn new() -> Self {
        Self {
            scale_factor: Vec::new(),
            huffman_code: Vec::new(),
        }
    }
}

impl Frame {
    pub fn create_from_buffer(buffer: &mut Buffer) -> Self {
        let header = Header::create_from_buffer(buffer);
        let crc = if header.error_protection {
            Some(buffer.get_bits(16).unwrap() as u16)
        } else {
            None
        };

        let side_info = SideInfo::create_from_buffer(buffer, &header.mode).unwrap();
        let length_byte = 144000
            * (header.get_bitrate().unwrap() / header.get_frequency().unwrap()) as usize
            + header.padding_bit as usize;

        let mut granules_data: Vec<GranuleData> = Vec::new();

        for _ in 0..side_info.granule_channels.len() {
            granules_data.push(GranuleData::new());
        }

        Self {
            header,
            crc,
            side_info,
            length_byte,
            granules_data,
        }
    }

    fn decode_scalefactors(&mut self, buffer: &mut Buffer) {
        for (i, granule) in self.side_info.granule_channels.iter().enumerate() {
            let slen1 = SLEN_TABLE[granule.scalefac_compress as usize][0] as u32;
            let slen2 = SLEN_TABLE[granule.scalefac_compress as usize][1] as u32;

            let start_pos = buffer.pos;

            if granule.block_type != 2 {
                for _ in 0..(18 - (granule.mixed_block_flag as usize)) {
                    self.granules_data[i]
                        .scale_factor
                        .push(buffer.get_bits(slen1).unwrap() as u8);
                }

                for _ in 0..18 {
                    self.granules_data[i]
                        .scale_factor
                        .push(buffer.get_bits(slen2).unwrap() as u8);
                }

                for _ in 0..3 {
                    self.granules_data[i].scale_factor.push(0);
                }
            } else {
                let (is_first_half, channel_index) = if self.header.mode == Mode::SingleChannel {
                    (i % 2 == 0, 0)
                } else {
                    (i / 2 == 0, i % 2)
                };

                let scfsi = if is_first_half {
                    0
                } else if self.header.mode == Mode::SingleChannel {
                    self.side_info.scfsi
                } else {
                    self.side_info.scfsi & (0xf0 >> (4 * (channel_index)))
                };

                if scfsi & 8 == 8 {
                    for band_index in 0..6 {
                        let scale_factor =
                            self.granules_data[channel_index].scale_factor[band_index];
                        self.granules_data[i].scale_factor.push(scale_factor);
                    }
                } else {
                    for band_index in 0..6 {
                        self.granules_data[i]
                            .scale_factor
                            .push(buffer.get_bits(slen1).unwrap() as u8);
                    }
                }

                if scfsi & 4 == 4 {
                    for band_index in 6..11 {
                        let scale_factor =
                            self.granules_data[channel_index].scale_factor[band_index];
                        self.granules_data[i].scale_factor.push(scale_factor);
                    }
                } else {
                    for band_index in 6..11 {
                        self.granules_data[i]
                            .scale_factor
                            .push(buffer.get_bits(slen1).unwrap() as u8);
                    }
                }

                if scfsi & 2 == 2 {
                    for band_index in 11..16 {
                        let scale_factor =
                            self.granules_data[channel_index].scale_factor[band_index];
                        self.granules_data[i].scale_factor.push(scale_factor);
                    }
                } else {
                    for band_index in 11..16 {
                        self.granules_data[i]
                            .scale_factor
                            .push(buffer.get_bits(slen2).unwrap() as u8);
                    }
                }

                if scfsi & 1 == 1 {
                    for band_index in 16..21 {
                        let scale_factor =
                            self.granules_data[channel_index].scale_factor[band_index];
                        self.granules_data[i].scale_factor.push(scale_factor);
                    }
                } else {
                    for band_index in 16..21 {
                        self.granules_data[i]
                            .scale_factor
                            .push(buffer.get_bits(slen2).unwrap() as u8);
                    }
                }

                self.granules_data[i].scale_factor.push(0);
            }
        }
    }

    pub fn decode_main_data(&mut self, buffer: &mut Buffer) {
        if self.side_info.main_data_begin != 0 {
            buffer.set_pos(self.header.pos - self.side_info.main_data_begin as usize);
        }

        self.decode_scalefactors(buffer);
    }

    fn check_crc(&self) {
        todo!("Implement CRC check for");
    }
}
