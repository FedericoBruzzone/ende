// use crate::utf8::ucs2;
use crate::prelude::*;

use std::io;
use std::u8;
use std::usize;

fn vec_to_bytes(vec: &Vec<u8>) -> io::Result<Vec<u8>> {
    let mut buffer = Vec::with_capacity(vec.len());
    unsafe {
        let ptr = vec.as_ptr() as *const u8;
        std::ptr::copy_nonoverlapping(ptr, buffer.as_mut_ptr(), buffer.capacity());
        buffer.set_len(buffer.capacity());
    }
    Ok(buffer)
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
        println!("Code point: {}", code_point);
        let s = (((code_point >> 6) & 0x1F) | 0xC0) as u8;
        byte_vec.push(s);
    } else if (code_point & 0xFFFF0000) == 0 {
        check_code_point(code_point);
        let s = (((code_point >> 12) & 0x0F) | 0xE0) as u8;
        byte_vec.push(s);
    } else if (code_point & 0xFFE00000) == 0 {
        let s = (((code_point >> 12) & 0x0F) | 0xE0) as u8;
        byte_vec.push(s);
        byte_vec.push(create_byte(code_point, 12));
        byte_vec.push(create_byte(code_point, 6));
    }
    byte_vec.push(((code_point & 0x3F) | 0x80) as u8);
    byte_vec
}

// ========================= Public API =========================

pub fn print_encoding<T: AsRef<Vec<u8>>>(byte_vec: T) {
    let byte_vec: Vec<u8> = byte_vec.as_ref().to_vec();
    let binary_repr: Vec<String> = byte_vec.iter().map(|x| format!("{:08b}", x)).collect();
    println!("In decimal:     {:?}", byte_vec);
    println!("In hexadecimal: {:x?}", byte_vec);
    println!("In binary:      {:?}", binary_repr);

}

pub fn utf8_encode<T: AsRef<str>>(s: T) -> Vec<u8> {
    let code_points: Vec<u32> = ucs2::ucs2_decode(s);
    println!("Code points: {:?}", code_points);
    let len_code_points: usize = code_points.len();
    let mut byte_vec: Vec<u8> = Vec::new();
    for i in 0..len_code_points {
        let code_point: u32 = code_points[i];
        byte_vec.append(&mut encode_code_point(code_point));
    }
    vec_to_bytes(&byte_vec).unwrap()
}

