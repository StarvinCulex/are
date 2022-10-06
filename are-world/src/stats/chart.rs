use std::cmp::Ordering;
use std::collections::hash_map::Entry;
use std::collections::{BinaryHeap, HashMap};
use std::fmt::{Debug, Display, Formatter};
use std::marker::PhantomData;
use std::ops::{Add, AddAssign};

pub trait Row: Display + Ord + Default {
    type V;
    fn add(&mut self, value: Self::V);
    fn merge(&mut self, value: Self);
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Default)]
pub struct Count {
    n: i64,
}

impl Display for Count {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(&self.n, f)
    }
}

impl Row for Count {
    type V = i64;

    #[inline]
    fn add(&mut self, value: Self::V) {
        self.n += value;
    }

    #[inline]
    fn merge(&mut self, value: Count) {
        self.n += value.n;
    }
}

pub struct Chart<K, R>
where
    K: Display + std::hash::Hash + Eq,
    R: Row,
{
    table: HashMap<K, R>,
}

impl<K, R> Chart<K, R>
where
    K: Display + std::hash::Hash + Eq,
    R: Row,
{
    #[inline]
    pub fn new() -> Self {
        Self {
            table: HashMap::new(),
        }
    }

    #[inline]
    pub fn add(&mut self, key: K, value: R::V) {
        self.table.entry(key).or_insert_with(R::default).add(value);
    }

    pub fn clear(&mut self) {
        self.table.clear();
    }
}

impl<K, R> AddAssign for Chart<K, R>
where
    K: Display + std::hash::Hash + Eq,
    R: Row,
{
    fn add_assign(&mut self, rhs: Self) {
        for (k, v) in rhs.table {
            match self.table.entry(k) {
                Entry::Occupied(mut o) => {
                    o.get_mut().merge(v);
                }
                Entry::Vacant(mut o) => {
                    o.insert(v);
                }
            }
        }
    }
}

impl<K, R> Add for Chart<K, R>
where
    K: Display + std::hash::Hash + Eq,
    R: Row,
{
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self::Output {
        self.add_assign(rhs);
        self
    }
}

impl<K, R> Display for Chart<K, R>
where
    K: Display + std::hash::Hash + Eq,
    R: Row,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let heap: BinaryHeap<_> = self
            .table
            .iter()
            .map(|(k, v)| KR { key: k, row: v })
            .collect();
        for KR { row, key, .. } in heap.into_iter_sorted() {
            write!(f, "[{key}]\n{value}", key = key, value = row)?;
        }
        Ok(())
    }
}

impl<K, R> Debug for Chart<K, R>
where
    K: Display + std::hash::Hash + Eq,
    R: Row,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Display::fmt(self, f)
    }
}

struct KR<'a, K, R>
where
    R: Row,
{
    row: &'a R,
    key: &'a K,
}

impl<K, R> Eq for KR<'_, K, R> where R: Row {}

impl<K, R> PartialEq<Self> for KR<'_, K, R>
where
    R: Row,
{
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row
    }
}

impl<K, R> PartialOrd<Self> for KR<'_, K, R>
where
    R: Row,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.row.partial_cmp(other.row)
    }
}

impl<K, R> Ord for KR<'_, K, R>
where
    R: Row,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.row.cmp(other.row)
    }
}

impl<K, R> Default for Chart<K, R>
where
    K: Display + std::hash::Hash + Eq,
    R: Row,
{
    fn default() -> Self {
        Self::new()
    }
}
