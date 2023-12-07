use encdec::prelude::*;

// cargo run --bin main
fn main() {
    // let s = "Hello á 鄬 𗜀";
    let s = "Hello á 鄬 𘚟";
    // From strring to vec of u32
    let v: Vec<u32> = s.chars().map(|c| c as u32).collect();
    println!("{:?}", v);
    utf8::utf8_encode(s);
}
