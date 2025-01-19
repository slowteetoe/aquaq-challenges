use num_bigint::BigUint;
use std::ops::Mul;

pub fn solve() -> BigUint {
    BigUint::from(203217u32)
        .mul(BigUint::from(151018u32))
        .mul(BigUint::from(482359u32))
        .mul(BigUint::from(782486u32))
        .mul(BigUint::from(281651u32))
        .mul(BigUint::from(721924u32))
        .mul(BigUint::from(945710u32))
        .mul(BigUint::from(131962u32))
        .mul(BigUint::from(78308u32))
        .mul(BigUint::from(661224u32))
}
