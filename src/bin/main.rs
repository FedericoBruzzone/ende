use encdec::prelude::*;

// cargo run --bin main
fn main() {
    // let s = "Hello á 鄬 𗜀";
    // let s = "Hello á 鄬 𘚟";
    let s = "hello";

    let bytes: [u8; 6] = [0x68, 0x65, 0x6C, 0x6C, 0x6F, 0x20];

    // Convert the byte array to a string
    let string_from_bytes = String::from_utf8_lossy(&bytes);

    let res = utf8::utf8_encode(&string_from_bytes);
    println!("The string_from_bytes is {}", string_from_bytes);
    println!("The res is {}", res);

    let res2 = utf8::utf8_encode(&bytes);
}
