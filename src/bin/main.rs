use encdec::prelude::*;

// cargo run --bin main
fn main() {
    // let s = "Hello á 鄬 𗜀";
    // let s = "Hello á 鄬 𘚟";

    let s = "hello";
    let res: Vec<u8> = utf8::utf8_encode(&s);
    utf8::print_encoding(&res);


}
