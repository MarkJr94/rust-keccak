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

    let sizes = [224u, 256, 384, 512];

    let in_str = "The quick brown fox jumps over the lazy dog";

    for sizes.iter().advance |&n| {
        let mut hasher = Keccak::new(n);

        debug!(fmt!("Input bytes = %?", in_str.as_bytes()));
        printfln!("Testing size %u:", n)
        hasher.input_str(in_str);
        let res = hasher.result_str();

        println(res);
    }
}
