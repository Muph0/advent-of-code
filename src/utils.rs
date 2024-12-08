use multimap::MultiMap;
use std::fmt::{Display, Formatter};
use std::hash::{BuildHasher, Hash};
use std::mem::{transmute_copy, MaybeUninit};
use std::ops::{Index, IndexMut};
use std::slice::{Iter, IterMut};
use std::str::FromStr;

pub trait DestructIterator<T> {
    fn destruct<const N: usize>(self) -> [T; N];
}
impl<T, I: Iterator<Item = T>> DestructIterator<T> for I {
    fn destruct<const N: usize>(mut self) -> [T; N] {
        let mut slice: [MaybeUninit<T>; N] = unsafe { MaybeUninit::uninit().assume_init() };

        for i in 0..N {
            let next = self.next().unwrap_or_else(|| {
                panic!(
                    "Tried to destruct {N} items but only {} were available",
                    i - 1
                );
            });
            slice[i].write(next);
        }
        assert!(
            self.next().is_none(),
            "{N} items were destructed, but there is still {} available",
            self.count()
        );

        unsafe { transmute_copy(&slice) }
    }
}

pub trait MultiMapContains<K, V> {
    fn contains(&self, k: K, v: V) -> bool;
}
impl<K, V, S> MultiMapContains<K, V> for MultiMap<K, V, S>
where
    K: Eq + Hash,
    V: PartialEq,
    S: BuildHasher,
{
    fn contains(&self, k: K, v: V) -> bool {
        match self.get_vec(&k) {
            Some(vec) => vec.contains(&v),
            None => false,
        }
    }
}

pub trait IntoOptionIterator<T: IntoIterator> {
    fn iter(self) -> OptionIter<T::IntoIter>;
}
impl<T: IntoIterator> IntoOptionIterator<T> for Option<T> {
    fn iter(self) -> OptionIter<T::IntoIter> {
        OptionIter {
            it: self.map(|x| x.into_iter()),
        }
    }
}

pub struct OptionIter<I: Iterator> {
    it: Option<I>,
}
impl<I: Iterator> Iterator for OptionIter<I> {
    type Item = I::Item;
    fn next(&mut self) -> Option<Self::Item> {
        match &mut self.it {
            Some(it) => it.next(),
            None => None,
        }
    }
}

pub struct Grid2D<T> {
    width: i32,
    height: i32,
    data: Vec<T>,
}

impl<T> Grid2D<T> {
    pub fn new(width: i32, height: i32, value: T) -> Self
    where
        T: Clone,
    {
        Grid2D {
            width,
            height,
            data: vec![value; (width * height) as usize],
        }
    }

    pub fn width(&self) -> i32 {
        self.width
    }

    pub fn height(&self) -> i32 {
        self.height
    }

    pub fn get<I: Into<(i32, i32)>>(&self, idx: I) -> Option<&T> {
        let (x, y) = idx.into();
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.data.get((y * self.width + x) as usize)
        } else {
            None
        }
    }

    pub fn get_mut<I: Into<(i32, i32)>>(&mut self, idx: I) -> Option<&mut T> {
        let (x, y) = idx.into();
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.data.get_mut((y * self.width + x) as usize)
        } else {
            None
        }
    }

    pub fn iter(&self) -> Iter<T> {
        self.data.iter()
    }

    pub fn iter_mut(&mut self) -> IterMut<T> {
        self.data.iter_mut()
    }

    pub fn iter_indices(&self) -> impl Iterator<Item = (i32, i32)> {
        let width = self.width;
        (0..self.data.len()).map(move |i| {
            let x = i as i32 % width;
            let y = i as i32 / width;
            (x, y)
        })
    }

    pub fn map<R, F>(self, mut f: F) -> Grid2D<R>
    where
        F: FnMut(T) -> R,
    {
        let data = self.data.into_iter().map(&mut f).collect();
        Grid2D {
            width: self.width,
            height: self.height,
            data,
        }
    }

    pub fn index_of<F>(&self, mut predicate: F) -> Option<(i32, i32)>
    where
        F: FnMut(&T) -> bool,
    {
        self.data.iter().enumerate().find_map(|(i, item)| {
            if predicate(item) {
                let x = i as i32 % self.width;
                let y = i as i32 / self.width;
                Some((x, y))
            } else {
                None
            }
        })
    }
}

