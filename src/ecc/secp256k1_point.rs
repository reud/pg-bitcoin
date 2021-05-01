use crate::ecc::secp256k1_field::{Sec256k1Element, new_secp256k1element_from_i32, new_secp256k1element};
use std::fmt::{Display, Formatter};
use std::fmt;
use num_bigint::BigUint;
use num_traits::{FromPrimitive, Num, Zero, One};
use crate::ecc::secp256k1_curve::{Secp256k1Curve, new_secp256k1curve};
use std::ops::{Add, Rem, Div};

use crate::ecc::secp256k1_scalar_element::Secp256k1ScalarElement;

fn prime() -> BigUint {
    return BigUint::from(2u32).pow(256)
        - BigUint::from_u8(2u8).unwrap().pow(32u32)
        - (BigUint::from(1u8) * 977u32)
}

#[derive(Debug, Clone)]
pub struct Secp256k1Point {
    x: Sec256k1Element,
    y: Sec256k1Element,
    is_infinity: bool,
    curve: Secp256k1Curve
}

impl PartialEq for Secp256k1Point {
    fn eq(&self, other: &Self) -> bool {
        if self.is_infinity || other.is_infinity {
            if self.is_infinity && other.is_infinity {
                return true;
            }
            return false;
        }
        return self.x == other.x && self.y == other.y;
    }
}

impl Secp256k1Point {
    fn inner_mul(self, f: Secp256k1Point, v: BigUint) -> Secp256k1Point {
        println!("nowf: {} now: {}",f,v);
        if v == BigUint::zero() {
            return new_secp256k1point_infinity();
        }
        if v.clone().rem(2u64) == BigUint::zero() {
            let half_res = self.inner_mul(f, v.clone().div(2u32));
            let half_res2 = half_res.clone();
            return half_res + half_res2;
        }
        let cf = f.clone();
        f.clone() + f.clone().inner_mul(cf, v - BigUint::one())
    }
    fn mul_from_u64(self, v: u64) -> Secp256k1Point {
        let this = self.clone();
        self.inner_mul(this, BigUint::from(v))
    }
    fn mul_from_u32(self, v: u32) -> Secp256k1Point {
        let this = self.clone();
        self.inner_mul(this, BigUint::from(v))
    }
    fn mul_from_i32(self, v: i32) -> Secp256k1Point {
        let this = self.clone();
        self.inner_mul(this, BigUint::from_i32(v).unwrap())
    }
    fn mul_from_big_uint(self, v: BigUint) -> Secp256k1Point {
        let this = self.clone();
        self.inner_mul(this, v)
    }
    fn mul_from_sec256k1scalar_element(self, v: Secp256k1ScalarElement) -> Secp256k1Point {
        let this = self.clone();
        self.inner_mul(this, v.num)
    }
}

impl Display for Secp256k1Point {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        if self.is_infinity {
            return write!(f,"(無限遠点)");
        }
        write!(f, "({},{})", self.x, self.y)
    }
}

impl Add for Secp256k1Point {
    type Output = Secp256k1Point;
    fn add(self, rhs: Self) -> Self::Output {

        if self.is_infinity {
            return rhs;
        }

        if rhs.is_infinity {
            return self;
        }

        // 楕円曲線の座標が一致する時は接点の傾きを利用する。
        if self == rhs {
            let f0 =new_secp256k1element_from_i32(0);
            let f1 = new_secp256k1element_from_i32(1);
            let f2 = new_secp256k1element_from_i32(2);
            let f3 = new_secp256k1element_from_i32(3);
            if self.y == f0 && rhs.y == f0 {
                return new_secp256k1point_infinity();
            }

            let b = new_secp256k1element(self.curve.rhs(f0.num));
            let a = new_secp256k1element(self.curve.rhs(f1.clone().num)) - f1 - b;
            let x = self.x.clone();
            let y = self.y.clone();
            let s = (f3 * x.clone() * x.clone() + a) / (f2.clone() * y.clone());
            let x3 = s.clone() * s.clone() - (f2.clone() * x.clone());
            let y3 = s.clone() * (x.clone() - x3.clone()) - y.clone();
            let p = new_secp256k1point_from_element(x3.clone(),y3.clone());
            return p;
        }

        // 加法逆元の場合、無限遠点を返す
        let f0 = new_secp256k1element_from_i32(0);
        if self.x.clone() == rhs.x.clone() && (self.y.clone() + rhs.y.clone()) == f0 {
            return new_secp256k1point_infinity();
        }

        let s = (rhs.y.clone() - self.y.clone()) / (rhs.x.clone() - self.x.clone());
        let x3 = s.clone() * s.clone() - self.x.clone() - rhs.x.clone();
        let y3 = s.clone() * (self.x.clone() - x3.clone()) - self.y.clone();

        return new_secp256k1point_from_element(x3.clone(),y3);
    }
}



fn new_secp256k1point_from_i64(x: i64,y:i64) -> Option<Secp256k1Point> {
    let xe = BigUint::from_i64(x);
    if xe.is_none() {
        return None;
    }
    let x = new_secp256k1element(xe.unwrap());

    let ye = BigUint::from_i64(y);
    if ye.is_none() {
        return None;
    }
    let y = new_secp256k1element(ye.unwrap());
    return Some(Secp256k1Point{
        x,
        y,
        is_infinity: false,
        curve: new_secp256k1curve()
    })
}

