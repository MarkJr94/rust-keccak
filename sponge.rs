use consts::*;
use reference;

#[packed]
pub struct SpongeState {
    state: [u8, ..PERM_SIZE_IN_BYTES],
    data_queue: [u8, ..MAX_RATE_IN_BYTES],
    rate: uint,
    capacity: uint,
    bits_in_queue: uint,
    fixed_out_len: uint,
    squeezing: bool,
    bits_for_squeezing: uint,
}

#[deriving(Eq,TotalEq,ToStr,Clone)]
pub enum SpongeError {
    Success,
    Failure,
}

impl SpongeState {
    pub fn new(rate: uint, capacity: uint) -> SpongeState {
        assert!(rate + capacity == 1600);
        assert!(rate % 64 == 0);

        debug!("Rate = %u", rate);

        SpongeState {
            state: [0u8, ..PERM_SIZE_IN_BYTES],
            data_queue: [0u8, ..MAX_RATE_IN_BYTES],
            rate: rate,
            capacity: capacity,
            fixed_out_len: 0,
            bits_in_queue: 0,
            bits_for_squeezing: 0,
            squeezing: false
        }
    }

    pub fn absorb(&mut self, data: &[u8], data_bit_len: uint) -> SpongeError {
        use std::vec::raw::*;
        use std::vec::*;

        if self.bits_in_queue % 8 != 0 {
            return Failure;
        }
        if self.squeezing {
            return Failure;
        }

        let mut whole_blocks;
        let mut cur_data;
        let mut part_block;
        let mut part_byte;
        let mut i = 0u;

        while i < data_bit_len {
            debug!("data_bit_len = %u : self.rate = %u", data_bit_len, self.rate);
            if (self.bits_in_queue == 0) && (data_bit_len >= self.rate)
                && (i <=  (data_bit_len - self.rate)) {
                whole_blocks = (data_bit_len - i) / self.rate;
                cur_data = to_ptr(data) + i/8;

                unsafe {
                    match self.rate {
                        576 => {
                            for _ in range(0, whole_blocks) {
                                do buf_as_slice(cur_data, 576/8) |buf| {
                                    debug!("Block to be absorbed: %?", buf);
                                    reference::absorb_576_bits(self.state, buf);
                                }
                                cur_data = cur_data + 576/8;
                            }
                        }
                        832 => {
                            for _ in range(0, whole_blocks) {
                                do buf_as_slice(cur_data, 832/8) |buf| {
                                    debug!("Block to be absorbed: %?", buf);
                                    reference::absorb_832_bits(self.state, buf);
                                }
                                cur_data = cur_data + 832/8;
                            }
                        }
                        1024 => {
                            for _ in range(0, whole_blocks) {
                                do buf_as_slice(cur_data, 1024/8) |buf| {
                                    debug!("Block to be absorbed: %?", buf);
                                    reference::absorb_1024_bits(self.state, buf);
                                }
                                cur_data = cur_data + 1024/8;
                            }
                        }
                        1088 => {
                            for _ in range(0, whole_blocks) {
                                do buf_as_slice(cur_data, 1088/8) |buf| {
                                    debug!("Block to be absorbed: %?", buf);
                                    reference::absorb_1088_bits(self.state, buf);
                                }
                                cur_data = cur_data + 1088/8;
                            }
                        }
                        1152 => {
                            for _ in range(0, whole_blocks) {
                                do buf_as_slice(cur_data, 1152/8) |buf| {
                                    debug!("Block to be absorbed: %?", buf);
                                    reference::absorb_1152_bits(self.state, buf);
                                }
                                cur_data = cur_data + 1152/8;
                            }
                        }
                        1344 => {
                            for _ in range(0, whole_blocks) {
                                do buf_as_slice(cur_data, 1344/8) |buf| {
                                    debug!("Block to be absorbed: %?", buf);
                                    reference::absorb_1344_bits(self.state, buf);
                                }
                                cur_data = cur_data + 1344/8;
                            }
                        }
                        n => {
                            for _ in range(0, whole_blocks) {
                                do buf_as_slice(cur_data, n/8) |buf| {
                                    debug!("Block to be absorbed: %?", buf);
                                    reference::absorb(self.state, buf, self.rate / 64);
                                }
                                cur_data = cur_data + self.rate/8;
                            }
                        }
                    }
                }
                i += whole_blocks * self.rate;
            } else {
                part_block = (data_bit_len - i);
                if part_block + self.bits_in_queue > self.rate {
                    part_block = self.rate - self.bits_in_queue;
                }
                part_byte = part_block % 8;
                part_block -= part_byte;
                unsafe {
                    let dq: &mut[u8] = self.data_queue;
                    copy_memory(dq
                        .mut_slice_from(self.bits_in_queue/8),
                        data.slice_from(i/8),
                        part_block/8);
                }
                self.bits_in_queue += part_block;
                i += part_block;

                if self.bits_in_queue == self.rate {
                    self.absorb_queue();
                }
                if part_byte > 0 {
                    let mask: u8 = (1 << part_byte) - 1;
                    self.data_queue[self.bits_in_queue/8] = data[i/8] & mask;
                    self.bits_in_queue += part_byte;
                    i += part_byte;
                }
            }

            debug!("(At the end of loop iteration) i = %u",i);
        }

        Success
    }

