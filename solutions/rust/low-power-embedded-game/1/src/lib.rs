// This stub file contains items which aren't used yet; feel free to remove this module attribute
// to enable stricter warnings.
#![allow(unused)]

pub fn divmod(dividend: i16, divisor: i16) -> (i16, i16) {
    // 5 % 2 => (2, 1
    let a = dividend % divisor;
    let b = dividend / divisor;
    (b, a)
} 


pub fn evens<T>(iter: impl Iterator<Item = T>) -> impl Iterator<Item = T> {
     iter.step_by(2)
}

pub struct Position(pub i16, pub i16);
impl Position {
    pub fn manhattan(&self) -> i16 {
        let a = Position(self.0, self.1);
        (a.0).abs() + (a.1).abs()
    }
}
