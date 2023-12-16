/*!
UTF-16 encoding and decoding.

# Encoding
A unicode code point is represented using two or four bytes in UTF-16, depending its value.
* If the unicode code point is less than 0xFFFF, it is represented using [a word of 16 bits (two bytes)](#two-bytes-one-word).
* If the unicode code point is greater than or equal to 0xFFFF, it is represented using a surrogate-pair [two words of 16 bits (four bytes)](#four-bytes-two-words---surrogate-pair).

# Decoding
A UTF-16 code point is decoded into a unicode code point using the following rules.
* If the UTF-16 code point is less than 0xD800 or greater than 0xDBFF and less than 0xFFFF, it is a unicode code point.
* If the UTF-16 code point is between 0xD800 and 0xDBFF, it is a [surrogate pair](#surrogate-pair), so two UTF-16 code points are required to represent a unicode code point.

When a unicode code point is represented using four bytes, the

## Representation

### Two bytes (one word)

### Four bytes (two words) - Surrogate pair

## Surrogate pair
A surrogate pair is a pair of UTF-16 code points that together encode a single unicode code point.
* The first UTF-16 code point is a high surrogate and the second UTF-16 code point is a low surrogate.
* The high surrogate is in the range 0xD800 to 0xDBFF.
* The low surrogate is in the range 0xDC00 to 0xDFFF.
* The unicode code point is in the range 0x10000 to 0x10FFFF.
* The unicode code point is calculated as follows:
    * Subtract 0xD800 from the high surrogate to form a 10-bit value in the range 0x0000 to 0x03FF.
    * Subtract 0xDC00 from the low surrogate to form a 10-bit value in the range 0x0000 to 0x03FF.
    * Add 0x10000 to the high 10-bit value, resulting in a 20-bit value in the range 0x10000 to 0x10FFFF.
    * Add the low 10-bit value to the 20-bit value, resulting in the final value.
    * The final value is the unicode code point.
*/

fn encode_code_point(unicode_cp: u32) -> Vec<u16> {
    if unicode_cp < 0xFFFF {
        // unicode_cp: 0x0251 -> 0b0000_0010_0101_0001
        return vec![unicode_cp as u16];
    }

    let mut byte_vec: Vec<u16> = Vec::new();
    let extra = unicode_cp - 0x10000;
    byte_vec.push((((extra >> 10) & 0x3FF) + 0xD800) as u16);
    let unicode_cp = 0xDC00 | unicode_cp & 0x3FF;
    byte_vec.push(unicode_cp as u16);

    byte_vec
}

fn decode_symbol(utf16_cp: &Vec<u16>, i: usize) -> Option<(u32, usize)> {
    if i > utf16_cp.len() {
        panic!("Index out of bounds");
    }

    if i == utf16_cp.len() {
        return None;
    }

    let mut code_point: u32;
    let mut offset: usize = 0;

    code_point = utf16_cp[i] as u32;
    offset += 1;
    if code_point < 0xD800 || code_point > 0xDBFF {
        return Some((code_point as u32, offset));
    }

    let extra = utf16_cp[i + 1] as u32;
    offset += 1;
    if (extra & 0xFC00) == 0xDC00 {
        code_point = (((code_point & 0x3FF) << 10) + (extra & 0x3FF)) + 0x10000;
        return Some((code_point, offset));
    } else {
        panic!("Invalid UCS-2 sequence");
    }
}

/// Pretty print the UTF-16 code points in hexadecimal, (binary) and decimal.
///
/// # Parameters
/// * `utf16_cp`: [`Vec<u16>`] - A vector of UTF-16 code points.
/// * `binary_flag`: [`bool`] - A flag to print the binary representation of the UTF-16 code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in UTF-16.
fn print_utf16_vec<T: AsRef<Vec<u16>>>(utf16_cp: T, binary_flag: bool) {
    let v: Vec<u16> = utf16_cp.as_ref().to_vec();
    let string_repr: String = String::from_utf16(&v).unwrap();
    let binary_repr: Vec<String> = v.iter().map(|x| format!("{:08b}", x)).collect();
    println!();
    println!(
        "--------------- UTF-16 of \"{}\" ---------------",
        string_repr
    );
    println!("Hex: {:x?}", v);
    if binary_flag {
        println!("Bin: {:?}", binary_repr);
    }
    println!("Dec: {:?}", v);
    println!(
        "{}{}",
        "-".repeat(44),
        "-".repeat(string_repr.chars().count())
    );
    println!();
}

