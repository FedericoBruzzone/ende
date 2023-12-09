/*!
UTF-8 encoding and decoding.

# Encoding
A unicode code point is represented using one to four bytes in UTF-8, depending on its value.
* If the unicode code point is in the range `0x0000` to `0x007F`, it is represented using [one byte](#one-byte).
* If the unicode code point is in the range `0x0080` to `0x07FF`, it is represented using [two bytes](#two-bytes).
* If the unicode code point is in the range `0x0800` to `0xFFFF`, it is represented using [three bytes](#three-bytes).
* If the unicode code point is in the range `0x10000` to `0x10FFFF`, it is represented using [four bytes](#four-bytes).

**Note**: The number of `x`s (bits) on the right side of the `0` in the unicode code point are the number
of free bits on the UTF-8 code point.

### One byte

If the eighth bit of the unicode code point is 0, the unicode code point is represented using one byte.

* Unicode code point: `xxxxxxxx|xxxxxxxx|xxxxxxxx|0xxxxxxx`
* UTF-8 code point: `0xxxxxxx`

### Two bytes

If the twelfth bit of the unicode code point is 0, the unicode code point is represented using two bytes.

* Unicode code point: `xxxxxxxx|xxxxxxxx|xxxx0xxx|xxxxxxxx`
* UTF-8 code point: `110xxxxx|10xxxxxx`

### Three bytes

If the seventeenth bit of the unicode code point is 0, the unicode code point is represented using three bytes.

* Unicode code point: `xxxxxxxx|xxxxxxx0|xxxxxxxx|xxxxxxxx`
* UTF-8 code point: `1110xxxx|10xxxxxx|10xxxxxx`

### Four bytes

If the twentysecond bit of the unicode code point is 0, the unicode code point is represented using four bytes.

* Unicode code point: `xxxxxxxx|xx0xxxxx|xxxxxxxx|xxxxxxxx`
* UTF-8 code point: `11110xxx|10xxxxxx|10xxxxxx|10xxxxxx`
*/

// use crate::utf8::ucs2;
// use crate::prelude::*;

fn check_code_point(code_point: u32) {
    if code_point >= 0xD800 && code_point <= 0xDFFF {
        panic!("Invalid UCS-2 sequence {}", code_point.to_string());
    }
}

