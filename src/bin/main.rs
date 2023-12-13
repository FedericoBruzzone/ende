// use encdec::prelude::*;
use encdec::ucs2;
use encdec::unicode;
use encdec::utf16;
use encdec::utf8;

// cargo run --bin main
fn main() {
    // let s2: String = "\xa9".to_string();
    // print!("s2: {:x?}\n", s2.as_bytes());

    // let v: Vec<u32> = vec![0xa9, 0xffff, 0x10451]; // Array of code point
    // let v: Vec<u32> = vec![0xa9]; // Array of code point
    // // println!("v: {:?}", v);
    // // println!("v: {:x?}", v);
    // let enc_utf8 = utf8::utf8_encode(&v);
    // println!("encoded utf-8: {:x?}", enc_utf8);
    // let enc_ucs2 = ucs2::ucs2_encode(&v);
    // println!("encoded ucs-2: {:x?}", enc_ucs2);
    // let dec = utf8::utf8_decode(&enc_utf8);
    // println!("UNICODE code point: {:x?}", dec);

    let v: Vec<u32> = vec![0x10001]; // Array of code points in unicode
    let utf8_vec: Vec<u8> = utf8::encode_in_utf8(&v);
    // println!("v encoded utf-8: {:x?}", utf8_vec);
    let enc_ucs2 = ucs2::ucs2_encode(&v);
    // println!("v encoded ucs-2: {:x?}", enc_ucs2);
    let unicode_vec_from_ucs2 = ucs2::ucs2_decode(&enc_ucs2);
    // println!("UNICODE code point from ucs-2: {:x?}", unicode_vec_from_ucs2);
    let unicode_vec_from_utf8 = utf8::decode_from_utf8(&utf8_vec);
    println!(
        "UNICODE code point from utf-8: {:x?}",
        unicode_vec_from_utf8
    );
    utf8::print_utf8(&utf8_vec);
    utf8::print_utf8_b(&utf8_vec);

    println!("UTF8 ------------------------------------");

    let v = vec![0x0024];
    let utf8_vec = utf8::encode_in_utf8(&v);
    let unicode_vec_from_utf8 = utf8::decode_from_utf8(&utf8_vec);
    utf8::print_utf8_b(&utf8_vec);

    let v = vec![0x0418];
    let utf8_vec = utf8::encode_in_utf8(&v);
    let unicode_vec_from_utf8 = utf8::decode_from_utf8(&utf8_vec);
    utf8::print_utf8_b(&utf8_vec);

    let v = vec![0x20AC];
    let utf8_vec = utf8::encode_in_utf8(&v);
    let unicode_vec_from_utf8 = utf8::decode_from_utf8(&utf8_vec);
    utf8::print_utf8_b(&utf8_vec);

    let v = vec![0x10348];
    let utf8_vec = utf8::encode_in_utf8(&v);
    let unicode_vec_from_utf8 = utf8::decode_from_utf8(&utf8_vec);
    utf8::print_utf8_b(&utf8_vec);
    unicode::print_unicode_b(&v);

    println!("UTF16 ------------------------------------");
    let v = vec![0x10001];
    let utf16_vec = utf16::encode_in_utf16(&v);
    println!("utf16_vec: {:x?}", utf16_vec);
    let unicode_vec_from_utf16 = utf16::decode_from_utf16(&utf16_vec);
    println!("unicode_vec_from_utf16: {:x?}", unicode_vec_from_utf16);

    unicode::print_unicode_b(&unicode_vec_from_utf16);

    // let v2: Vec<u32> = vec![0x10001, 0x10002]; // Array of code point in unicode
    // let enc2 = utf8::encode_in_utf8(&v2);
    // utf8::print_utf8(&enc2);
    // unicode::print_unicode(&v2);

    // let s_test = "💖";
    // let a = [0xf0, 0x9f, 0x92, 0x96]; // Valid UTF-8 sequence of code points
    // println!("s_test: {:x?}", s_test.as_bytes());
    // println!("a: {:?}", String::from_utf8(a.to_vec()).unwrap());

    // let s: String = "©".to_string();
    // print!("s: {:x?}\n", s.as_bytes());
    // let res: Vec<u8> = utf8::utf8_encode(&s);
    // println!("res: {:x?}", res);
}
