use std::io::{Writer};
use consts::*;

pub struct ValDisplayer {
    priv writer: @Writer,
    priv display_level: i32
}


impl ValDisplayer {
    pub fn new(writer: @Writer, level: i32) -> ValDisplayer {
        ValDisplayer { display_level: level, writer: writer}
    }

    pub fn set_file(&mut self, writer: @Writer) {
        self.writer = writer;
    }

    pub fn set_level(&mut self, level: i32) {
        self.display_level = level;
    }

    pub fn display_bytes(&self, level: i32, text: &str, bytes: &[u8]) {
        if level <= self.display_level {
            self.writer.write_line(text);

            for bytes.iter().advance |&byte| {
                self.writer.write_str(fmt!("%02X ", byte as uint));
            }

            self.writer.write_line("");
            self.writer.write_line("");
        }
    }

    pub fn display_bits(&self, level: i32, text: &str, data: &[u8], msb_fist: bool) {
        if level <= self.display_level {
            let mut i_byte = 0;
            let mut i_bit = 0;

            self.writer.write_line(text);

            for data.iter().enumerate().advance |(i, _)| {
                i_byte = i / 8;
                i_bit = i % 8;

                if msb_fist {
                    self.writer.write_str(fmt!("%d ", (((data[i_byte] << i_bit) & 0x80) != 0) as int));
                } else {
                    self.writer.write_str(fmt!("%d ", (((data[i_byte] >> i_bit) & 0x01) != 0) as int));
                }
            }

            self.writer.write_line("");
            self.writer.write_line("");
        }
    }

    pub fn display_state_as_bytes(&self, level: i32, text: &str, state: &[u8]) {
        self.display_bytes(level, text, state);
    }

    pub fn display_state_as_32bit(&self, level: i32, text: &str, state: &[u32]) {
        use std::iterator::*;

        if level <= self.display_level {
            self.writer.write_line(text);

            let mut c = Counter::new(0,1).take_(PERM_SIZE/64);
            for c.advance |i| {
                self.writer.write_str(fmt!("%08X:%08X", state[2 *  i] as uint, state[2 * i + 1] as uint));

                if i % 5 == 4 {
                    self.writer.write_str("\n");
                } else {
                    self.writer.write_str(" ");
                }
            }
        }
    }

    pub fn display_state_as_64bit(&self, level: i32, text: &str, state: &[u64]) {
        use std::iterator::*;

        if level <= self.display_level {
            self.writer.write_line(text);
            let mut c = Counter::new(0,1).take_(PERM_SIZE/64);
            for c.advance |i| {
                self.writer.write_str(fmt!("%16X", state[i] as uint));

                if i % 5 == 4 {
                    self.writer.write_str("\n");
                } else {
                    self.writer.write_str(" ");
                }
            }
        }
    }

    pub fn display_round_number(&self, level: i32, rn: u32) {
        if level <= self.display_level {
            self.writer.write_line("");
            self.writer.write_line(fmt!("--- Round %d ---", rn as int));
            self.writer.write_line("");
        }
    }

    pub fn display_test(&self, level: i32, text: &str) {
        if level <= self.display_level {
            self.writer.write_line(text);
            self.writer.write_line("");
        }
    }
}
