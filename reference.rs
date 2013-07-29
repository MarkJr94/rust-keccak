use consts::*;
use std::uint::range;

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

priv fn theta( A: &mut [u64]) {
    let c = &mut [0u64, ..5];
    let d = &mut [0u64, ..5];

    for range(0,5) |x| {
        for range(0, 5) |y| {
            c[x] ^= A[index!(x, y)];
        }
    }

    for range(0, 5) |x| {
        d[x] = ROL64!( c[ (x + 1) % 5], 1 ) ^ c[(x + 4) % 5];
    }

    for range(0,5) |x| {
        for range(0, 5) |y| {
            A[index!(x, y)] ^= d[x];
        }
    }
}

priv fn rho(A: &mut [u64]) {
    for range(0, 5) |x| {
        for range(0, 5) |y| {
            A[index!(x, y)] = ROL64!(A[index!(x, y)], RHO_OFFSETS[index!(x, y)]);
        }
    }
}

priv fn pi(A: &mut [u64]) {
    let tempA = &mut [0u64, ..25];

    for range(0, 5) |x| {
        for range(0, 5) |y| {
            tempA[index!(x, y)] = A[index!(x, y)];
        }
    }

    for range(0, 5) |x| {
        for range(0, 5) |y| {
            A[index!(y, 2 * x + 3 * y)] = tempA[index!(x, y)];
        }
    }
}

priv fn chi(A: &mut [u64]) {
    let c = &mut [0u64, ..5];

    for range(0, 5) |y| {
        for range(0, 5) |x| {
            c[x] = A[index!(x, y)] ^ ((!A[index!(x + 1, y)]) & A[index!(x + 2, y)]);
        }

        for range(0, 5) |x| {
            A[index!(x, y)] = c[x];
        }
    }
}

priv fn iota(A: &mut [u64], index_round: uint) {
    A[index!(0, 0)] ^= ROUND_CONST[index_round] as u64;
}

priv fn permute_on_words(state: &mut[u64]) {
    for range(0, ROUND_N) |i| {
        theta(state);
        rho(state);
        pi(state);
        chi(state);
        iota(state,i)
    }
}

pub fn permute_after_xor(state: &mut[u8], data: &[u8], data_len_bytes: uint) {
    use std::iterator::*;

    for range(0, data_len_bytes) |i| {
        state[i] ^= data[i];
    }

    permute(state)
}

pub fn permute(state: &mut[u8]) {
    use std::cast::transmute;
    debug!(fmt!("Permutation input: %?",state));

    unsafe { permute_on_words(transmute(state)); }
}

pub fn new_state() -> ~[u8] {
    ~[0u8, ..PERM_SIZE_IN_BYTES]
}

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
