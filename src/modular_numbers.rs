use crate::clean_up::big;
use num_bigint::BigUint;
use num_integer::Integer;
use std::fmt;

//Modular number, private fields to prevent creating invalid modular numbers.
// Aka fields of 0 or values greater than the field.
#[derive(Debug, Eq, PartialEq)]
pub struct ModNum {
    value: BigUint,
    field: BigUint,
}

impl ModNum {
    //creates a new modular number and forces the value to conform.
    // panics if the field is zero.
    pub fn new(value: &BigUint, field: &BigUint) -> ModNum {
        assert!(
            field != &big(0),
            "Cannot create a modular number with field of  0."
        );
        ModNum {
            value: value % field,
            field: field.clone(),
        }
    }

    pub fn value(&self) -> &BigUint {
        &self.value
    }
    pub fn field(&self) -> &BigUint {
        &self.field
    }

    // Chose to create add function instead of overriding because the standard function requires
    //  consuming the left hand side. This prevents potentially copying very large numbers with the
    //   downside of being slightly unreadable.
    pub fn add(&self, other: &Self) -> Self {
        // Ensures self/other are from the same field. Operations are undefined for values from
        //  different fields
        assert!(
            self.field == other.field,
            "Can't add Modular Numbers with different fields"
        );
        let value: BigUint = (&self.value + &other.value) % &self.field;
        ModNum {
            value: value,
            field: self.field.clone(),
        }
    }

    // Chose to create mul function instead of overriding because the standard function requires
    //  consuming the left hand side. This prevents potentially copying very large numbers with the
    //   downside of being slightly unreadable.
    pub fn mul(&self, other: &Self) -> Self {
        // Ensures self/other are from the same field. Operations are undefined for values from
        //  different fields
        assert!(
            self.field == other.field,
            "Can't add Modular Numbers with different fields"
        );
        let value: BigUint = (&self.value * &other.value) % &self.field;
        ModNum {
            value,
            field: self.field.clone(),
        }
    }

    // Returns the additive inverse of self - self + self.add_inv() = 0
    pub fn add_inv(&self) -> Self {
        let value: BigUint = (&self.field - &self.value) % &self.field;
        ModNum {
            value,
            field: self.field().clone(),
        }
    }

    // Returns the multiplicate inverse of self - self * self.mul_inv() = 1
    // Uses Extended Euclidean Algorithm to find the modular multiplicative inverse.
    //    https://en.wikipedia.org/wiki/Extended_Euclidean_algorithm
    pub fn mul_inv(&self) -> Self {
        let mut r_one = self.value.clone();
        let mut r_zero = self.field.clone();
        let mut t_zero = big(0);
        let mut t_one = big(1);
        let mul_field = &self.field;
        loop {
            // Div_mod_floor returns (quotient, remainder)
            let q = (&r_zero).div_mod_floor(&r_one);
            r_zero = r_one.clone();
            r_one = q.1.clone();
            let test_val = q.0 * &t_one;
            let t_temp = if t_zero < test_val {
                t_zero + (mul_field - (test_val % mul_field))
            } else {
                t_zero - test_val
            };
            t_zero = t_one.clone();
            t_one = t_temp.clone();
            if q.1 == big(0) {
                break;
            }
        }

        ModNum {
            value: t_zero,
            field: self.field.clone(),
        }
    }

    // Returns self^exp - uses the recursive algorithm implementation
    pub fn pow(&self, exp: &BigUint) -> Self {
        if exp == &big(0) {
            return ModNum {
                value: big(1),
                field: self.field.clone(),
            };
        } else if exp == &big(1) {
            return ModNum {
                value: self.value.clone(),
                field: self.field.clone(),
            };
        } else {
            if exp % &big(2) == big(0) {
                let z = self.pow(&(exp / &big(2)));
                return z.mul(&z);
            } else {
                let z = self.pow(&((exp - &big(1)) / &big(2)));

                return z.mul(&z).mul(&self);
            }
        }
    }

    // Returns the sqrt(self) if it exists.
    //  Returns 0 if the sqrt(self) doesn't exist.
    // In a finite field only ~50% of numbers have a square root or "is a quadratic residue of the field."
    pub fn sqrt(&self) -> Self {
        let sqrt_field = &self.field;

        // this function is used first to check that self is a quadratic residue aka has a square root.
        fn legendre_symbol(value: &BigUint, field: &BigUint) -> i8 {
            // value^((field -1)/2) mod field
            let leg_sym = value.modpow(&((field - &big(1)) / big(2)), field);

            // if value^((field -1)/2) mod field is congruent to -1 -(field -1) return -1
            // Have to use (field - 1) instead of -1 because these are unsigned integers.
            if leg_sym == field - &big(1) {
                return -1;
            }
            // if value^((field -1)/2) mod field is congruent to 1 it is a quadratic residue
            else if leg_sym == big(1) {
                return 1;
            }
            // This is a placeholder value thats only use is that it is not 1 or -1.
            else {
                return 2;
            }
        }
        // Check to determine if self is a quadratic residue
        //  If self isn't a quadratic residue return 0.
        if legendre_symbol(&self.value, sqrt_field) != 1 {
            return ModNum::new(&big(0), &self.field);
        }
        // sqrt(0) = 0
        if self.value == big(0) {
            return ModNum::new(&big(0), &self.field);
        }
        // If the field is congruent to 3 mod 4 then we can directly calculate the sqrt with this formula.
        if self.field.mod_floor(&big(4)) == big(3) {
            // self.value^((field+1)/4) mod field
            let value = (&self.value).modpow(&((&self.field + big(1)) / &big(4)), &self.field);
            return ModNum {
                value,
                field: self.field.clone(),
            };
        }

        let mut s = &self.field - big(1);
        let mut e = big(1);

        while s.mod_floor(&big(2)) == big(0) {
            s = s.mod_floor(&big(2));
            e = e + &big(1);
        }
        let mut n = big(2);

        // Looking for a value of n congruent to field -1 mod field
        while legendre_symbol(&n, &self.field) != -1 {
            n = n + &big(1);
        }
        let mut xenon = (&self.value).modpow(&((&s + &big(1)) / &big(2)), sqrt_field);
        let mut baby = self.value.modpow(&s, sqrt_field);
        let mut garnish = n.modpow(&s, sqrt_field);

        loop {
            let mut t = baby.clone();
            let mut m = big(0);

            for i in 0..256 {
                if t == big(1) {
                    break;
                }
                t = t.modpow(&big(2), sqrt_field);
                m = m + &big(1);
                if i == 256 {
                    println!("You should increase the size of for loop in sqrt function!");
                    panic!();
                }
                if m == big(0) {
                    println!("xenon is {:?}", xenon);
                    return ModNum::new(&xenon, &self.field);
                }
                let garnishes = (&garnish).modpow(
                    &(&big(2).modpow(&(&e - &m - &big(1)), sqrt_field)),
                    sqrt_field,
                );
                garnish = (&garnishes).modpow(&big(2), sqrt_field);
                xenon = (xenon * garnishes).mod_floor(sqrt_field);
                baby = (baby * &garnish).mod_floor(sqrt_field);
                e = m.clone();
            }
        }
    }
}

impl Clone for ModNum {
    fn clone(&self) -> ModNum {
        ModNum::new(&self.value(), &self.field())
    }
}

impl fmt::Display for ModNum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} mod {}", self.value(), self.field())
    }
}
