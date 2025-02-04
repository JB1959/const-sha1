//! A const evaluated sha1 function.
//!
//! # Use
//!
//! ```
//! const fn signature() -> [u32; 5] {
//!     const_sha1::sha1(stringify!(MyType).as_bytes()).data
//! }
//! ```

/// A const evaluated sha1 function.
///
/// # Use
///
/// ```
/// const fn signature() -> [u32; 5] {
///     const_sha1::sha1(stringify!(MyType).as_bytes()).data
/// }
/// ```
pub const fn sha1(data: &[u8]) -> Digest {
    let state: [u32; 5] = [0x67452301, 0xefcdab89, 0x98badcfe, 0x10325476, 0xc3d2e1f0];
    let len: u64 = 0;
    let blocks = Blocks {
        len: 0,
        data: [0; 64],
    };
    let (blocks, len, state) = process_blocks(blocks, data, len, state);
    digest(state, len, blocks)
}

struct Blocks {
    len: u32,
    data: [u8; 64],
}

const fn process_blocks(
    mut blocks: Blocks,
    data: &[u8],
    mut len: u64,
    mut state: [u32; 5],
) -> (Blocks, u64, [u32; 5]) {
    const fn as_block(input: &[u8], offset: usize) -> [u32; 16] {
        let mut result = [0u32; 16];

        let mut i = 0;
        while i != 16 {
            let off = offset + (i * 4);
            result[i] = (input[off + 3] as u32)
                | ((input[off + 2] as u32) << 8)
                | ((input[off + 1] as u32) << 16)
                | ((input[off] as u32) << 24);
            i += 1;
        }
        result
    }

    const fn clone_from_slice_64(
        mut data: [u8; 64],
        slice: &[u8],
        offset: usize,
        num_elems: usize,
    ) -> [u8; 64] {
        let mut i = 0;
        loop {
            if i >= num_elems {
                break;
            }
            data[i] = slice[offset + i];
            i += 1;
        }
        data
    }

    let mut i = 0;
    loop {
        if i >= data.len() {
            break;
        }
        if data.len() - i >= 64 {
            let chunk_block = as_block(data, i);
            len = 16 * 4;
            state = process_state(state, chunk_block);
            i += 64;
        } else {
            let num_elems = data.len() - i;
            blocks.data = clone_from_slice_64(blocks.data, data, i, num_elems);
            blocks.len = num_elems as u32;
            break;
        }
    }
    (blocks, len, state)
}