/// Encode a unicode code point into a vector of UTF-8 code points.
///
/// # Parameters
/// * `code_point`: [`u32`] - A unicode code point.
///
/// # Returns
/// A [`Vec<u8>`] containing the UTF-8 code points.
///
/// # Panics
/// * If the input unicode code point is invalid.
fn encode_code_point(unicode_cp: u32) -> Vec<u8> {
    if (unicode_cp & 0xFFFFFF80) == 0 {
        return vec![unicode_cp as u8];
    }

    let mut byte_vec: Vec<u8> = Vec::new();
    // Check if the code point is representable using two bytes in UTF-8.
    // If so, it is in the range 0x0080 to 0x07FF.
    if (unicode_cp & 0xFFFFF800) == 0 {
        let s = (((unicode_cp >> 6) & 0x1F) | 0xC0) as u8;
        byte_vec.push(s);
    } else if (unicode_cp & 0xFFFF0000) == 0 {
        check_code_point(unicode_cp);
        let s = (((unicode_cp >> 12) & 0x0F) | 0xE0) as u8;
        byte_vec.push(s);
    } else if (unicode_cp & 0xFFE00000) == 0 {
        let s = (((unicode_cp >> 18) & 0x07) | 0xF0) as u8;
        byte_vec.push(s);
        byte_vec.push((((unicode_cp >> 12) & 0x3F) | 0x80) as u8); // Create a continuation byte
        byte_vec.push((((unicode_cp >> 6) & 0x3F) | 0x80) as u8); // Create a continuation byte
    }
    byte_vec.push(((unicode_cp & 0x3F) | 0x80) as u8);
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

fn decode_symbol(byte_vec: &Vec<u8>, i: usize) -> Option<(u32, usize)> {
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

/// Pretty print the unicode code points in hexadecimal, (binary) and decimal of a vector of unicode code points.
///
/// # Parameters
/// * `unicode_cp`: [`Vec<u32>`] - A vector of unicode code points.
/// * `binary_flag`: [`bool`] - A flag to print the binary representation of the unicode code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in unicode.
fn print_utf8_vec<T: AsRef<Vec<u8>>>(utf8_cp: T, binary_flag: bool) {
    let v: Vec<u8> = utf8_cp.as_ref().to_vec();
    let string_repr: String = String::from_utf8(v.clone()).unwrap();
    let binary_repr: Vec<String> = v.iter().map(|x| format!("{:08b}", x)).collect();
    println!();
    println!(
        "--------------- UTF-8 encoding of \"{}\" ---------------",
        string_repr
    );
    println!("Hex: {:x?}", v);
    if binary_flag {
        println!("Bin: {:?}", binary_repr);
    }
    println!("Dec: {:?}", v);
    println!(
        "{}{}",
        "-".repeat(52),
        "-".repeat(string_repr.chars().count())
    );
    println!();
}

// ============================================================================
// ================================ Public API ================================
// ============================================================================

/// Pretty print the UTF-8 encoding in hexadecimal and decimal of a vector of UTF-8 code points.
///
/// # Parameters
/// * `uft8_cp`: [`Vec<u8>`] - A vector of UTF-8 code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in UTF-8.
///
/// # Example
/// ```rust
/// let v: Vec<u8> = vec![0xf0, 0x90, 0x80, 0x81];
/// utf8::print_utf8(&v);
/// ```
/// **Output**
/// ```text
/// --------------- UTF-8 encoding of "𐀁" ---------------
/// Hex: [f0, 90, 80, 81]
/// Dec: [240, 144, 128, 129]
/// ----------------------------------------------------
pub fn print_utf8<T: AsRef<Vec<u8>>>(uft8_cp: T) {
    print_utf8_vec(uft8_cp, false);
}

/// Pretty print the UTF-8 encoding in hexadecimal, binary and decimal of a vector of UTF-8 code points.
///
/// # Parameters
/// * `uft8_cp`: [`Vec<u8>`] - A vector of UTF-8 code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in UTF-8.
///
/// # Example1
/// ```rust
/// let v: Vec<u8> = vec![0xf0, 0x90, 0x80, 0x81];
/// utf8::print_utf8_b(&v);
/// ```
/// **Output**
/// ```text
/// --------------- UTF-8 encoding of "𐀁" ---------------
/// Hex: [f0, 90, 80, 81]
/// Bin: ["11110000", "10010000", "10000000", "10000001"]
/// Dec: [240, 144, 128, 129]
/// -----------------------------------------------------
pub fn print_utf8_b<T: AsRef<Vec<u8>>>(uft8_cp: T) {
    print_utf8_vec(uft8_cp, true);
}

/// Encode a vector of unicode code points into a vector of UTF-8 code points.
///
/// # Parameters
/// * `unicode_cp`: [`Vec<u32>`] - A vector of unicode code points.
///
/// # Returns
/// A [`Vec<u8>`] containing the UTF-8 code points.
///
/// # Panics
/// * If the input vector (`unicode_cp`) of unicode code points contains invalid unicode code points.
///
/// # Example
/// ```rust
/// let v: Vec<u32> = vec![0x10001]; // Array of code points in unicode
/// let enc: Vec<u8> = utf8::encode_in_utf8(&v);
/// assert_eq!(enc, vec![0xf0, 0x90, 0x80, 0x81]);
/// ```
pub fn encode_in_utf8<T: AsRef<Vec<u32>>>(unicode_cp: T) -> Vec<u8> {
    let unicode_cp: Vec<u32> = unicode_cp.as_ref().to_vec();
    let len: usize = unicode_cp.len();
    let mut utf8_cp: Vec<u8> = Vec::new();
    for i in 0..len {
        let cp: u32 = unicode_cp[i];
        utf8_cp.append(&mut encode_code_point(cp));
    }
    utf8_cp
}

/// Decode a vector of UTF-8 code points into a vector of unicode code points.
///
/// # Parameters
/// * `utf8_cp`: [`Vec<u8>`] - A vector of UTF-8 code points.
///
/// # Returns
/// A [`Vec<u32>`] containing the unicode code points.
///
/// # Panics
/// * If the input vector (`utf8_cp`) of UTF-8 code points contains invalid code points.
/// * If the input vector (`utf8_cp`) of UTF-8 code points contains invalid continuation bytes.
/// * If the input vector (`utf8_cp`) of UTF-8 code points contains invalid UTF-8 sequences.
///
/// # Example
/// ```rust
/// let v: Vec<u8> = vec![0xf0, 0x90, 0x80, 0x81]; // Array of code points in UTF-8
/// let dec: Vec<u32> = utf8::decode_form_utf8(&v);
/// assert_eq!(dec, vec![0x10001]);
/// ```
pub fn decode_from_utf8<T: AsRef<Vec<u8>>>(utf8_cp: T) -> Vec<u32> {
    let v = utf8_cp.as_ref();
    let mut code_points: Vec<u32> = Vec::new();
    let mut i: usize = 0;
    while i < v.len() {
        let (code_point, offset) = decode_symbol(&v, i).unwrap();
        i += offset;
        code_points.push(code_point);
    }
    code_points
}
