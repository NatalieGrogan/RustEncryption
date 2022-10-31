use num_bigint::BigUint;

pub fn big(x: u32) -> BigUint {
    BigUint::new(vec![x])
}
