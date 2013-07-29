extern mod extra;

use extra::digest::*;

pub mod consts;
pub mod display;
pub mod reference;
pub mod nist;
pub mod sponge;



pub struct Keccak2 {
    priv state: [u8, .. consts::PERM_SIZE_IN_BYTES],
    priv rate: uint,
    priv cap: uint
}

impl Keccak2 {
    pub fn new(out_len: uint) -> Keccak2 {
        assert!([224,256,384,512].contains(&out_len));

        let c = 2 * out_len;
        let r = 25 * 64 - c;

        Keccak2 {
            state: [0u8, ..consts::PERM_SIZE_IN_BYTES],
            rate: r,
            cap: c
        }
    }
}

impl Digest for Keccak2 {
    pub fn input(&mut self, input: &[u8]) {
        use std::vec;

        let len = input.len();
        let mut full_buf = vec::from_elem(self.rate, 0u8);
        let end_pos = full_buf.copy_from(input);

        if len < self.rate {
            debug!(fmt!("Padding %u bytes to %u bytes", len, self.rate));

            // find last byte
            let mut las_byte;
            for full_buf.slice_to(end_pos).mut_rev_iter().enumerate().advance |(idx, byte)| {
                if *byte != 0 {
                    let mut b = *byte;
                    // find last bit
                    let mut mask = 1u8;
                    while mask & b != 0 {
                        b = b << 1;
                    }
                }
            }

        }

        reference::absorb(self.state, input, input.len() / 8);
    }

    pub fn result(&mut self, out: &mut [u8]) {
        let l = out.len() / 8;
        reference::extract(self.state, out, l);
    }

    pub fn reset(&mut self) {
        for self.state.mut_iter().advance |b| {
            *b = 0;
        }
    }

    pub fn output_bits(&self) -> uint {
        self.cap / 2
    }
}

fn main() {
    use extra::digest::*;
    use nist::*;

    let sizes = [224u, 256, 384, 512];

    let in_str = "The quick brown fox jumps over the lazy dog";

    for sizes.iter().advance |&n| {
        let mut hasher = Keccak2::new(n);

        debug!(fmt!("Input bytes = %?", in_str.as_bytes()));
        hasher.input_str(in_str);
        let res = hasher.result_str();

        println(res);
    }
}
