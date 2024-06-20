use std::fmt::Display;

use crate::constant;
use crate::error;

#[derive(PartialEq, Debug)]
pub enum Version {
    MPEG1,
    MPEG2,
}

#[derive(PartialEq, Debug)]
pub enum Layer {
    Layer1,
    Layer2,
    Layer3,
}

#[derive(PartialEq, Debug)]
pub enum Mode {
    Stereo,
    JointStereo,
    DualChannel,
    SingleChannel,
}

#[derive(Debug, PartialEq)]
pub struct Header {
    pub sync_word: u16,
    pub version: Version,
    pub layer: Layer,
    pub error_protection: bool,
    pub bitrate: u8,
    pub frequency: u8,
    pub padding_bit: bool,
    pub private_bit: bool,
    pub mode: Mode,
    pub intensity_stereo: bool,
    pub ms_stereo: bool,
    pub copy_right: bool,
    pub copy_of_original: bool,
    pub emphasis: u8,
}

impl Display for Layer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let layer = match self {
            Self::Layer3 => "Layer III",
            Self::Layer2 => "Layer II",
            Self::Layer1 => "Layer I",
        };

        write!(f, "{layer}")
    }
}

impl Layer {
    fn decode_layer(layer: u8) -> Result<Layer, error::ErrorType> {
        match layer {
            1 => Ok(Layer::Layer3),
            2 => Ok(Layer::Layer2),
            3 => Ok(Layer::Layer1),
            _ => Err(error::ErrorType::UnknownLayer),
        }
    }
}

impl Display for Version {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let version = match self {
            Self::MPEG1 => "MPEG-1",
            Self::MPEG2 => "MPEG-2",
        };

        write!(f, "{version}")
    }
}

impl Version {
    fn decode_version(version: u8) -> Result<Version, error::ErrorType> {
        match version {
            0 => Ok(Version::MPEG2),
            1 => Ok(Version::MPEG1),
            _ => Err(error::ErrorType::UnknownVersion),
        }
    }
}

impl Display for Mode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mode = match self {
            Self::Stereo => "Stereo",
            Self::JointStereo => "Joint Stereo",
            Self::DualChannel => "Dual Channel",
            Self::SingleChannel => "Single Channel",
        };

        write!(f, "{mode}")
    }
}

impl Mode {
    fn decode_mode(mode: u8) -> Result<Mode, error::ErrorType> {
        match mode {
            0 => Ok(Mode::Stereo),
            1 => Ok(Mode::JointStereo),
            2 => Ok(Mode::DualChannel),
            3 => Ok(Mode::SingleChannel),
            _ => Err(error::ErrorType::UnknownMode),
        }
    }
}

impl Header {
    fn validate_header(&self) -> Result<(), error::ErrorType> {
        if self.sync_word == 0xfff && self.layer == Layer::Layer3 {
            return Ok(());
        }

        Err(error::ErrorType::InvalidHeader)
    }

    pub fn from_buffer(buffer: &Vec<u8>) -> Self {
        let sync_word = ((buffer[0] as u16) << 4) | (buffer[1] as u16 & 0xf0) >> 4;
        let version = Version::decode_version((buffer[1] & 8) >> 3).unwrap();
        let layer = Layer::decode_layer((buffer[1] & 0b110) >> 1).unwrap();
        let error_protection = (buffer[1] & 1) == 0;
        let bitrate = (buffer[2] & 0xf0) >> 4;
        let frequency = (buffer[2] & 0xc) >> 2;
        let padding_bit = ((buffer[2] & 0x10) >> 1) == 1;
        let private_bit = buffer[2] & 1 == 1;
        let mode = Mode::decode_mode((buffer[3] & 0xc0) >> 6).unwrap();
        let intensity_stereo = (buffer[3] & 0x20) >> 5 == 1;
        let ms_stereo = (buffer[3] & 0x10) >> 4 == 1;
        let copy_right = (buffer[3] & 0b1000) >> 3 == 1;
        let copy_of_original = (buffer[3] & 0b100) >> 2 == 0;
        let emphasis = buffer[3] & 0b11;

        Self {
            sync_word,
            version,
            layer,
            error_protection,
            bitrate,
            frequency,
            padding_bit,
            private_bit,
            mode,
            intensity_stereo,
            ms_stereo,
            copy_right,
            copy_of_original,
            emphasis,
        }
    }

    fn get_bitrate(&self) -> Result<u16, error::ErrorType> {
        if self.version == Version::MPEG1 && self.layer == Layer::Layer3 {
            return Ok(constant::BITRATE_MPEG1_LAYER3[self.bitrate as usize - 1]);
        }

        if self.version == Version::MPEG2
            && (self.layer == Layer::Layer3 || self.layer == Layer::Layer2)
        {
            return Ok(constant::BITRATE_MPEG2_LAYER3[self.bitrate as usize - 1]);
        }

        Err(error::ErrorType::UnknownBitrate)
    }

    fn get_frequency(&self) -> Result<u16, error::ErrorType> {
        if self.version == Version::MPEG1 {
            return Ok(constant::FREQUENCY_MPEG1[self.frequency as usize]);
        }

        if self.version == Version::MPEG2 {
            return Ok(constant::FREQUENCY_MPEG2[self.frequency as usize]);
        }

        Err(error::ErrorType::UnknownFrequency)
    }
}

impl Display for Header {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "
    Version: {}
    Layer: {}
    Error Protection: {}
    Bitrate: {}kb/sec
    Frequency: {}Hz
    Padding: {}
    Set Private Bit: {}
    Channel Mode: {}
    Intensity Stereo: {}
    M/S Stereo: {}
    Copy Right: {},
    Copy of Original: {},
    emphasis: {},
            ",
            self.version,
            self.layer,
            self.error_protection,
            self.get_bitrate().unwrap(),
            self.get_frequency().unwrap(),
            self.padding_bit,
            self.private_bit,
            self.mode,
            if self.intensity_stereo { "On" } else { "Off" },
            if self.ms_stereo { "On" } else { "Off" },
            if self.copy_right {
                "Has Copy Right"
            } else {
                "Not Set"
            },
            self.copy_of_original,
            match self.emphasis {
                00 => "None",
                01 => "50/15 ms",
                10 => "Reserved",
                11 => "CCIT J.17",
                _ => "",
            }
        )
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::helper::get_buffer_from_file;

    #[test]
    fn test_header_from_buffer() {
        let buffer = get_buffer_from_file("mp3-examples/test_data_1mb.mp3");
        let header = Header::from_buffer(&buffer);

        assert_eq!(
            header,
            Header {
                sync_word: 0xfff,
                version: Version::MPEG1,
                layer: Layer::Layer3,
                error_protection: false,
                bitrate: 0b1001,
                frequency: 0,
                padding_bit: false,
                private_bit: false,
                mode: Mode::JointStereo,
                intensity_stereo: true,
                ms_stereo: false,
                copy_right: false,
                copy_of_original: false,
                emphasis: 0,
            }
        );
    }
}
