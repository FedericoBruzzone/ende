// use crate::utf8::ucs2;
use crate::prelude::*;

use std::usize;

fn create_byte(code_point: u32, shift: u32) -> u8 {
    let mut byte: u8 = ((code_point >> shift) & 0x3F) as u8;
    byte |= 0x80;
    byte
}

fn encode_code_point(code_point: u32) -> String {
    if (code_point & 0xFFFFFF80) == 0 {
        return (code_point as u8 as char).to_string();
    }
    let mut byte_string: String = String::new();

    byte_string
}

pub fn utf8_encode<T: AsRef<str>>(s: T) -> String {
    let code_points: Vec<u32> = ucs2::ucs2_decode(s);
    let len_code_points: usize = code_points.len();
    let mut byte_string: String = String::new();
    for i in 0..len_code_points {
        let code_point: u32 = code_points[i];
        byte_string.push_str(&encode_code_point(code_point));
    }
    byte_string
}
