use core::marker::Unsize;
use core::ops::CoerceUnsized;
use std::alloc::{alloc, Layout};
use std::marker::PhantomData;
use std::sync::{self, Arc};

use rc_box::ArcBox;

use crate::arena::cosmos::PKey;

pub struct P<Element, ReadKey = PKey, WriteKey = PKey>
where
    Element: ?Sized,
    ReadKey: ?Sized,
    WriteKey: ?Sized,
{
    data: sync::Arc<Element>,
    _ak: PhantomData<ReadKey>,
    _wk: PhantomData<WriteKey>,
}

impl<Element, ReadKey, WriteKey> Clone for P<Element, ReadKey, WriteKey>
where
    Element: ?Sized,
    ReadKey: ?Sized,
    WriteKey: ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            _ak: PhantomData::default(),
            _wk: PhantomData::default(),
        }
    }
}

impl<Element, ReadKey, WriteKey> PartialEq for P<Element, ReadKey, WriteKey>
where
    Element: ?Sized,
    ReadKey: ?Sized,
    WriteKey: ?Sized,
{
    fn eq(&self, other: &P<Element, ReadKey, WriteKey>) -> bool {
        Arc::ptr_eq(&self.data, &other.data)
    }
}

impl<Element, ReadKey, WriteKey> Eq for P<Element, ReadKey, WriteKey>
where
    Element: ?Sized,
    ReadKey: ?Sized,
    WriteKey: ?Sized,
{
}

impl<Element, ReadKey, WriteKey> std::hash::Hash for P<Element, ReadKey, WriteKey>
where
    Element: ?Sized,
    ReadKey: ?Sized,
    WriteKey: ?Sized,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        sync::Arc::as_ptr(&self.data).hash(state)
    }
}

pub struct Weak<Element, ReadKey = PKey, WriteKey = PKey>
where
    Element: ?Sized,
    ReadKey: ?Sized,
    WriteKey: ?Sized,
{
    data: sync::Weak<Element>,
    _ak: PhantomData<ReadKey>,
    _wk: PhantomData<WriteKey>,
}

impl<Element, ReadKey, WriteKey> Clone for Weak<Element, ReadKey, WriteKey>
where
    Element: ?Sized,
    ReadKey: ?Sized,
    WriteKey: ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            _ak: PhantomData::default(),
            _wk: PhantomData::default(),
        }
    }
}

impl<Element, ReadKey, WriteKey> PartialEq for Weak<Element, ReadKey, WriteKey>
where
    Element: ?Sized,
    ReadKey: ?Sized,
    WriteKey: ?Sized,
{
    fn eq(&self, other: &Weak<Element, ReadKey, WriteKey>) -> bool {
        sync::Weak::ptr_eq(&self.data, &other.data)
    }
}

impl<Element, ReadKey, WriteKey> Eq for Weak<Element, ReadKey, WriteKey>
where
    Element: ?Sized,
    ReadKey: ?Sized,
    WriteKey: ?Sized,
{
}

impl<Element, ReadKey, WriteKey> std::hash::Hash for Weak<Element, ReadKey, WriteKey>
where
    Element: ?Sized,
    ReadKey: ?Sized,
    WriteKey: ?Sized,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        sync::Weak::as_ptr(&self.data).hash(state)
    }
}

impl<Element, ReadKey, WriteKey> P<Element, ReadKey, WriteKey>
where
    ReadKey: ?Sized,
    WriteKey: ?Sized,
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

impl<Element, ReadKey, WriteKey> P<Element, ReadKey, WriteKey>
where
    Element: ?Sized,
    ReadKey: ?Sized,
    WriteKey: ?Sized,
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

impl<Element, ReadKey, WriteKey> From<Box<Element>> for P<Element, ReadKey, WriteKey>
where
    Element: ?Sized,
    ReadKey: ?Sized,
    WriteKey: ?Sized,
{
    #[inline]
    fn from(b: Box<Element>) -> Self {
        Self {
            data: Arc::from(b),
            _ak: PhantomData::default(),
            _wk: PhantomData::default(),
        }
    }
}

impl<Element, ReadKey, WriteKey> From<Arc<Element>> for P<Element, ReadKey, WriteKey>
where
    Element: ?Sized,
    ReadKey: ?Sized,
    WriteKey: ?Sized,
{
    #[inline]
    fn from(arc: Arc<Element>) -> Self {
        Self {
            data: arc,
            _ak: PhantomData::default(),
            _wk: PhantomData::default(),
        }
    }
}

impl<Element, ReadKey, WriteKey> From<ArcBox<Element>> for P<Element, ReadKey, WriteKey>
where
    Element: ?Sized,
    ReadKey: ?Sized,
    WriteKey: ?Sized,
{
    #[inline]
    fn from(b: ArcBox<Element>) -> Self {
        Self {
            data: b.into(),
            _ak: PhantomData::default(),
            _wk: PhantomData::default(),
        }
    }
}

