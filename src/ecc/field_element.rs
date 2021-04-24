use std::fmt::{Display, Formatter};
use std::fmt;
use std::ops::{Sub, Add, Mul, Rem, Div};

// Debugの自動実装
#[derive(Debug, Copy, Clone)]
pub struct FieldElement {
    pub num: u64,
    pub prime: u64
}

// P.5 練習問題1
impl PartialEq for FieldElement {
    fn eq(&self, other: &Self) -> bool {
        return self.num == other.num && self.prime == other.prime;
    }
}

impl Add for FieldElement {
    type Output = FieldElement;

    // 左側のprimeに依存させる。
    fn add(self, rhs: FieldElement) -> FieldElement {
        Self::Output{
            prime: self.prime,
            num: (self.num+rhs.num).rem_euclid(self.prime)
        }
    }
}

impl Sub for FieldElement {
    type Output = FieldElement;

    fn sub(self, rhs: FieldElement) -> FieldElement {
        Self::Output{
            prime: self.prime,
            num: ((self.num as i128) - (rhs.num as i128)).rem_euclid(self.prime as i128) as u64
        }
    }
}

impl Mul for FieldElement {
    type Output = FieldElement;

    fn mul(self, rhs: FieldElement) -> FieldElement {
        Self::Output{
            prime: self.prime,
            num: ((self.num as i128) * (rhs.num as i128)).rem_euclid(self.prime as i128) as u64
        }
    }
}

impl Rem for FieldElement {
    type Output = FieldElement;

    fn rem(self, rhs: FieldElement) -> FieldElement {
        Self::Output{
            prime: self.prime,
            num: ((self.num as i128) % (rhs.num as i128)).rem_euclid(self.prime as i128) as u64
        }
    }
}


// TODO: どうにかして実装したい。
impl FieldElement {
    fn inner_pow(self,f: FieldElement,exp: u64) -> FieldElement {
        if exp == 0 {
            return FieldElement{
                num: 1,
                prime: f.prime
            }
        }
        if exp % 2 == 0 {
            return self.inner_pow(FieldElement{
                num: (f.num * f.num).rem_euclid(f.prime),
                prime: f.prime,
            },exp / 2);
        }
        f * f.inner_pow(FieldElement{
            num: (f.num * f.num).rem_euclid(f.prime),
            prime: f.prime,
        },(exp-1)/2)
    }
    // rem_euclidを使って負数でもよしなに整数値に変更する。
    pub fn pow(self, exp: i64) -> FieldElement {
        self.inner_pow(self, exp.rem_euclid((self.prime - 1) as i64) as u64)
    }

    // フェルマーの小定理からインバースを実装する。 位数が素数で無い場合は正しく動作しない
    pub fn inv(self) -> FieldElement {
        return self.pow((self.prime - 2) as i64)
    }
}

impl Div for FieldElement {
    type Output = FieldElement;

    fn div(self, rhs: Self) -> Self::Output {
        return self * rhs.inv();
    }
}

impl Display for FieldElement {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f,"{}",self.num)
    }
}

pub fn new_field_element(num: i64, prime: u64) -> FieldElement {
    FieldElement{
        num: num.rem_euclid(prime as i64) as u64,
        prime
    }
}

#[cfg(test)]
mod tests {
    extern crate test;
    use super::*;

    #[test]
    fn test_field_element_all() {
        {
            let a = new_field_element(7,13);
            let b = new_field_element(6,13);

            println!("{}",a == b);
            println!("{}",a != b);

            println!("{}",a == a);
            println!("{}",a != a);
        }

        // P.9 練習問題2
        {
            println!("P.9 Q2");
            {
                let a = new_field_element(44,57);
                let b = new_field_element(33,57);
                println!("{}",a + b);
            }
            {
                let a = new_field_element(9,57);
                let b = new_field_element(-29,57);
                println!("{}", a + b);
            }
            {
                let a = new_field_element(17,57);
                let b = new_field_element(42,57);
                let c = new_field_element(49,57);
                println!("{}", a + b + c);
            }
            {
                let a = new_field_element(52,57);
                let b = new_field_element(-30,57);
                let c = new_field_element(-38,57);
                println!("{}", a + b + c);
            }
        }

        // P.10 練習問題3
        {
            println!("P.10 Q3");
            {
                let a = new_field_element(44,57);
                let b = new_field_element(33,57);
                println!("{}",a - b);
            }
            {
                let a = new_field_element(9,57);
                let b = new_field_element(-29,57);
                println!("{}", a - b);
            }
            {
                let a = new_field_element(17,57);
                let b = new_field_element(42,57);
                let c = new_field_element(49,57);
                println!("{}", a + b - c);
            }
            {
                let a = new_field_element(52,57);
                let b = new_field_element(-30,57);
                let c = new_field_element(-38,57);
                println!("{}", a + b - c);
            }
        }

        // P.11 練習問題4
        {
            println!("P.11 Q4");
            {
                let a = new_field_element(95,97);
                let b = new_field_element(45,97);
                let c = new_field_element(31,97);
                println!("{}",a * b * c);
            }
            {
                let a = new_field_element(17,97);
                let b = new_field_element(13,97);
                let c = new_field_element(19,97);
                let d = new_field_element(44,97);
                println!("{}", a * b * c * d);
            }
            {
                let a = new_field_element(12,97).pow(7);
                let b = new_field_element(77,97).pow(49);
                println!("{}", a * b);
            }
        }

        // P.11 練習問題5
        {
            println!("P.11 Q5");
            let solver = |k: i32|{
                print!("k = {{");
                for i in 0..19 {
                    let f = new_field_element((i * k) as i64, 19);
                    print!(" {},",f)
                }
                print!("}} \n");
            };
            solver(1);
            solver(3);
            solver(7);
            solver(17);
            solver(18);
        }

        // P.12 練習問題6
        {
            println!("P.12 Q6");
            let f = new_field_element(7 * 7,31);
            println!("F31 7 * 7 = {}",f)
        }

        // P.13 練習問題7
        {
            println!("P.13 Q7");
            let solver = |k: i32|{
                print!("k = {} {{",k);
                for i in 0..k-1 {
                    let f = new_field_element((i + 1) as i64, k as u64);
                    print!(" {},",f.pow((k - 1) as i64))
                }
                print!("}} \n");
            };

            solver(7);
            solver(11);
            solver(17);
            solver(31);
        }

        // P.16 練習問題8
        {
            println!("P.16 練習問題");
            {
                let l = new_field_element(3,31);
                let r = new_field_element(24,31);
                println!("F31 3/24 = {}",l/r);
            }
            {
                let l = new_field_element(17,31);
                println!("F31 17^(-3) = {}",l.pow(-3));
            }
            {
                let l = new_field_element(4,31);
                println!("F31 4^(-4) * 11 = {}",l.pow(-4) * new_field_element(11,31));
            }
        }

        // P.16 練習問題9
        {
            {
                let l = new_field_element(3,31);
                let r = new_field_element(24,31);
                println!("F31 3/24 = {}",l/r);
            }
        }
    }
}

