// UCS-2 is a character encoding standard in which characters are represented
// by a fixed-length 16 bits (2 bytes).
// UCS-2 represents a possible maximum of 65,536 characters,
// or in hexadecimals from 0000h - FFFFh (2 bytes).

pub fn ucs2_decode<T: AsRef<str>>(s: T) -> Vec<u32> {
    let mut v: Vec<u32> = Vec::new();
    let mut iter = s.as_ref().chars();
    let len = iter.clone().count();
    for _ in 0..len {
        let c = iter.next().unwrap();
        if c as u32 >= 0xD800 && c as u32 <= 0xDBFF {
            let c2 = iter.next().unwrap();
            if c2 as u32 >= 0xDC00 && c2 as u32 <= 0xDFFF {
                let extra = ((c as u32 & 0x3FF) << 10) | (c2 as u32 & 0x3FF);
                v.push(extra + 0x10000);
            } else {
                panic!("Invalid UCS-2 sequence");
            }
        } else {
            v.push(c as u32);
        }
    }
    v
}

pub fn ucs2_encode<T: AsRef<str>>(s: T) -> Vec<u16> {
    s.as_ref()
        .chars()
        .flat_map(|c| {
            if c as u32 > 0xFFFF {
                let extra = c as u32 - 0x10000;
                let first = (extra >> 10) & 0x3FF | 0xD800;
                let second = (extra & 0x3FF) | 0xDC00;
                vec![first as u16, second as u16]
            } else {
                vec![c as u16]
            }
        })
        .collect::<Vec<u16>>()
}
