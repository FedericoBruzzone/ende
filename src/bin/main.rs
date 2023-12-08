use encdec::prelude::*;

// cargo run --bin main
fn main() {
    // let s = "Hello á 鄬 𗜀";
    // let s = "hello";

    let s = "Hello á".to_string();
    let res: Vec<u8> = utf8::utf8_encode(&s); // utf8::print_encoding(&res);
    println!("res: {:?}", res);
}
