use crate::numbers::signed_integers::SignedInteger;
use crate::numbers::value::Value;
use std::marker::PhantomData;
use std::ops::{Add, AddAssign, MulAssign};

pub struct ModInt<T: SignedInteger + Copy, V: Value<T>> {
    n: T,
    phantom: PhantomData<V>,
}

impl<T: SignedInteger + Copy, V: Value<T>> ModInt<T, V> {
    pub fn new(n: T) -> Self {
        let mut res = Self {
            n: n % (V::VAL),
            phantom: Default::default(),
        };
        res.safe();
        res
    }

    fn safe(&mut self) -> &mut Self {
        if self.n < 0.into() {
            self.n += V::VAL;
        } else if self.n >= V::VAL {
            self.n -= V::VAL;
        }
        self
    }
}

impl<T: SignedInteger + Copy, V: Value<T>> From<T> for ModInt<T, V> {
    fn from(n: T) -> Self {
        Self::new(n)
    }
}

impl<T: SignedInteger + Copy, V: Value<T>> AddAssign for ModInt<T, V> {
    fn add_assign(&mut self, rhs: Self) {
        self.n += rhs.n;
        self.safe();
    }
}

impl<T: SignedInteger + Copy, V: Value<T>> Add for ModInt<T, V> {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl<T: SignedInteger + Copy, V: Value<T>> MulAssign for ModInt<T, V> {
    fn mul_assign(&mut self, rhs: Self) {
        self.n = (T::wide_mul(self.n, rhs.n)
            % T::W::try_from(V::VAL).unwrap_or_else(|_| panic!("")))
        .try_into()
        .unwrap_or_else(|_| panic!(""));
        self.safe();
    }
}
