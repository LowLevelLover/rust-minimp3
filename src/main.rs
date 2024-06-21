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
    // let header = Header::create_from_buffer(&mut buffer);
    let frame = Frame::create_from_buffer(&mut buffer);
    println!("{}", &frame.header);
    println!("{:?}", &frame);
}