    fn pad_and_switch_to_squeeze(&mut self) {
        use std::ptr::set_memory;

        debug!("Bits in queue: %u %?",self.bits_in_queue, self.data_queue);
        if self.bits_in_queue + 1 == self.rate {
            self.data_queue[self.bits_in_queue/8] |= 1 << (self.bits_in_queue % 8);
            self.absorb_queue();

            do self.data_queue.as_mut_buf |buf, _| {
                unsafe {
                    set_memory(buf, 0u8, self.rate / 8);
                }
            }
        } else {
            debug!("(self.bits_in_queue + 7)/8: %u | self.rate/8 - (self.bits_in_queue + 7)/8: %u ",
                (self.bits_in_queue + 7)/8, self.rate/8 - (self.bits_in_queue + 7)/8 );

            do self.data_queue.as_mut_buf |buf, _| {
                unsafe {
                    set_memory(buf + (self.bits_in_queue + 7)/8,
                        0u8,
                        self.rate/8 - (self.bits_in_queue + 7)/8);
                }
            }
            self.data_queue[self.bits_in_queue/8] |= 1 << (self.bits_in_queue % 8);
        }
        self.data_queue[(self.rate-1)/8] |= 1 << ((self.rate-1) % 8);
        self.absorb_queue();

        debug!("--- Switching to squeezing phase ---");

        if self.rate == 1024 {
            debug!("Fast 1024");
            reference::extract_1024_bits(self.state, self.data_queue);
            self.bits_for_squeezing = 1024;
        } else {
            debug!("Other rate");
            reference::extract(self.state, self.data_queue, self.rate/64);
            self.bits_for_squeezing = self.rate;
        }

        debug!("Block available for squeezing: %?", self.data_queue.slice_to(self.bits_for_squeezing / 8));
        self.squeezing = true;
    }

    pub fn squeeze(&mut self, out: &mut[u8], out_len: uint) -> SpongeError {
        use std::vec::raw::copy_memory;

        if !self.squeezing {
            self.pad_and_switch_to_squeeze();
        }

        if out_len % 8 != 0 {
            return Failure;
        }

        let mut i = 0;
        let mut part_block;

        while i < out_len {
            if self.bits_for_squeezing == 0 {
                reference::permute(self.state);

                if self.rate == 1024 {
                    reference::extract_1024_bits(self.state, self.data_queue);
                    self.bits_for_squeezing = 1024;
                } else {
                    reference::extract(self.state, self.data_queue, self.rate / 64);
                    self.bits_for_squeezing = self.rate;
                }

                debug!("Blocks available for squeezing: %?",
                    self.data_queue.slice_to(self.bits_for_squeezing / 8));
            }

            part_block = self.bits_for_squeezing;
            if part_block > out_len - i {
                part_block = out_len - i;
            }

            unsafe {
                copy_memory(out.mut_slice_from(i/8),
                    self.data_queue.slice_from((self.rate - self.bits_for_squeezing) / 8),
                    part_block/8);
            }
            self.bits_for_squeezing -= part_block;
            i += part_block;
        }

        Success
    }

    fn absorb_queue(&mut self) {
        debug!("Absorbing Queue");
        debug!("Block to be absorbed: %?", self.data_queue.slice_to(self.rate/8));
        match self.rate {
            576 => reference::absorb_576_bits(self.state, self.data_queue),
            832 => reference::absorb_832_bits(self.state, self.data_queue),
            1024 => reference::absorb_1024_bits(self.state, self.data_queue),
            1088 => reference::absorb_1088_bits(self.state, self.data_queue),
            1152 => reference::absorb_1152_bits(self.state, self.data_queue),
            1344 => reference::absorb_1344_bits(self.state, self.data_queue),
            _ => reference::absorb(self.state, self.data_queue, self.rate / 64)
        }

        self.bits_in_queue = 0;
    }
}

#[cfg(test)]
mod test {
    use super::*;
    static TEST_OUT: [u64, ..25] = [
        0xF1258F7940E1DDE7, 0x84D5CCF933C0478A,
        0xD598261EA65AA9EE, 0xBD1547306F80494D,
        0x8B284E056253D057, 0xFF97A42D7F8E6FD4,
        0x90FEE5A0A44647C4, 0x8C5BDA0CD6192E76,
        0xAD30A6F71B19059C, 0x30935AB7D08FFC64,
        0xEB5AA93F2317D635, 0xA9A6E6260D712103,
        0x81A57C16DBCF555F, 0x43B831CD0347C826,
        0x01F22F1A11A5569F, 0x05E5635A21D9AE61,
        0x64BEFEF28CC970F2, 0x613670957BC46611,
        0xB87C5A554FD00ECB, 0x8C3EE88A1CCF32C8,
        0x940C7922AE3A2614, 0x1841F924A2C509E4,
        0x16F53526E70465C2, 0x75F644E97F30A13B,
        0xEAF1FF7B5CECA249
    ];

    #[test]
    fn test_sponge() {
        use std::cast::transmute;
        let mut sp = SpongeState::new(1152, 448);

        let test_in = [0u8, ..144];

        sp.absorb(test_in, 1152);

        unsafe {
            assert_eq!(transmute::<&mut [u8], &[u64]>(sp.state), TEST_OUT);
        }
    }
}
