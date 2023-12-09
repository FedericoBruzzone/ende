// use crate::utf8::ucs2;
use crate::prelude::*;

use std::u8;
use std::usize;

fn vec_to_bytestring(vec: &Vec<u8>) -> String {
    vec.into_iter().map(|b| *b as char).collect()
}

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

fn encode_code_point(code_point: u32) -> Vec<u8> {
    if (code_point & 0xFFFFFF80) == 0 {
        return vec![code_point as u8];
    }
    let mut byte_vec: Vec<u8> = Vec::new();

    if (code_point & 0xFFFFF800) == 0 {
        let s = (((code_point >> 6) & 0x1F) | 0xC0) as u8;
        byte_vec.push(s);
    } else if (code_point & 0xFFFF0000) == 0 {
        check_code_point(code_point);
        let s = (((code_point >> 12) & 0x0F) | 0xE0) as u8;
        byte_vec.push(s);
    } else if (code_point & 0xFFE00000) == 0 {
        let s = (((code_point >> 18) & 0x07) | 0xF0) as u8;
        byte_vec.push(s);
        byte_vec.push(create_byte(code_point, 12));
        byte_vec.push(create_byte(code_point, 6));
    }
    byte_vec.push(((code_point & 0x3F) | 0x80) as u8);
    byte_vec
}

fn read_next_byte(byte_vec: &Vec<u8>, i: usize) -> u32 {
    if i >= byte_vec.len() {
        panic!("Index out of bounds");
    }
    let continuation_byte: u32 = (byte_vec[i] & 0xFF) as u32;
    if (continuation_byte & 0xC0) == 0x80 {
        return continuation_byte & 0x3F;
    }
    panic!("Invalid continuation byte");
}

fn decode_symbol_starting_from(byte_vec: &Vec<u8>, i: usize) -> Option<(u32, usize)> {
    if i > byte_vec.len() {
        panic!("Index out of bounds");
    }

    if i == byte_vec.len() {
        return None;
    }

    let mut code_point: u32;
    let mut offset: usize = 0;

    let byte1: u32 = (byte_vec[i] & 0xFF) as u32;
    code_point = byte1;
    offset += 1;
    if (byte1 & 0x80) == 0 {
        return Some((code_point, offset));
    }

    if (byte1 & 0xE0) == 0xC0 {
        let byte2: u32 = read_next_byte(byte_vec, i + offset);
        code_point = ((byte1 & 0x1F) << 6) | byte2;
        offset += 1;
        if code_point >= 0x80 {
            return Some((code_point, offset));
        } else {
            panic!("Invalid continuation byte");
        }
    }

    if (byte1 & 0xF0) == 0xE0 {
        let byte2: u32 = read_next_byte(byte_vec, i + offset);
        offset += 1;
        let byte3: u32 = read_next_byte(byte_vec, i + offset);
        offset += 1;
        code_point = ((byte1 & 0x0F) << 12) | (byte2 << 6) | byte3;
        if code_point >= 0x0800 {
            check_code_point(code_point);
            return Some((code_point, offset));
        } else {
            panic!("Invalid continuation byte");
        }
    }

    if (byte1 & 0xF8) == 0xF0 {
        let byte2: u32 = read_next_byte(byte_vec, i + offset);
        offset += 1;
        let byte3: u32 = read_next_byte(byte_vec, i + offset);
        offset += 1;
        let byte4: u32 = read_next_byte(byte_vec, i + offset);
        offset += 1;
        code_point = ((byte1 & 0x07) << 0x12) | (byte2 << 0x0C) | (byte3 << 0x06) | byte4;
        if code_point >= 0x010000 && code_point <= 0x10FFFF {
            return Some((code_point, offset));
        }
    }

    panic!("Invalid UTF-8 sequence");
}

// ========================= Public API =========================

pub fn print_encoding<T: AsRef<Vec<u8>>>(byte_vec: T) {
    let byte_vec: Vec<u8> = byte_vec.as_ref().to_vec();
    let binary_repr: Vec<String> = byte_vec.iter().map(|x| format!("{:08b}", x)).collect();
    println!("In decimal:     {:?}", byte_vec);
    println!("In hexadecimal: {:x?}", byte_vec);
    println!("In binary:      {:?}", binary_repr);
}

// Assuming that the vector v contains only valid UCS-2 code points
pub fn utf8_encode<T: AsRef<Vec<u32>>>(v: T) -> Vec<u8> {
    let code_points: Vec<u32> = v.as_ref().to_vec();
    let len_code_points: usize = code_points.len();
    let mut byte_vec: Vec<u8> = Vec::new();
    for i in 0..len_code_points {
        let code_point: u32 = code_points[i];
        byte_vec.append(&mut encode_code_point(code_point));
    }
    byte_vec
}

// pub fn utf8_decode<T: AsRef<Vec<u8>>>(v: T) -> String {
//     let byte_string: String = vec_to_bytestring(v.as_ref());
//     let byte_vec: Vec<u32> = ucs2::ucs2_decode(byte_string);
//     let mut code_points: Vec<u32> = Vec::new();
//     let mut i: usize = 0;
//     while i < byte_vec.len() {
//         let (code_point, offset) = decode_symbol_starting_from(&byte_vec, i).unwrap();
//         i += offset;
//         code_points.push(code_point);
//     }
//     ucs2::ucs2_encode(code_points)
// }

// Assuming that the vector v contains only valid UTF-8 code points
pub fn utf8_decode<T: AsRef<Vec<u8>>>(v: T) -> Vec<u32> {
    let byte_vec = v.as_ref();
    let mut code_points: Vec<u32> = Vec::new();
    let mut i: usize = 0;
    while i < byte_vec.len() {
        let (code_point, offset) = decode_symbol_starting_from(&byte_vec, i).unwrap();
        i += offset;
        code_points.push(code_point);
    }
    code_points
}

