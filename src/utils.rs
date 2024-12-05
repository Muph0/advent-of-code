use multimap::MultiMap;
use std::hash::{BuildHasher, Hash};
use std::mem::{transmute_copy, MaybeUninit};

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
