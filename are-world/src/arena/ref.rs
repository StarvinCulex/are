use std::borrow::{Borrow, BorrowMut};
use std::marker::PhantomData;
use std::sync;
use std::sync::{Arc, RwLock};

use crate::arena::cosmos::PKey;
use crate::arena::Cosmos;

use duplicate::duplicate;

duplicate! {
    [
        PtrType   UnderlyingPtr;
        [ P    ]  [ Arc        ];
        [ Weak ]  [ sync::Weak ];
    ]

pub struct PtrType<Element, ReadKey = Cosmos, WriteKey = PKey>
where
    Element: ?Sized,
{
    data: UnderlyingPtr<RwLock<Element>>,
    _ak: PhantomData<ReadKey>,
    _wk: PhantomData<WriteKey>,
}

impl<Element, AccessKey> Clone for PtrType<Element, AccessKey>
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

impl<Element, AccessKey> PartialEq for PtrType<Element, AccessKey>
where
    Element: ?Sized,
{
    fn eq(&self, other: &PtrType<Element, AccessKey>) -> bool {
        UnderlyingPtr::as_ptr(&self.data) == UnderlyingPtr::as_ptr(&other.data)
    }
}

impl<Element, AccessKey> Eq for PtrType<Element, AccessKey>
where
    Element: ?Sized,
{
}

impl<Element, AccessKey> std::hash::Hash for PtrType<Element, AccessKey>
where
    Element: ?Sized,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        UnderlyingPtr::as_ptr(&self.data).hash(state)
    }
}
}

impl<Element, AccessKey> P<Element, AccessKey>
where
    Element: Sized,
{
    #[inline]
    pub fn new(e: Element) -> Self {
        Self {
            data: Arc::new(RwLock::new(e)),
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
    pub fn get(&self, _: &'_ AccessKey) -> std::sync::RwLockReadGuard<Element> {
        self._get()
    }

    pub unsafe fn get_mut(&self, _: &'_ WriteKey) -> std::sync::RwLockWriteGuard<Element> {
        self._get_mut()
    }

    pub fn downgrade(&self) -> Weak<Element, AccessKey> {
        Weak {
            data: Arc::downgrade(&self.data),
            _ak: PhantomData::default(),
            _wk: PhantomData::default(),
        }
    }

    fn _get(&self) -> std::sync::RwLockReadGuard<Element> {
        self.data.read().unwrap()
    }

    fn _get_mut(&self) -> std::sync::RwLockWriteGuard<Element> {
        self.data.write().unwrap()
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