const fn process_state(mut state: [u32; 5], block: [u32; 16]) -> [u32; 5] {
    let a = state[0];
    let b = state[1];
    let c = state[2];
    let d = state[3];
    let e = state[4];
    let (block, b, e) = r0(block, a, b, c, d, e, 0);
    let (block, a, d) = r0(block, e, a, b, c, d, 1);
    let (block, e, c) = r0(block, d, e, a, b, c, 2);
    let (block, d, b) = r0(block, c, d, e, a, b, 3);
    let (block, c, a) = r0(block, b, c, d, e, a, 4);
    let (block, b, e) = r0(block, a, b, c, d, e, 5);
    let (block, a, d) = r0(block, e, a, b, c, d, 6);
    let (block, e, c) = r0(block, d, e, a, b, c, 7);
    let (block, d, b) = r0(block, c, d, e, a, b, 8);
    let (block, c, a) = r0(block, b, c, d, e, a, 9);
    let (block, b, e) = r0(block, a, b, c, d, e, 10);
    let (block, a, d) = r0(block, e, a, b, c, d, 11);
    let (block, e, c) = r0(block, d, e, a, b, c, 12);
    let (block, d, b) = r0(block, c, d, e, a, b, 13);
    let (block, c, a) = r0(block, b, c, d, e, a, 14);
    let (block, b, e) = r0(block, a, b, c, d, e, 15);
    let (block, a, d) = r1(block, e, a, b, c, d, 0);
    let (block, e, c) = r1(block, d, e, a, b, c, 1);
    let (block, d, b) = r1(block, c, d, e, a, b, 2);
    let (block, c, a) = r1(block, b, c, d, e, a, 3);
    let (block, b, e) = r2(block, a, b, c, d, e, 4);
    let (block, a, d) = r2(block, e, a, b, c, d, 5);
    let (block, e, c) = r2(block, d, e, a, b, c, 6);
    let (block, d, b) = r2(block, c, d, e, a, b, 7);
    let (block, c, a) = r2(block, b, c, d, e, a, 8);
    let (block, b, e) = r2(block, a, b, c, d, e, 9);
    let (block, a, d) = r2(block, e, a, b, c, d, 10);
    let (block, e, c) = r2(block, d, e, a, b, c, 11);
    let (block, d, b) = r2(block, c, d, e, a, b, 12);
    let (block, c, a) = r2(block, b, c, d, e, a, 13);
    let (block, b, e) = r2(block, a, b, c, d, e, 14);
    let (block, a, d) = r2(block, e, a, b, c, d, 15);
    let (block, e, c) = r2(block, d, e, a, b, c, 0);
    let (block, d, b) = r2(block, c, d, e, a, b, 1);
    let (block, c, a) = r2(block, b, c, d, e, a, 2);
    let (block, b, e) = r2(block, a, b, c, d, e, 3);
    let (block, a, d) = r2(block, e, a, b, c, d, 4);
    let (block, e, c) = r2(block, d, e, a, b, c, 5);
    let (block, d, b) = r2(block, c, d, e, a, b, 6);
    let (block, c, a) = r2(block, b, c, d, e, a, 7);
    let (block, b, e) = r3(block, a, b, c, d, e, 8);
    let (block, a, d) = r3(block, e, a, b, c, d, 9);
    let (block, e, c) = r3(block, d, e, a, b, c, 10);
    let (block, d, b) = r3(block, c, d, e, a, b, 11);
    let (block, c, a) = r3(block, b, c, d, e, a, 12);
    let (block, b, e) = r3(block, a, b, c, d, e, 13);
    let (block, a, d) = r3(block, e, a, b, c, d, 14);
    let (block, e, c) = r3(block, d, e, a, b, c, 15);
    let (block, d, b) = r3(block, c, d, e, a, b, 0);
    let (block, c, a) = r3(block, b, c, d, e, a, 1);
    let (block, b, e) = r3(block, a, b, c, d, e, 2);
    let (block, a, d) = r3(block, e, a, b, c, d, 3);
    let (block, e, c) = r3(block, d, e, a, b, c, 4);
    let (block, d, b) = r3(block, c, d, e, a, b, 5);
    let (block, c, a) = r3(block, b, c, d, e, a, 6);
    let (block, b, e) = r3(block, a, b, c, d, e, 7);
    let (block, a, d) = r3(block, e, a, b, c, d, 8);
    let (block, e, c) = r3(block, d, e, a, b, c, 9);
    let (block, d, b) = r3(block, c, d, e, a, b, 10);
    let (block, c, a) = r3(block, b, c, d, e, a, 11);
    let (block, b, e) = r4(block, a, b, c, d, e, 12);
    let (block, a, d) = r4(block, e, a, b, c, d, 13);
    let (block, e, c) = r4(block, d, e, a, b, c, 14);
    let (block, d, b) = r4(block, c, d, e, a, b, 15);
    let (block, c, a) = r4(block, b, c, d, e, a, 0);
    let (block, b, e) = r4(block, a, b, c, d, e, 1);
    let (block, a, d) = r4(block, e, a, b, c, d, 2);
    let (block, e, c) = r4(block, d, e, a, b, c, 3);
    let (block, d, b) = r4(block, c, d, e, a, b, 4);
    let (block, c, a) = r4(block, b, c, d, e, a, 5);
    let (block, b, e) = r4(block, a, b, c, d, e, 6);
    let (block, a, d) = r4(block, e, a, b, c, d, 7);
    let (block, e, c) = r4(block, d, e, a, b, c, 8);
    let (block, d, b) = r4(block, c, d, e, a, b, 9);
    let (block, c, a) = r4(block, b, c, d, e, a, 10);
    let (block, b, e) = r4(block, a, b, c, d, e, 11);
    let (block, a, d) = r4(block, e, a, b, c, d, 12);
    let (block, e, c) = r4(block, d, e, a, b, c, 13);
    let (block, d, b) = r4(block, c, d, e, a, b, 14);
    let (_, c, a) = r4(block, b, c, d, e, a, 15);

    state[0] = state[0].wrapping_add(a);
    state[1] = state[1].wrapping_add(b);
    state[2] = state[2].wrapping_add(c);
    state[3] = state[3].wrapping_add(d);
    state[4] = state[4].wrapping_add(e);
    state
}

const fn digest(mut state: [u32; 5], len: u64, blocks: Blocks) -> Digest {
    const fn clone_from_array_64(
        mut data: [u8; 128],
        array: [u8; 64],
        offset: usize,
        num_elems: usize,
    ) -> [u8; 128] {
        let mut i = 0;
        loop {
            if i >= num_elems {
                break;
            }
            data[i] = array[offset + i];
            i += 1;
        }
        data
    }

    const fn clone_from_array_8(
        mut data: [u8; 128],
        array: [u8; 8],
        offset: usize,
        num_elems: usize,
    ) -> [u8; 128] {
        let mut i = 0;
        loop {
            if i >= num_elems {
                break;
            }
            data[offset + i] = array[i];
            i += 1;
        }
        data
    }

    const fn as_block(input: [u8; 128], offset: usize) -> [u32; 16] {
        let mut result = [0u32; 16];

        let mut i = 0;
        while i != 16 {
            let off = offset + (i * 4);
            result[i] = (input[off + 3] as u32)
                | ((input[off + 2] as u32) << 8)
                | ((input[off + 1] as u32) << 16)
                | ((input[off] as u32) << 24);
            i += 1;
        }
        result
    }

    let bits = (len + (blocks.len as u64)) * 8;
    let extra = [
        (bits >> 56) as u8,
        (bits >> 48) as u8,
        (bits >> 40) as u8,
        (bits >> 32) as u8,
        (bits >> 24) as u8,
        (bits >> 16) as u8,
        (bits >> 8) as u8,
        (bits >> 0) as u8,
    ];
    let mut last = [0; 128];
    let blocklen = blocks.len as usize;
    last = clone_from_array_64(last, blocks.data, 0, blocklen);
    last[blocklen] = 0x80;

    if blocklen < 56 {
        last = clone_from_array_8(last, extra, 56, 8);
        state = process_state(state, as_block(last, 0));
    } else {
        last = clone_from_array_8(last, extra, 120, 8);
        state = process_state(state, as_block(last, 0));
        state = process_state(state, as_block(last, 64));
    }
    Digest { data: state }
}

