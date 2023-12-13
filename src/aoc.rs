use std::io;
use std::iter::*;
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

pub trait IterablePlus {
    type Iter<'a>: Iterator
    where
        Self: 'a;

    fn _it<'a>(&'a self) -> Self::Iter<'a>;

    fn enumerate<'a>(&'a self) -> Enumerate<Self::Iter<'a>> {
        self._it().enumerate()
    }

    fn all<'a, F>(&'a self, f: F) -> bool
    where
        F: FnMut(<Self::Iter<'a> as Iterator>::Item) -> bool,
    {
        self._it().all(f)
    }

    fn map<'a, B, F>(&'a self, f: F) -> Map<Self::Iter<'a>, F>
    where
        F: FnMut(<Self::Iter<'a> as Iterator>::Item) -> B,
    {
        self._it().map(f)
    }

    fn map_v<'a, B, F>(&'a self, f: F) -> Vec<B>
    where
        F: FnMut(<Self::Iter<'a> as Iterator>::Item) -> B,
    {
        self._it().map_v(f)
    }

    fn filter_map<'a, B, F>(&'a self, f: F) -> FilterMap<Self::Iter<'a>, F>
    where
        F: FnMut(<Self::Iter<'a> as Iterator>::Item) -> Option<B>,
    {
        self._it().filter_map(f)
    }

    fn cloned<'a, T>(&'a self) -> Cloned<Self::Iter<'a>>
    where
        T: Clone + 'a,
        Self::Iter<'a>: Iterator<Item = &'a T>,
    {
        self._it().cloned()
    }

    fn position<'a, P>(&'a self, p: P) -> Option<usize>
    where
        P: FnMut(<Self::Iter<'a> as Iterator>::Item) -> bool,
    {
        self._it().position(p)
    }

    fn rev<'a>(&'a self) -> Rev<Self::Iter<'a>>
    where
        Self::Iter<'a>: DoubleEndedIterator,
    {
        self._it().rev()
    }

    fn zip<'a, U>(
        &'a self,
        other: U,
    ) -> Zip<Self::Iter<'a>, <U as IntoIterator>::IntoIter>
    where
        U: IntoIterator,
    {
        self._it().zip(other)
    }

    fn fold1_<'a, F, T>(&'a self, f: F) -> T
    where
        T: Clone + 'a,
        F: FnMut(T, &'a T) -> T,
        Self::Iter<'a>: IteratorPlus<Item = &'a T>,
    {
        self._it().fold1_(f)
    }
}

impl<T> IterablePlus for T
where
    T: ?Sized,
    for<'a> &'a T: IntoIterator,
{
    type Iter<'a> = <&'a T as IntoIterator>::IntoIter where Self: 'a;

    fn _it<'a>(&'a self) -> Self::Iter<'a> {
        self.into_iter()
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
