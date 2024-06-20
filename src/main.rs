#![allow(unused_variables, dead_code)]

mod constant;
mod error;
mod header;
mod helper;

use header::Header;
use helper::get_buffer_from_file;

fn main() {
    let buffer = get_buffer_from_file("mp3-examples/test_data_1mb.mp3");
    let header = Header::from_buffer(&buffer);
    println!("{}", header);
}