impl FromStr for Grid2D<char> {
    type Err = ();
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        let lines = input.lines();

        let width = lines.clone().next().unwrap().len() as i32;
        let height = lines.clone().filter(|l| l.trim().len() != 0).count() as i32;
        let mut data = Vec::new();
        data.reserve((width * height) as _);

        for l in lines {
            if data.len() as i32 == width * height {
                break;
            }
            assert_eq!(l.len(), width as usize);

            for c in l.chars() {
                data.push(c);
            }
        }
        Ok(Self {
            width,
            height,
            data,
        })
    }
}
impl<T: Display> Display for Grid2D<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(item) = self.get((x, y)) {
                    write!(f, "{} ", item)?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl<T, I: Into<(i32, i32)>> Index<I> for Grid2D<T> {
    type Output = T;
    fn index(&self, index: I) -> &Self::Output {
        let (x, y) = index.into();
        self.get((x, y)).unwrap_or_else(|| {
            panic!(
                "Index ({}, {}) is outside the grid dimensions ({}x{})",
                x, y, self.width, self.height
            )
        })
    }
}

impl<T, I: Into<(i32, i32)>> IndexMut<I> for Grid2D<T> {
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        let (x, y) = index.into();
        let width = self.width;
        let height = self.height;
        self.get_mut((x, y)).unwrap_or_else(|| {
            panic!(
                "Index ({}, {}) is outside the grid dimensions ({}x{})",
                x, y, width, height
            )
        })
    }
}

impl<T> IntoIterator for Grid2D<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}

impl<'a, T> IntoIterator for &'a Grid2D<T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter()
    }
}

impl<'a, T> IntoIterator for &'a mut Grid2D<T> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;
    fn into_iter(self) -> Self::IntoIter {
        self.data.iter_mut()
    }
}

pub struct AllPairsIter<I: Iterator + Clone> {
    outer_val: Option<I::Item>,
    outer: I,
    inner: I,
}
pub trait AllPairs<I: Iterator + Clone> {
    fn all_pairs(self) -> AllPairsIter<I>;
}
impl<I: Iterator + Clone> AllPairs<I> for I
where
    I::Item: Clone,
{
    fn all_pairs(self) -> AllPairsIter<I> {
        let outer = self.clone();
        let mut inner = self;
        inner.next();

        return AllPairsIter {
            outer_val: None,
            outer,
            inner,
        };
    }
}
impl<I: Iterator + Clone> Iterator for AllPairsIter<I>
where
    I::Item: Clone,
{
    type Item = (I::Item, I::Item);
    fn next(&mut self) -> Option<Self::Item> {
        if self.outer_val.is_none() {
            self.outer_val = self.outer.next();
        }
        while let Some(outer) = &self.outer_val {
            if let Some(inner) = self.inner.next() {
                return Some((outer.clone(), inner));
            }

            self.outer_val = self.outer.next();
            self.inner = self.outer.clone();
        }
        None
    }
}

#[test]
fn test_pairs() {
    let a = [1];
    let mut it = a.into_iter().all_pairs();
    assert_eq!(it.next(), None);

    let a = [1, 2];
    let mut it = a.into_iter().all_pairs();
    assert_eq!(it.next(), Some((1, 2)));
    assert_eq!(it.next(), None);

    let a = [1, 2, 3];
    let mut it = a.into_iter().all_pairs();
    assert_eq!(it.next(), Some((1, 2)));
    assert_eq!(it.next(), Some((1, 3)));
    assert_eq!(it.next(), Some((2, 3)));
    assert_eq!(it.next(), None);

    let a = [1, 2, 3, 4];
    let mut it = a.into_iter().all_pairs();
    assert_eq!(it.next(), Some((1, 2)));
    assert_eq!(it.next(), Some((1, 3)));
    assert_eq!(it.next(), Some((1, 4)));
    assert_eq!(it.next(), Some((2, 3)));
    assert_eq!(it.next(), Some((2, 4)));
    assert_eq!(it.next(), Some((3, 4)));
    assert_eq!(it.next(), None);
}
