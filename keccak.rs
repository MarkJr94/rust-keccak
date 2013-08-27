extern mod extra;

// pub use self::nist::Keccak;
use extra::digest::*;

pub mod consts;
pub mod reference;
pub mod nist;
pub mod sponge;

fn main() {
    use nist::*;
    use std::io;
    use std::vec;
    use std::uint;

    let sizes = [224u, 256, 384, 512];

    let inp = io::stdin();
    let mut size;

    loop {
        print("Choose hash size from [224, 256, 384, 512]: ");
        let in_str = inp.read_line();

        if in_str.len() == 0 {
            break;
        }

        match uint::from_str(in_str) {
            Some(n) => { size = n; }
            None => { loop; }
        }

        if !sizes.contains(&size) {
            loop;
        }

        let mut kc = Keccak::new(size);
        let mut res = vec::from_elem(size / 8, 0u8);

        print("Give input string to be hashed: ");

        let in_str = inp.read_line();

        let data = in_str.as_bytes();

        debug!("User input as bytes: %?", data);

        kc.input(data);
        kc.result(res);

        print("Hash: 0x");
        for &b in res.iter() {
            printf!("%02x", b as uint);
        }
        println("");
    }
}
