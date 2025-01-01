mod direction;
mod wrapped;
mod set;

pub use direction::*;
pub use wrapped::*;
pub use set::*;

pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    if b == 0 { return a }
    let temp = a;
    a = b;
    b = temp % b;
    gcd(a, b)
}
pub fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}