// ============================================================================
// ================================ Public API ================================
// ============================================================================

/// Pretty print the UTF-16 encoding in hexadecimal and decimal of a vector of UTF-16 code points.
///
/// # Parameters
/// * `uft16_cp`: [`Vec<u16>`] - A vector of UTF-16 code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in UTF-16.
///
/// # Example
/// ```rust
/// use encdec::prelude::*;
/// let v: Vec<u16> = vec![0xD800, 0xDC00];
/// utf16::print_utf16(&v);
/// ```
/// **Output**
/// ```text
/// --------------- UTF-16 encoding of "𐀁" ---------------
/// Hex: [d800, dc00]
/// Dec: [55296, 56320]
/// ------------------------------------------------------
pub fn print_utf16<T: AsRef<Vec<u16>>>(utf16_cp: T) {
    print_utf16_vec(utf16_cp, false);
}

/// Pretty print the UTF-16 encoding in hexadecimal and decimal of a vector of UTF-16 code points.
///
/// # Parameters
/// * `uft16_cp`: [`Vec<u16>`] - A vector of UTF-16 code points.
///
/// # Note
/// The bytes printed in hexadecimal are code points in UTF-16.
///
/// # Example
/// ```rust
/// use encdec::prelude::*;
/// let v: Vec<u16> = vec![0xD800, 0xDC00];
/// utf16::print_utf16(&v);
/// ```
/// **Output**
/// ```text
/// --------------- UTF-16 encoding of "𐀀" ---------------
/// Hex: [d800, dc00]
/// Bin: ["1101100000000000", "1101110000000000"]
/// Dec: [55296, 56320]
/// ------------------------------------------------------
pub fn print_utf16_b<T: AsRef<Vec<u16>>>(utf16_cp: T) {
    print_utf16_vec(utf16_cp, true);
}

/// Encode a vector of unicode code points into a vector of UTF-16 code points.
///
/// # Parameters
/// * `unicode_cp`: [`Vec<u32>`] - A vector of unicode code points.
///
/// # Returns
/// A [`Vec<u16>`] containing the UTF-16 code points.
///
/// # Panics
/// * If the input vector (`unicode_cp`) of unicode code points contains invalid unicode code points.
///
/// # Example
/// ```rust
/// use encdec::prelude::*;
/// let v: Vec<u32> = vec![0x10001]; // Array of code points in unicode
/// let enc: Vec<u16> = utf16::encode_in_utf16(&v);
/// assert_eq!(enc, vec![0xD800, 0xDC01]);
/// ```
pub fn encode_in_utf16<T: AsRef<Vec<u32>>>(unicode_cp: T) -> Vec<u16> {
    let unicode_cp: Vec<u32> = unicode_cp.as_ref().to_vec();
    let len: usize = unicode_cp.len();
    let mut utf16_cp: Vec<u16> = Vec::new();
    for i in 0..len {
        let cp = unicode_cp[i];
        utf16_cp.append(&mut encode_code_point(cp));
    }
    utf16_cp
}

/// Decode a vector of UTF-16 code points into a vector of unicode code points.
///
/// # Parameters
/// * `utf16_cp`: [`Vec<u16>`] - A vector of UTF-16 code points.
///
/// # Returns
/// A [`Vec<u32>`] containing the unicode code points.
///
/// # Panics
/// * If the input vector (`utf16_cp`) of UTF-16 code points contains invalid code points.
/// * If the input vector (`utf16_cp`) of UTF-16 code points contains invalid continuation bytes.
///
/// # Example
/// ```rust
/// use encdec::prelude::*;
/// let v: Vec<u16> = vec![0xD800, 0xDC01]; // Array of code points in UTF-16
/// let dec: Vec<u32> = utf16::decode_from_utf16(&v);
/// assert_eq!(dec, vec![0x10001]);
/// ```
pub fn decode_from_utf16<T: AsRef<Vec<u16>>>(utf16_cp: T) -> Vec<u32> {
    let utf16_cp: Vec<u16> = utf16_cp.as_ref().to_vec();
    let len: usize = utf16_cp.len();
    let mut i = 0;
    let mut unicode_cp: Vec<u32> = Vec::new();
    while i < len {
        let (cp, offset) = decode_symbol(&utf16_cp, i).unwrap();
        i += offset;
        unicode_cp.push(cp);
    }
    unicode_cp
}