impl<Element, ReadKey, WriteKey> P<Element, ReadKey, WriteKey>
where
    Element: ?Sized,
    ReadKey: ?Sized,
    WriteKey: ?Sized,
{
    #[inline]
    pub fn get<'a>(&'a self, _read_guard: &'a ReadGuard<ReadKey>) -> &'a Element {
        self.data.as_ref()
    }

    #[inline]
    pub fn get_const<'a>(&'a self, _write_guard: &'a WriteGuard<WriteKey>) -> &'a Element {
        self.data.as_ref()
    }

    #[inline]
    pub fn get_mut<'a>(
        &'a mut self,
        _write_guard: &'a WriteGuard<WriteKey>,
    ) -> Option<&'a mut Element> {
        Arc::get_mut(&mut self.data)
    }

    #[inline]
    pub unsafe fn get_mut_unchecked<'a>(
        &'a mut self,
        _write_guard: &'a WriteGuard<WriteKey>,
    ) -> &'a mut Element {
        Arc::get_mut_unchecked(&mut self.data)
    }

    #[inline]
    pub fn try_into_box(self, _key: &WriteKey) -> Result<ArcBox<Element>, Self> {
        self.data.try_into().map_err(|arc| Self::from(arc))
    }

    #[inline]
    pub fn downgrade(&self) -> Weak<Element, ReadKey, WriteKey> {
        Weak {
            data: Arc::downgrade(&self.data),
            _ak: PhantomData::default(),
            _wk: PhantomData::default(),
        }
    }

    #[inline]
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.data)
    }

    #[inline]
    pub fn weak_count(&self) -> usize {
        Arc::weak_count(&self.data)
    }
}

impl<Element, ReadKey, WriteKey> Weak<Element, ReadKey, WriteKey>
where
    ReadKey: ?Sized,
    WriteKey: ?Sized,
{
    pub fn new() -> Self {
        Self {
            data: sync::Weak::new(),
            _ak: PhantomData::default(),
            _wk: PhantomData::default(),
        }
    }
}

impl<Element, ReadKey, WriteKey> Weak<Element, ReadKey, WriteKey>
where
    Element: ?Sized,
    ReadKey: ?Sized,
    WriteKey: ?Sized,
{
    #[inline]
    pub fn upgrade(self) -> Option<P<Element, ReadKey, WriteKey>> {
        self.data.upgrade().map(|arc| arc.into())
    }

    #[inline]
    pub fn strong_count(&self) -> usize {
        sync::Weak::strong_count(&self.data)
    }

    #[inline]
    pub fn weak_count(&self) -> usize {
        sync::Weak::weak_count(&self.data)
    }
}

// P<_MobBlock<Bio>> -> P<_MobBlock<dyn Mob>>
impl<T: ?Sized + Unsize<U>, U: ?Sized, ReadKey, WriteKey> CoerceUnsized<P<U, ReadKey, WriteKey>>
    for P<T, ReadKey, WriteKey>
{
}

// impl<T: ?Sized + Unsize<U>, U: ?Sized, ReadKey, WriteKey> DispatchFromDyn<P<U, ReadKey, WriteKey>> for P<T, ReadKey, WriteKey> {}
impl<T: ?Sized + Unsize<U>, U: ?Sized, ReadKey, WriteKey> CoerceUnsized<Weak<U, ReadKey, WriteKey>>
    for Weak<T, ReadKey, WriteKey>
{
}
// impl<T: ?Sized + Unsize<U>, U: ?Sized, ReadKey, WriteKey> DispatchFromDyn<Weak<U, ReadKey, WriteKey>> for Weak<T, ReadKey, WriteKey> {}

// ReadGuard & WriteGuard can be explicitly drop()-ed, ensuring references' lifetime obtained from it shorter than itself.
// while ref obtained with just P + Key may live longer, causing copying ref and use it in next tick possible.
// but ReadGuard & WriteGuard can still be moved in order to extend its lifetime, so pass fn to with() is recommended.
// Pin<ReadGuard> / Pin<WriteGuard> will not solve anything, as the Pin itself can be moved to extend its lifetime.
// NEVER pass ReadKey / WriteKey to code you don't trust, the ref of key can be copied & reused, then ruin everything.
pub struct ReadGuard<'a, ReadKey: ?Sized>(PhantomData<&'a ReadKey>);
pub struct WriteGuard<'a, WriteKey: ?Sized>(PhantomData<&'a WriteKey>);

impl<'a, ReadKey: ?Sized> ReadGuard<'a, ReadKey> {
    // unsafe, you should drop() it manually to terminate the lifetime of the references it returned
    #[inline]
    pub unsafe fn new(_key: &'a ReadKey) -> Self {
        Self(PhantomData::default())
    }

    #[inline]
    pub fn with<T, F: FnOnce(&Self) -> T>(key: &'a ReadKey, f: F) -> T {
        let guard = unsafe { Self::new(key) };
        let ret = f(&guard);
        drop(guard); // useless, but a guarantee that it is not moved away(to extend its lifetime)
        ret
    }
}

impl<'a, WriteKey: ?Sized> WriteGuard<'a, WriteKey> {
    #[inline]
    pub unsafe fn new(_key: &'a WriteKey) -> Self {
        Self(PhantomData::default())
    }

    #[inline]
    pub fn with<T, F: FnOnce(&Self) -> T>(key: &'a WriteKey, f: F) -> T {
        let guard = unsafe { Self::new(key) };
        let ret = f(&guard);
        drop(guard); // useless, but a guarantee that it is not moved away(to extend its lifetime)
        ret
    }
}
