use consts::*;

priv static NR_LANES: uint = 25;

macro_rules! index(
    ($x:expr, $y:expr) => (
        ((($x) % 5) + 5 * (($y) % 5))
    )
)

macro_rules! ROL64(
    ($a:expr, $offset:expr) => (
        (if $offset != 0 {  (($a as u64) << $offset) ^ (($a as u64) >> (64 - $offset) ) } else { $a })
    )
)

pub fn absorb_576_bits(state: &mut[u8], data: &[u8]) {
    permute_after_xor(state, data, 72);
}

pub fn absorb_832_bits(state: &mut[u8], data: &[u8]) {
    permute_after_xor(state, data, 104);
}

pub fn absorb_1024_bits(state: &mut[u8], data: &[u8]) {
    permute_after_xor(state, data, 128);
}

pub fn absorb_1088_bits(state: &mut[u8], data: &[u8]) {
    permute_after_xor(state, data, 136);
}

pub fn absorb_1152_bits(state: &mut[u8], data: &[u8]) {
    permute_after_xor(state, data, 144);
}

pub fn absorb_1344_bits(state: &mut[u8], data: &[u8]) {
    permute_after_xor(state, data, 168);
}

pub fn absorb(state: &mut[u8], data: &[u8], lane_count: uint) {
    permute_after_xor(state, data, lane_count * 8);
}

pub fn extract_1024_bits(state: &[u8], data: &mut[u8]) {
    use std::vec::raw::copy_memory;

    unsafe { copy_memory(data, state, 128); }
}

pub fn extract(state: &[u8], data: &mut[u8], lane_count: uint) {
    use std::vec::raw::copy_memory;

    unsafe { copy_memory(data, state, lane_count * 8); }
}

pub fn permute(state: &mut[u8]) {
    use std::cast::transmute;

    unsafe {
        let fixed = transmute::<&mut [u8], &mut [u64]> (state);
        debug!("fixed.len() = %u \t state.len() = %u", fixed.len(), state.len());

        dump(fixed,"Input of permutation");
        permute_on_words(fixed);
        dump(fixed,"State after permutation");
    }
}

priv fn theta( A: &mut [u64]) {
    let c = &mut [0u64, ..5];
    let d = &mut [0u64, ..5];

    for x in range(0, 5) {
        for y in range(0, 5) {
            c[x] ^= A[index!(x, y)];
        }
    }

    for x in range(0, 5) {
        d[x] = ROL64!( c[ (x + 1) % 5], 1 ) ^ c[(x + 4) % 5];
    }

    for x in range(0,5) {
        for y in range(0, 5) {
            A[index!(x, y)] ^= d[x];
        }
    }
}

priv fn rho(A: &mut [u64]) {
    for x in range(0, 5) {
        for y in range(0, 5) {
            A[index!(x, y)] = ROL64!(A[index!(x, y)], RHO_OFFSETS[index!(x, y)]);
        }
    }
}

priv fn pi(A: &mut [u64]) {
    let tempA = &mut [0u64, ..25];

    for x in range(0, 5) {
        for y in range(0, 5) {
            tempA[index!(x, y)] = A[index!(x, y)];
        }
    }

    for x in range(0, 5) {
        for y in range(0, 5) {
            A[index!(0 * x + 1 * y, 2 * x + 3 * y)] = tempA[index!(x, y)];
        }
    }
}

priv fn chi(A: &mut [u64]) {
    let c = &mut [0u64, ..5];

    for y in range(0, 5) {
        for x in range(0, 5) {
            c[x] = A[index!(x, y)] ^ ((!A[index!(x + 1, y)]) & A[index!(x + 2, y)]);
        }

        for x in range(0, 5) {
            A[index!(x, y)] = c[x];
        }
    }
}

priv fn iota(A: &mut [u64], index_round: uint) {
    A[index!(0, 0)] ^= ROUND_CONST[index_round] as u64;
}

priv fn dump(state: &mut[u64], msg: &str) {
    debug!("%s: %?", msg, state);
}

priv fn permute_on_words(state: &mut[u64]) {

    for i in range(0, ROUND_N) {
        debug!("--- Round %u ---", i);

        theta(state);
        dump(state, "After Theta");

        rho(state);
        dump(state, "After Rho");

        pi(state);
        dump(state, "After Pi");

        chi(state);
        dump(state, "After Chi");

        iota(state,i);
        dump(state, "After Iota");

    }
}

priv fn permute_after_xor(state: &mut[u8], data: &[u8], data_len_bytes: uint) {
    for i in range(0, data_len_bytes) {
        state[i] ^= data[i];
    }

    permute(state)
}

#[test]
fn test_permutation() {
    use std::io;
    use std::path::PosixPath;
    use std::vec;

    let mut state = vec::from_elem(25, 0u64);

    let r = match io::file_reader(
            &PosixPath("test_vectors/KeccakPermutationIntermediateValues.txt")) {
        Ok(reader) => { reader }
        Err(msg) => { fail!(msg) }
    };

    let mut round_n = -1;

    do r.each_line |line| {
        let mut ret = true;

        if line.starts_with("E7 DD E1 40") {
            ret = false;
        }

        if line.starts_with("--- Round") {
            round_n += 1;

            info!(line);
            r.read_line();

            let rtheta = r.read_line();
            assert_eq!(~"After theta:", rtheta);
            let ref_state = get_state(r);
            theta(state);
            assert!(state == ref_state, "State is incorrect after theta");

            let rrho = r.read_line();
            assert_eq!(~"After rho:", rrho);
            let ref_state = get_state(r);
            rho(state);
            assert!(state == ref_state, "State is incorrect after rho");

            let rpi = r.read_line();
            assert_eq!(~"After pi:", rpi);
            let ref_state = get_state(r);
            pi(state);
            assert!(state == ref_state, "State is incorrect after pi");

            let rchi = r.read_line();
            assert_eq!(~"After chi:", rchi);
            let ref_state = get_state(r);
            chi(state);
            assert!(state == ref_state, "State is incorrect after chi");

            let riota = r.read_line();
            assert_eq!(~"After iota:", riota);
            let ref_state = get_state(r);
            iota(state, round_n as uint);
            assert!(state == ref_state, "State is incorrect after iota");

            ret = true;
        }

        ret
    };
}

#[cfg(test)]
priv fn get_state(reader: @Reader) -> ~[u64] {
    use std::u64;

    let mut state = ~[];

    for _ in range(0, 5) {
        let line = reader.read_line();

        for s in line.split_iter(' ') {
            match u64::from_str_radix(s, 16) {
                Some(x) => { state.push(x); }
                None => { fail!("Error in parsing number"); }
            }
        }
    }

    state
}
