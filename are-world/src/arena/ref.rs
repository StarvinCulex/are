use std::borrow::{Borrow, BorrowMut};
use std::marker::PhantomData;
use std::sync;
use std::sync::{Arc, RwLock};

use duplicate::duplicate;

use crate::arena::cosmos::PKey;
use crate::arena::Cosmos;

pub struct P<Element, ReadKey = Cosmos, WriteKey = PKey>
where
    Element: ?Sized,
{
    data: sync::Arc<Element>,
    _ak: PhantomData<ReadKey>,
    _wk: PhantomData<WriteKey>,
}

impl<Element, AccessKey> Clone for P<Element, AccessKey>
where
    Element: ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            _ak: PhantomData::default(),
            _wk: PhantomData::default(),
        }
    }
}

impl<Element, AccessKey> PartialEq for P<Element, AccessKey>
where
    Element: ?Sized,
{
    fn eq(&self, other: &P<Element, AccessKey>) -> bool {
        sync::Arc::as_ptr(&self.data) == sync::Arc::as_ptr(&other.data)
    }
}

impl<Element, AccessKey> Eq for P<Element, AccessKey> where Element: ?Sized {}

impl<Element, AccessKey> std::hash::Hash for P<Element, AccessKey>
where
    Element: ?Sized,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        sync::Arc::as_ptr(&self.data).hash(state)
    }
}

pub struct Weak<Element, ReadKey = Cosmos, WriteKey = PKey>
where
    Element: ?Sized,
{
    data: sync::Weak<Element>,
    _ak: PhantomData<ReadKey>,
    _wk: PhantomData<WriteKey>,
}

impl<Element, AccessKey> Clone for Weak<Element, AccessKey>
where
    Element: ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            _ak: PhantomData::default(),
            _wk: PhantomData::default(),
        }
    }
}

impl<Element, AccessKey> PartialEq for Weak<Element, AccessKey>
where
    Element: ?Sized,
{
    fn eq(&self, other: &Weak<Element, AccessKey>) -> bool {
        sync::Weak::as_ptr(&self.data) == sync::Weak::as_ptr(&other.data)
    }
}

impl<Element, AccessKey> Eq for Weak<Element, AccessKey> where Element: ?Sized {}

impl<Element, AccessKey> std::hash::Hash for Weak<Element, AccessKey>
where
    Element: ?Sized,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        sync::Weak::as_ptr(&self.data).hash(state)
    }
}

impl<Element, AccessKey> P<Element, AccessKey>
where
    Element: Sized,
{
    #[inline]
    pub fn new(e: Element) -> Self {
        Self {
            data: Arc::new(e),
            _ak: PhantomData::default(),
            _wk: PhantomData::default(),
        }
    }
}

impl<Element, AccessKey> P<Element, AccessKey>
where
    Element: ?Sized,
{
    #[inline]
    pub unsafe fn from_raw(ptr: *const Element) -> Self {
        Self {
            data: Arc::from_raw(ptr),
            _ak: PhantomData::default(),
            _wk: PhantomData::default(),
        }
    }
}

impl<Element, AccessKey, WriteKey> P<Element, AccessKey, WriteKey>
where
    Element: ?Sized,
{
    #[inline]
    pub fn get(&self, _: &'_ AccessKey) -> &Element {
        self.data.as_ref()
    }

    pub unsafe fn get_mut<'a>(&'a self, _: &'_ WriteKey) -> &mut Element {
        (self.data.as_ref() as *const Element as *mut Element)
            .as_mut::<'a>()
            .unwrap()
    }

    pub fn downgrade(&self) -> Weak<Element, AccessKey> {
        Weak {
            data: Arc::downgrade(&self.data),
            _ak: PhantomData::default(),
            _wk: PhantomData::default(),
        }
    }
}

impl<Element, AccessKey, WriteKey> Weak<Element, AccessKey, WriteKey> {
    pub fn new() -> Self {
        Self {
            data: sync::Weak::new(),
            _ak: PhantomData::default(),
            _wk: PhantomData::default(),
        }
    }
}

impl<Element, AccessKey> Weak<Element, AccessKey>
where
    Element: ?Sized,
{
    pub fn upgrade(self) -> Option<P<Element, AccessKey>> {
        self._upgrade()
    }

    pub fn strong_count(&self) -> usize {
        self.data.strong_count()
    }

    pub fn weak_count(&self) -> usize {
        self.data.weak_count()
    }

    fn _upgrade(&self) -> Option<P<Element, AccessKey>> {
        self.data.upgrade().map(|x| P {
            data: x,
            _ak: PhantomData::default(),
            _wk: PhantomData::default(),
        })
    }
}
