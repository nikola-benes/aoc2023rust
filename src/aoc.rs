use std::io;
use std::iter::*;
use std::slice::Iter;
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

    fn filter_map_v<B, F>(self, f: F) -> Vec<B>
    where
        Self: Sized,
        F: FnMut(Self::Item) -> Option<B>,
    {
        self.filter_map(f).collect()
    }

    fn flatten_v<B>(self) -> Vec<B>
    where
        Self: Sized,
        Self::Item: IntoIterator<Item = B>,
    {
        self.flatten().collect()
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

    fn sorted_it(self) -> <Vec<Self::Item> as IntoIterator>::IntoIter
    where
        Self: Sized,
        Self::Item: Ord,
    {
        self.collect::<Vec<_>>().sorted().into_iter()
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

pub trait SlicePlus {
    type Item;
    fn _s(&self) -> &[Self::Item];

    fn enumerate(&self) -> Enumerate<Iter<Self::Item>> {
        self._s().iter().enumerate()
    }

    fn all<F>(&self, f: F) -> bool
    where
        F: FnMut(&Self::Item) -> bool,
    {
        self._s().iter().all(f)
    }

    fn map<B, F>(&self, f: F) -> Map<Iter<Self::Item>, F>
    where
        F: FnMut(&Self::Item) -> B,
    {
        self._s().iter().map(f)
    }
}

impl<T> SlicePlus for [T] {
    type Item = T;
    fn _s(&self) -> &[T] {
        self
    }
}

impl<T> SlicePlus for Vec<T> {
    type Item = T;
    fn _s(&self) -> &[T] {
        self
    }
}

pub trait Sorted {
    type Item;
    fn sorted(self) -> Vec<Self::Item>;
}

impl<T: Ord> Sorted for Vec<T> {
    type Item = T;
    fn sorted(mut self) -> Self {
        self.sort();
        self
    }
}
