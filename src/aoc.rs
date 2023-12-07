use std::io;
use std::iter::Product;
use std::iter::Sum;
use std::str::FromStr;

pub fn lines() -> impl Iterator<Item = String> {
    io::stdin().lines().map(|l| l.unwrap())
}

pub trait IteratorPlus: Iterator {
    fn map_v<B, F>(self, f: F) -> Vec<B>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> B,
    {
        self.map(f).collect()
    }

    fn next_(&mut self) -> Self::Item {
        self.next().unwrap()
    }

    fn nth_(&mut self, n: usize) -> Self::Item {
        self.nth(n).unwrap()
    }

    fn reduce_<F>(self, f: F) -> Self::Item
    where
        Self: Sized,
        F: FnMut(Self::Item, Self::Item) -> Self::Item,
    {
        self.reduce(f).unwrap()
    }

    // This is a version of ‹reduce› that works for iterators over references
    // without cloning all of the elements. (It only clones the first one.)
    fn fold1_<'a, F, T>(mut self, f: F) -> T
    where
        T: Clone + 'a,
        Self: Sized + Iterator<Item = &'a T>,
        F: FnMut(T, &'a T) -> T,
    {
        let init = self.next_().clone();
        self.fold(init, f)
    }

    fn sum_(self) -> Self::Item
    where
        Self: Sized,
        Self::Item: Sum,
    {
        self.sum::<Self::Item>()
    }

    fn product_(self) -> Self::Item
    where
        Self: Sized,
        Self::Item: Product,
    {
        self.product::<Self::Item>()
    }

    fn to_pair(mut self) -> (Self::Item, Self::Item)
    where
        Self: Sized,
    {
        let a = self.next_();
        let b = self.next_();
        (a, b)
    }

    fn to_triple(mut self) -> (Self::Item, Self::Item, Self::Item)
    where
        Self: Sized,
    {
        let a = self.next_();
        let b = self.next_();
        let c = self.next_();
        (a, b, c)
    }
}

impl<T: Iterator> IteratorPlus for T {}

pub trait StringPlus {
    fn _s(&self) -> &str;
    fn parse_<F: FromStr>(&self) -> F {
        self._s().parse().ok().unwrap()
    }

    fn chars_v(&self) -> Vec<char> {
        self._s().chars().collect()
    }
}

impl StringPlus for &str {
    fn _s(&self) -> &str {
        self
    }
}

impl StringPlus for String {
    fn _s(&self) -> &str {
        self.as_str()
    }
}
