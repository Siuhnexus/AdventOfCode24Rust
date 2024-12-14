use std::ops::{Add, AddAssign};

use num::{traits::Euclid, Num};

#[derive(Clone)]
pub struct Wrapped<T: Num + Euclid + Copy> {
    pub max: T,
    pub value: T
}

impl<T : Num + Euclid + Copy> Wrapped<T> {
    pub fn from(value: T, max: T) -> Self {
        Wrapped { value: value.rem_euclid(&max), max }
    }

    pub fn additions_until_target(&self, summand: T, target: T) -> Option<u64> {
        if summand % self.max == T::zero() { return if target == self.value { Some(0) } else { None } }
        let mut current = self.clone();
        let mut count = 0;
        loop {
            if current.value == target { return Some(count); }

            count += 1;
            current += summand;

            if current.value == self.value { return None; }
        };
    }
    pub fn additions_until_loop(&self, summand: T) -> u64 {
        let mut current = self.clone();
        let mut count = 0;
        loop {
            count += 1;
            current += summand;

            if current.value == self.value { return count; }
        }
    }
}
impl<T: Num + Euclid + Copy> Add for Wrapped<T> {
    type Output = Wrapped<T>;
    
    fn add(self, rhs: Self) -> Wrapped<T> {
        Wrapped { value: (self.value + rhs.value) % self.max, max: self.max }
    }
}
impl<T: Num + Euclid + Copy> Add<T> for Wrapped<T> {
    type Output = Wrapped<T>;
    
    fn add(self, rhs: T) -> Wrapped<T> {
        Wrapped { value: (self.value + rhs.into()) % self.max, max: self.max }
    }
}
impl<T: Num + Euclid + Copy> AddAssign for Wrapped<T> {
    fn add_assign(&mut self, rhs: Self) {
        self.value = (self.value + rhs.value) % self.max;
    }
}
impl<T: Num + Euclid + Copy> AddAssign<T> for Wrapped<T> {
    fn add_assign(&mut self, rhs: T) {
        self.value = (self.value + rhs.into()) % self.max;
    }
}