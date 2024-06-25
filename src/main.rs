#![allow(unused_variables, dead_code)]

mod buffer;
mod constant;
mod error;
mod frame;
mod header;
mod side_info;

use buffer::Buffer;
use header::Header;

use crate::frame::Frame;

fn main() {
    let mut buffer = Buffer::create_buffer_from_file("mp3-examples/test_data_1mb.mp3");
    buffer.set_pos(14192);

    let mut frame = Frame::create_from_buffer(&mut buffer);
    frame.decode_main_data(&mut buffer);

    println!("header: {}\n\n", &frame.header);
    println!("{:?}\n\n", &frame.side_info);
    println!("{:?}", frame.granules_data);
}
