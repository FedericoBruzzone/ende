// use crate::utf8::ucs2;
use crate::prelude::*;

use std::usize;

fn create_byte(code_point: u32, shift: u32) -> u8 {
    let mut byte: u8 = ((code_point >> shift) & 0x3F) as u8;
    byte |= 0x80;
    byte
}

fn check_code_point(code_point: u32) {
    if code_point >= 0xD800 && code_point <= 0xDFFF {
        panic!("Invalid UCS-2 sequence {}", code_point.to_string());
    }
}

fn encode_code_point(code_point: u32) -> String {
    if (code_point & 0xFFFFFF80) == 0 {
        return (code_point as u8 as char).to_string();
    }
    let mut byte_string: String = String::new();

    if (code_point & 0xFFFFF800) == 0 {
        let s: String = ((((code_point >> 6) & 0x1F) | 0xC0) as u8 as char).to_string();
        byte_string.push_str(&s);
    } else if (code_point & 0xFFFF0000) == 0 {
        check_code_point(code_point);
        let s: String = ((((code_point >> 12) & 0x0F) | 0xE0) as u8 as char).to_string();
        byte_string.push_str(&s);
    } else if (code_point & 0xFFE00000) == 0 {
        let s: String = ((((code_point >> 12) & 0x0F) | 0xE0) as u8 as char).to_string();
        byte_string.push_str(&s);
        byte_string.push_str(&create_byte(code_point, 12).to_string());
        byte_string.push_str(&create_byte(code_point, 6).to_string());
    }

    byte_string
}

pub fn utf8_encode<T: AsRef<str>>(s: T) -> String {
    let code_points: Vec<u32> = ucs2::ucs2_decode(s);
    println!("The code_points is {:?}", code_points);
    let len_code_points: usize = code_points.len();
    let mut byte_string: String = String::new();
    for i in 0..len_code_points {
        let code_point: u32 = code_points[i];
        byte_string.push_str(&encode_code_point(code_point as u32));
    }
    byte_string
}

