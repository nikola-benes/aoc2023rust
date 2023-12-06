use std::io;
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
}

impl<T: Iterator> IteratorPlus for T {}

pub trait StringPlus: Sized {
    fn parse_<F: FromStr>(&self) -> F;
}

impl StringPlus for &str {
    fn parse_<F: FromStr>(&self) -> F {
        self.parse().ok().unwrap()
    }
}
