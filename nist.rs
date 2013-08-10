use sponge::*;
use extra::digest::Digest;

pub struct Keccak {
    priv sponge_state: SpongeState,
    priv hash_size: uint,
}

impl Keccak {
    pub fn new(hash_size: uint) -> Keccak {

        let mut sponge = match hash_size {
            0 => SpongeState::new(1024, 576),
            224 => SpongeState::new(1152, 448),
            256 => SpongeState::new(1088, 512),
            384 => SpongeState::new(832, 768),
            512 => SpongeState::new(576, 1024),
            _ => fail!("hash_size must be 0, 224, 256, 384, or 512")
        };

        sponge.fixed_out_len = hash_size;

        Keccak {
            hash_size: hash_size,
            sponge_state: sponge,
        }
    }
}

impl Digest for Keccak {
    pub fn input(&mut self, input: &[u8]) {
        let data_bit_len = input.len() * 8;

        let mut err;

        if data_bit_len % 8 == 0 {
            self.sponge_state.absorb(input, data_bit_len);
        } else {
            err = self.sponge_state.absorb(input, data_bit_len - (data_bit_len % 8));
            if err == Success {
                let last_byte = input[data_bit_len/8] >> (8 - (data_bit_len % 8));
                debug!("Got to second absorb");
                self.sponge_state.absorb(&[last_byte], data_bit_len % 8);
            } else {
                fail!(err.to_str());
            }
        }
    }

    pub fn result(&mut self, out: &mut [u8]) {
        self.sponge_state.squeeze(out, self.sponge_state.fixed_out_len);
    }

    pub fn reset(&mut self) {
        for x in self.sponge_state.state.mut_iter() {
            *x = 0u8;
        }

        for x in self.sponge_state.data_queue.mut_iter() {
            *x = 0u8;
        }

        self.sponge_state.bits_in_queue = 0;
        self.sponge_state.squeezing = false;
        self.sponge_state.bits_for_squeezing = 0;
    }

    pub fn output_bits(&self) -> uint {
        self.hash_size
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_hashing() {
        use extra::digest::*;
        use std::io;
        use std::path::PosixPath;
        use std::u8;
        use std::uint;
        use std::vec;
        use std::str;
        use std::cast::transmute;

        let sizes = [0u, 224, 256, 384, 512];

        for &size in sizes.iter() {

            do spawn {

                let mut len: uint = 0;
                let mut msg: ~[u8] = ~[];
                let mut md_ref: ~[u8] = ~[];

                let fname = fmt!("test_vectors/ShortMsgKAT_%u.txt", size);
                let r = match io::file_reader(&PosixPath(fname)) {
                    Ok(reader) => reader,
                    Err(msg) => fail!(msg)
                };

                do r.each_line |line| {
                    if line.starts_with("Len") {
                        let s = line.split_iter(' ').collect::<~[&str]>()[2];
                        len = uint::from_str(s).unwrap();
                    } else if line.starts_with("Msg") {
                        let tmp = line.split_iter(' ').collect::<~[&str]>()[2]
                            .iter()
                            .collect::<~[char]>();

                        msg =  tmp
                            .chunk_iter(2)
                            .transform(|cs| {
                                let s = str::from_chars(cs);
                                u8::from_str_radix(s, 16).unwrap()
                            })
                            .collect();
                    } else if line.starts_with("MD") && len % 8 == 0 && len != 0 {

                        let tmp = line.split_iter(' ')
                            .transform(|s| s.to_owned())
                            .collect::<~[~str]>()[2];


                        let tmp2 = tmp.iter()
                            .collect::<~[char]>();

                        md_ref = tmp2
                            .chunk_iter(2)
                            .transform(|cs| {
                                let s = str::from_chars(cs);
                                u8::from_str_radix(s, 16).unwrap()
                            })
                            .collect();

                        let mut kc = Keccak::new(size);
                        let mut res = vec::from_elem(size / 8, 0u8);

                        debug!("Len = %u", md_ref.len());
                        debug!("Msg = %?",
                            unsafe { transmute::<&[u8],&[u64]>(msg.as_slice()) });
                        debug!("Reference hash =  %?",
                            unsafe { transmute::<&[u8],&[u64]>(md_ref.as_slice()) });


                        kc.input(msg);
                        kc.result(res);

                        debug!("Result hash =  %?",
                            unsafe { transmute::<&[u8],&[u64]>(res.as_slice()) });

                        assert!(md_ref == res, fmt!("Error: Reference %? does not match result %?", md_ref, res));
                    }
                    true
                };
            }
        }
    }
}
