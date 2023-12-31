use std::borrow::Borrow;
use std::io;
use std::iter::*;
use std::ops::{Index, IndexMut};
use std::slice::Chunks;
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

    fn to_pair(&mut self) -> (Self::Item, Self::Item)
    where
        Self: Sized,
    {
        let a = self.next_();
        let b = self.next_();
        (a, b)
    }

    fn to_triple(&mut self) -> (Self::Item, Self::Item, Self::Item)
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

    fn to_grid<T>(self) -> Grid<T>
    where
        Self: Sized,
        Self::Item: IntoIterator<Item = T>,
    {
        self.collect()
    }
}

impl<T: Iterator> IteratorPlus for T {}

pub trait StringPlus: Borrow<str> {
    fn parse_<F: FromStr>(&self) -> F {
        self.borrow().parse().ok().unwrap()
    }

    fn chars_v(&self) -> Vec<char> {
        self.borrow().chars().collect()
    }
}

impl<T: Borrow<str>> StringPlus for T {}

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

    fn filter<'a, P>(&'a self, p: P) -> Filter<Self::Iter<'a>, P>
    where
        P: FnMut(&<Self::Iter<'a> as Iterator>::Item) -> bool,
    {
        self._it().filter(p)
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct Grid<T> {
    pub rows: usize,
    pub cols: usize,
    tiles: Vec<T>,
}

impl<T, I: TryInto<usize>> Index<(I, I)> for Grid<T> {
    type Output = T;

    fn index(&self, i: (I, I)) -> &T {
        self.tiles.index(self._index(i))
    }
}

impl<T, I: TryInto<usize>> IndexMut<(I, I)> for Grid<T> {
    fn index_mut(&mut self, i: (I, I)) -> &mut T {
        self.tiles.index_mut(self._index(i))
    }
}

impl<T, A: IntoIterator<Item = T>> FromIterator<A> for Grid<T> {
    fn from_iter<I: IntoIterator<Item = A>>(iter: I) -> Self {
        let mut ts = Vec::new();
        let mut rows = 0;
        let mut cols = 0;
        for row in iter {
            let size = ts.len();
            ts.extend(row);
            let new_cols = ts.len() - size;
            if cols != 0 && new_cols != cols {
                panic!();
            }
            cols = new_cols;
            rows += 1;
        }

        Self {
            rows: rows,
            cols: cols,
            tiles: ts,
        }
    }
}

impl<T> Grid<T> {
    fn _index<I: TryInto<usize>>(&self, (y, x): (I, I)) -> usize {
        let y = y.try_into().ok().unwrap();
        let x = x.try_into().ok().unwrap();
        assert!(x < self.cols);
        y * self.cols + x
    }

    pub fn new(rows: usize, cols: usize, elem: T) -> Self
    where
        T: Clone,
    {
        Self {
            rows,
            cols,
            tiles: vec![elem.clone(); rows * cols],
        }
    }

    pub fn valid_row<I: TryInto<usize>>(&self, y: I) -> bool {
        y.try_into().is_ok_and(|y| y < self.rows)
    }

    pub fn valid_col<I: TryInto<usize>>(&self, x: I) -> bool {
        x.try_into().is_ok_and(|x| x < self.cols)
    }

    pub fn valid_coords<I: TryInto<usize>>(&self, y: I, x: I) -> bool {
        self.valid_row(y) && self.valid_col(x)
    }

    pub fn as_rows(&self) -> Chunks<'_, T> {
        self.tiles.chunks(self.cols)
    }

    pub fn as_tiles(&self) -> &[T] {
        &self.tiles
    }

    pub fn enumerate_tiles(&self) -> impl Iterator<Item = (usize, usize, &T)> {
        self.tiles
            .enumerate()
            .map(|(i, tile)| (i / self.cols, i % self.cols, tile))
    }

    pub fn swap<I: TryInto<usize>>(&mut self, a: (I, I), b: (I, I)) {
        let a = self._index(a);
        let b = self._index(b);
        self.tiles.swap(a, b);
    }

    pub fn transpose_clone(&self) -> Self
    where
        T: Clone,
    {
        // Doing transpose in place seems hard if the grid is not square.
        let mut v = Vec::new();
        v.reserve(self.tiles.len());
        for x in 0..self.cols {
            for y in 0..self.rows {
                v.push(self[(y, x)].clone());
            }
        }
        Self {
            rows: self.cols,
            cols: self.rows,
            tiles: v,
        }
    }
}