const fn rol(value: u32, bits: usize) -> u32 {
    (value << bits) | (value >> (32 - bits))
}

const fn blk(block: &[u32; 16], i: usize) -> u32 {
    let value = block[(i + 13) & 15] ^ block[(i + 8) & 15] ^ block[(i + 2) & 15] ^ block[i];
    rol(value, 1)
}

const fn r0(
    block: [u32; 16],
    v: u32,
    mut w: u32,
    x: u32,
    y: u32,
    mut z: u32,
    i: usize,
) -> ([u32; 16], u32, u32) {
    let n = ((w & (x ^ y)) ^ y)
        .wrapping_add(block[i])
        .wrapping_add(0x5a82_7999)
        .wrapping_add(rol(v, 5));
    z = z.wrapping_add(n);
    w = rol(w, 30);
    (block, w, z)
}

const fn r1(
    mut block: [u32; 16],
    v: u32,
    mut w: u32,
    x: u32,
    y: u32,
    mut z: u32,
    i: usize,
) -> ([u32; 16], u32, u32) {
    block[i] = blk(&block, i);
    let n = ((w & (x ^ y)) ^ y)
        .wrapping_add(block[i])
        .wrapping_add(0x5a82_7999)
        .wrapping_add(rol(v, 5));
    z = z.wrapping_add(n);
    w = rol(w, 30);
    (block, w, z)
}

const fn r2(
    mut block: [u32; 16],
    v: u32,
    mut w: u32,
    x: u32,
    y: u32,
    mut z: u32,
    i: usize,
) -> ([u32; 16], u32, u32) {
    block[i] = blk(&block, i);
    let n = (w ^ x ^ y)
        .wrapping_add(block[i])
        .wrapping_add(0x6ed_9eba1)
        .wrapping_add(rol(v, 5));
    z = z.wrapping_add(n);
    w = rol(w, 30);
    (block, w, z)
}

const fn r3(
    mut block: [u32; 16],
    v: u32,
    mut w: u32,
    x: u32,
    y: u32,
    mut z: u32,
    i: usize,
) -> ([u32; 16], u32, u32) {
    block[i] = blk(&block, i);
    let n = (((w | x) & y) | (w & x))
        .wrapping_add(block[i])
        .wrapping_add(0x8f1b_bcdc)
        .wrapping_add(rol(v, 5));
    z = z.wrapping_add(n);
    w = rol(w, 30);
    (block, w, z)
}

const fn r4(
    mut block: [u32; 16],
    v: u32,
    mut w: u32,
    x: u32,
    y: u32,
    mut z: u32,
    i: usize,
) -> ([u32; 16], u32, u32) {
    block[i] = blk(&block, i);
    let n = (w ^ x ^ y)
        .wrapping_add(block[i])
        .wrapping_add(0xca62_c1d6)
        .wrapping_add(rol(v, 5));
    z = z.wrapping_add(n);
    w = rol(w, 30);
    (block, w, z)
}

/// A sha1 digest
pub struct Digest {
    /// The sha1 digest's data
    pub data: [u32; 5],
}

impl std::fmt::Display for Digest {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for i in self.data.iter() {
            write!(f, "{:08x}", i)?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn it_works() {
        let tests = [
            (
                "The quick brown fox jumps over the lazy dog",
                "2fd4e1c67a2d28fced849ee1bb76e7391b93eb12",
            ),
            (
                "The quick brown fox jumps over the lazy cog",
                "de9f2c7fd25e1b3afad3e85a0bd17d9b100db4b3",
            ),
            ("", "da39a3ee5e6b4b0d3255bfef95601890afd80709"),
            ("testing\n", "9801739daae44ec5293d4e1f53d3f4d2d426d91c"),
            ("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
             "025ecbd5d70f8fb3c5457cd96bab13fda305dc59"),
            ("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
             "4300320394f7ee239bcdce7d3b8bcee173a0cd5c"),
            ("xxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxxx",
             "cef734ba81a024479e09eb5a75b6ddae62e6abf1"),
        ];

        for &(s, expected) in tests.iter() {
            let hash = sha1(s.as_bytes()).to_string();

            assert_eq!(hash, expected);
        }
    }
}
