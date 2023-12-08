use encdec::prelude::*;

// cargo run --bin main
fn main() {
    // let s = "Hello á 鄬 𗜀";
    // let s = "hello";
    let s = "Hello á".to_string();

    let res: Vec<u32> = ucs2::ucs2_decode(&s);
    println!("res: {:?}", res);
    // utf8::print_encoding(&(res.iter().flat_map(|&x| x.to_ne_bytes().to_vec()).collect::<Vec<u8>>()));

    let res: String = ucs2::ucs2_encode(&res);
    println!("res: {:?}", res);

    let res: Vec<u8> = utf8::utf8_encode(&s);
    println!("res: {:?}", res);
    // utf8::print_encoding(&res);

    let res: String = utf8::utf8_decode(&res);
    println!("res: {:?}", res);

}
