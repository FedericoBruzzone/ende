use encdec::prelude::*;

// cargo run --bin main
fn main() {

    // let s2: String = "\xa9".to_string();
    // print!("s2: {:x?}\n", s2.as_bytes());

    // let v: Vec<u32> = vec![0xa9, 0xffff, 0x10451]; // Array of code point
    let v: Vec<u32> = vec![0xa9]; // Array of code point
    println!("v: {:?}", v);
    println!("v: {:x?}", v);
    let tmp = utf8::utf8_encode(&v);
    println!("tmp: {:x?}", tmp);

    let v: Vec<u32> = vec![0x10001]; // Array of code point
    println!("v: {:?}", v);
    println!("v: {:x?}", v);
    let tmp = utf8::utf8_encode(&v);
    println!("encoded utf-8: {:x?}", tmp);
    assert_eq!(tmp, vec![0xf0, 0x90, 0x80, 0x81]);

    let decoded = utf8::utf8_decode(&tmp);
    println!("decoded ucs-2: {:x?}", decoded);

    let s_test = "💖";
    let a = [0xf0, 0x9f, 0x92, 0x96]; // Valid UTF-8 sequence of code points
    println!("s_test: {:x?}", s_test.as_bytes());
    println!("a: {:?}", String::from_utf8(a.to_vec()).unwrap());

    // let s: String = "©".to_string();
    // print!("s: {:x?}\n", s.as_bytes());
    // let res: Vec<u8> = utf8::utf8_encode(&s);
    // println!("res: {:x?}", res);

}
