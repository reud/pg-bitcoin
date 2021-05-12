use std::num::ParseIntError;
use std::io::{Cursor, Read};
use num_bigint::BigUint;

#[allow(dead_code)]
pub fn u8vec_to_str(v: Vec<u8>) -> String {
    let mut ret = "".to_string();
    for x in v {
        ret += &*format!("{:02x}", x);
    }
    return ret;
}

#[allow(dead_code)]
pub fn hash256(v: Vec<u8>) -> Vec<u8> {
    let sha256r1 = crypto_hash::digest(crypto_hash::Algorithm::SHA256, &*v);
    let sha256r2 = crypto_hash::digest(crypto_hash::Algorithm::SHA256, &*sha256r1);
    return sha256r2;
}

pub fn vector_as_u8_4_array(vector: Vec<u8>) -> [u8;4] {
    let mut arr = [0u8;4];
    for (place, element) in arr.iter_mut().zip(vector.iter()) {
        *place = *element;
    }
    arr
}
#[allow(dead_code)]
pub fn decode_hex(s: &str) -> Result<Vec<u8>, ParseIntError> {
    (0..s.len())
        .step_by(2)
        .map(|i| u8::from_str_radix(&s[i..i + 2], 16))
        .collect()
}

#[allow(dead_code)]
pub fn read_varint(c: &mut Cursor<Vec<u8>>) -> u64 {
    let mut i = [0u8];
    c.read(&mut i);
    let i = i[0];
    if i == 0xfd {
        let mut bytes = [0u8;2];
        c.read(&mut bytes);
        return u16::from_le_bytes(bytes) as u64;
    }
    if i == 0xfe {
        let mut bytes = [0u8;4];
        c.read(&mut bytes);
        return u32::from_le_bytes(bytes) as u64;
    }
    if i == 0xff {
        let mut bytes = [0u8;8];
        c.read(&mut bytes);
        return u64::from_le_bytes(bytes);
    }
    return i as u64;
}

#[allow(dead_code)]
pub fn encode_varint(i: u64) -> Vec<u8> {
    if i < 0xfd {
        return vec![i as u8];
    }
    if i < 0x10000 {
        let mut v = vec![0xfdu8];
        for x in (i as u16).to_le_bytes() {
            v.push(x);
        }
        assert_eq!(v.len(),3);
        return v;
    }
    if i < 0x100000000 {
        let mut v = vec![0xfeu8];
        for x in (i as u32).to_le_bytes() {
            v.push(x);
        }
        assert_eq!(v.len(),5);
        return v;
    }
    if i < 0x10000000000000000 {
        let mut v = vec![0xffu8];
        for x in i.to_le_bytes() {
            v.push(x);
        }
        assert_eq!(v.len(),9);
        return v;
    }
    panic!("integer too large: {}", i);
}

#[allow(dead_code)]
pub fn biguint_to_32_bytes_be(num: BigUint) -> [u8;32] {
    let mut ret = [0u8;32];
    let mut bin = num.to_bytes_be();
    if bin.len() > 32 {
        return ret;
    }
    let buf = bin.len(); // 20
    let x = 32 - buf; // 12
    let i = 0;
    while i < x {
        ret[i+buf] = bin[i];
    }
    return ret;
}

#[allow(dead_code)]
pub fn biguint_to_32_bytes_le(num: BigUint) -> [u8;32] {
    let mut ret = [0u8;32];
    let mut bin = num.to_bytes_le();
    if bin.len() > 32 {
        return ret;
    }
    while i < bin.len() {
        ret[i] = bin[i];
    }
    return ret;
}