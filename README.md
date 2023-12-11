# Encdec

A library for encoding/decoding unicode/utf-8/ucs-2(utf-16) code points.

This crate is working in progress, the goal is to be able to have different types of encoding and decoding from and to different types of code points.

## Examples

**Encoding/decoding unicode/UTF-8 code points**

```rust
let v: Vec<u32> = vec![0x10348 /*...*/]; // Array of code points in unicode
let utf8_vec: Vec<u8> = utf8::encode_in_utf8(&v);
utf8::print_utf8_b(&utf8_vec);
let unicode_vec: Vec<u32> = utf8::decode_from_utf8(&utf8_vec);
unicode::print_unicode_b(&v);
```

```shell
--------------- UTF-8 of "𐍈" ---------------
Hex: [f0, 90, 8d, 88]
Bin: ["11110000", "10010000", "10001101", "10001000"]
Dec: [240, 144, 141, 136]
--------------------------------------------

--------------- UNICODE of "𐍈" ---------------
Hex: [10348]
Bin: ["10000001101001000"]
Dec: [66376]
----------------------------------------------
```

## Contributing

Contributions to this project are welcome! If you have any suggestions, improvements, or bug fixes, feel free to submit a pull request.

## License

This repository is licensed under the [GNU General Public License (GPL)](https://www.gnu.org/licenses/gpl-3.0.html). Please review the license file provided in the repository for more information regarding the terms and conditions of the GPL license.

## Contact

- Email: [federico.bruzzone.i@gmail.com] or [federico.bruzzone@studenti.unimi.it]