fn new_secp256k1point_from_i32(x: i32,y:i32) -> Option<Secp256k1Point> {
    let xe = BigUint::from_i32(x);
    if xe.is_none() {
        return None;
    }
    let x = new_secp256k1element(xe.unwrap());

    let ye = BigUint::from_i32(y);
    if ye.is_none() {
        return None;
    }
    let y = new_secp256k1element(ye.unwrap());
    return Some(Secp256k1Point{
        x,
        y,
        is_infinity: false,
        curve: new_secp256k1curve()
    })
}

fn new_secp256k1point_from_big_uint(x: BigUint,y: BigUint) -> Secp256k1Point {
    let x = new_secp256k1element(x);
    let y = new_secp256k1element(y);
    return Secp256k1Point {
        x,
        y,
        is_infinity: false,
        curve: new_secp256k1curve()
    }
}


fn new_secp256k1point_from_element(x: Sec256k1Element,y: Sec256k1Element) -> Secp256k1Point {
    return Secp256k1Point {
        x,
        y,
        is_infinity: false,
        curve: new_secp256k1curve()
    }
}

fn new_secp256k1point_infinity() -> Secp256k1Point {
    return Secp256k1Point {
        x: new_secp256k1element(BigUint::from(1u64)),
        y: new_secp256k1element(BigUint::from(1u64)),
        is_infinity: true,
        curve: new_secp256k1curve()
    }
}

fn new_secp256k1point_g() -> Secp256k1Point {
    let x = BigUint::from_str_radix("79BE667EF9DCBBAC55A06295CE870B07029BFCDB2DCE28D959F2815B16F81798",16).unwrap();
    let y = BigUint::from_str_radix("483ADA7726A3C4655DA4FBFC0E1108A8FD17B448A68554199C47D08FFB10D4B8",16).unwrap();
    return new_secp256k1point_from_big_uint(x,y);
}

fn new_secp256k1point_from_hex_str(x: &str,y: &str) -> Option<Secp256k1Point> {
    let x = BigUint::from_str_radix(x,16);
    if x.is_err() {
        return None;
    }
    let y = BigUint::from_str_radix(y,16);
    if y.is_err() {
        return None;
    }
    Some(new_secp256k1point_from_big_uint(x.unwrap(),y.unwrap()))
}



#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;
    use crate::ecc::secp256k1_field::{new_secp256k1element_from_hex_str};
    use crate::ecc::secp256k1_scalar_element::new_secp256k1scalarelement_from_hex_str;

    #[test]
    fn test_base_point() {
        // ベースポイントGx,Gyに位数nを掛けたら無限遠点が帰ること
        let base = new_secp256k1point_g();
        let n = BigUint::from_str_radix("FFFFFFFFFFFFFFFFFFFFFFFFFFFFFFFEBAAEDCE6AF48A03BBFD25E8CD0364141", 16).unwrap();
        assert_eq!(new_secp256k1point_infinity(),base.mul_from_big_uint(n));
    }

    #[test]
    fn test_signature_practice_p69q6() {

        {
            // signature 1
            let px = "887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c";
            let py = "61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34";
            let p = new_secp256k1point_from_hex_str(px,py).unwrap();
            let z = new_secp256k1scalarelement_from_hex_str("ec208baa0fc1c19f708a9ca96fdeff3ac3f230bb4a7ba4aede4942ad003c0f60").unwrap();
            let r = new_secp256k1scalarelement_from_hex_str("ac8d1c87e51d0d441be8b3dd5b05c8795b48875dffe00b7ffcfac23010d3a395").unwrap();
            let s = new_secp256k1scalarelement_from_hex_str("68342ceff8935ededd102dd876ffd6ba72d6a427a3edb13d26eb0781cb423c4").unwrap();

            let u = z.clone()/s.clone();
            let v= r.clone()/s.clone();
            let g = new_secp256k1point_g();
            let ug = g.clone().mul_from_sec256k1scalar_element(u.clone());
            let vp = p.clone().mul_from_sec256k1scalar_element(v.clone());
            let r_point = ug.clone() + vp.clone();
            println!("u: {} v: {} r_point: {} g: {} p: {} ug: {} vp: {}",u.clone(),v.clone(),r_point.clone(),g.clone(),p.clone(),ug,vp);
            // Rxとrが一致していれば署名は有効
            assert_eq!(r_point.x.num,r.num);
        }
        {
            // signature 1
            let px = "887387e452b8eacc4acfde10d9aaf7f6d9a0f975aabb10d006e4da568744d06c";
            let py = "61de6d95231cd89026e286df3b6ae4a894a3378e393e93a0f45b666329a0ae34";
            let p = new_secp256k1point_from_hex_str(px,py).unwrap();
            let z = new_secp256k1scalarelement_from_hex_str("7c076ff316692a3d7eb3c3bb0f8b1488cf72e1afcd929e29307032997a838a3d").unwrap();
            let r = new_secp256k1scalarelement_from_hex_str("eff69ef2b1bd93a66ed5219add4fb51e11a840f404876325a1e8ffe0529a2c").unwrap();
            let s = new_secp256k1scalarelement_from_hex_str("c7207fee197d27c618aea621406f6bf5ef6fca38681d82b2f06fddbdce6feab6").unwrap();

            let u = z.clone()/s.clone();
            let v= r.clone()/s.clone();
            let g = new_secp256k1point_g();
            let r_point = g.mul_from_sec256k1scalar_element(u) + p.mul_from_sec256k1scalar_element(v);
            // Rxとrが一致していれば署名は有効
            assert_eq!(r_point.x.num,r.num);
        }
    }
}
