extern mod extra;

use extra::digest::*;

pub mod consts;
pub mod display;
pub mod reference;
pub mod nist;
pub mod sponge;


fn main() {
    use extra::digest::*;
    use nist::*;
    use std::io;
    use std::path::PosixPath;
    use std::vec;

    let sizes = [224u, 256, 384, 512];

//     let in_str = match io::file_reader(&PosixPath("lol")) {
//         Ok(r) => r.read_whole_stream(),
//         Err(msg) => fail!(msg)
//     };

    let in_str = [0x0u8, ..400];
    let mut len = 0;
    "00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 00 "
    .word_iter().advance(|_| {len += 1; true});

    printfln!("len = %u", len);

    for sizes.iter().take_(1).advance |&n| {
        let mut hasher = Keccak::new(n);

        debug!(fmt!("Input bytes = %?", in_str));
        printfln!("Testing size %u:", n)
        hasher.input(in_str);
        let mut res = vec::from_elem(n / 8, 0u8);
        hasher.result(res);
        for res.iter().advance |&b| {
            printf!("%x", b as uint);
        }
        println("");

        return;
    }
}
