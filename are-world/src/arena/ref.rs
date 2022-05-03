use core::marker::Unsize;
use core::ops::{CoerceUnsized, DispatchFromDyn};
use std::marker::PhantomData;
use std::sync::{self, Arc};

use rc_box::ArcBox;

use crate::arena::cosmos::{PKey, MobBlock, _MobBlock};
use crate::arena::mob::Mob;
use crate::arena::defs::CrdI;

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
    pub fn upgrade(self, _read_key: &ReadKey) -> Option<P<Element, ReadKey, WriteKey>> {
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

// ReadGuard & WriteGuard can be explicitly drop()-ed, ensuring references' lifetime obtained from it shorter than itself.
// while ref obtained with just P + Key may live longer, causing copying ref and use it in next tick possible.
// but ReadGuard & WriteGuard can still be moved in order to extend its lifetime, so pass fn to with() is recommended.
// Pin<ReadGuard> / Pin<WriteGuard> will not solve anything, as the Pin itself can be moved to extend its lifetime.
// NEVER pass ReadKey / WriteKey to code you don't trust, the ref of key can be copied & reused, then ruin everything.
pub struct ReadGuard<ReadKey: ?Sized>(PhantomData<ReadKey>);
pub struct WriteGuard<WriteKey: ?Sized>(PhantomData<WriteKey>);

impl<ReadKey: ?Sized> ReadGuard<ReadKey> {
    // unsafe, you should drop() it manually to terminate the lifetime of the references it returned
    #[inline]
    pub unsafe fn new(_key: &ReadKey) -> Self {
        Self(PhantomData::default())
    }

    #[inline]
    pub fn with<T, F: FnOnce(&Self) -> T>(key: &ReadKey, f: F) -> T {
        let guard = unsafe { Self::new(key) };
        let ret = f(&guard);
        drop(guard); // useless, but a guarantee that it is not moved away(to extend its lifetime)
        ret
    }

    #[inline]
    pub fn wrap<'g, M: ?Sized, WriteKey: ?Sized>(&'g self, p: P<_MobBlock<M>, ReadKey, WriteKey>) -> MobRef<'g, M, ReadKey, WriteKey> {
        MobRef(p, PhantomData::default())
    }

    #[inline]
    pub fn wrap_weak<'g, M: ?Sized, WriteKey: ?Sized>(&'g self, weak: Weak<_MobBlock<M>, ReadKey, WriteKey>) -> Option<MobRef<'g, M, ReadKey, WriteKey>> {
        Some(MobRef(weak.data.upgrade().unwrap().into(), PhantomData::default()))
    }
}

impl<WriteKey: ?Sized> WriteGuard<WriteKey> {
    #[inline]
    pub unsafe fn new(_key: &WriteKey) -> Self {
        Self(PhantomData::default())
    }

    #[inline]
    pub fn with<T, F: FnOnce(&Self) -> T>(key: &WriteKey, f: F) -> T {
        let guard = unsafe { Self::new(key) };
        let ret = f(&guard);
        drop(guard); // useless, but a guarantee that it is not moved away(to extend its lifetime)
        ret
    }

    #[inline]
    pub fn wrap<'g, M: ?Sized, ReadKey: ?Sized>(&'g self, p: P<_MobBlock<M>, ReadKey, WriteKey>) -> MobRef<'g, M, ReadKey, WriteKey> {
        MobRef(p, PhantomData::default())
    }

    #[inline]
    pub fn wrap_weak<'g, M: ?Sized, ReadKey: ?Sized>(&'g self, weak: Weak<_MobBlock<M>, ReadKey, WriteKey>) -> Option<MobRef<'g, M, ReadKey, WriteKey>> {
        Some(MobRef(weak.data.upgrade().unwrap().into(), PhantomData::default()))
    }

    #[inline]
    pub unsafe fn wrap_mut<'g, M: ?Sized, ReadKey: ?Sized>(&'g self, p: P<_MobBlock<M>, ReadKey, WriteKey>) -> MobRefMut<'g, M, ReadKey, WriteKey> {
        MobRefMut(p, PhantomData::default())
    }

    #[inline]
    pub unsafe fn wrap_weak_mut<'g, M: ?Sized, ReadKey: ?Sized>(&'g self, weak: Weak<_MobBlock<M>, ReadKey, WriteKey>) -> Option<MobRefMut<'g, M, ReadKey, WriteKey>> {
        Some(MobRefMut(weak.data.upgrade().unwrap().into(), PhantomData::default()))
    }
}

pub struct MobRef<'g, M: ?Sized, ReadKey: ?Sized = PKey, WriteKey: ?Sized = PKey>(P<_MobBlock<M>, ReadKey, WriteKey>, PhantomData<&'g ReadGuard<ReadKey>>);
pub struct MobRefMut<'g, M: ?Sized, ReadKey: ?Sized = PKey, WriteKey: ?Sized = PKey>(P<_MobBlock<M>, ReadKey, WriteKey>, PhantomData<&'g WriteGuard<WriteKey>>);

impl<'g, M: ?Sized, ReadKey: ?Sized, WriteKey: ?Sized> MobRef<'g, M, ReadKey, WriteKey> {
    #[inline]
    pub fn at(&self) -> CrdI {
        self.0.data.as_ref().at
    }
}

impl<'g, M: ?Sized + Mob, ReadKey: ?Sized, WriteKey: ?Sized> MobRefMut<'g, M, ReadKey, WriteKey> {
    #[inline]
    pub fn at(&self) -> CrdI {
        self.0.data.as_ref().at
    }

    #[inline]
    pub fn get_inner(&self, _write_key: &WriteKey) -> P<_MobBlock<M>, ReadKey, WriteKey> {
        self.0.clone()
    }
}

impl<'g, M: Mob + 'static, ReadKey: ?Sized, WriteKey: ?Sized> MobRef<'g, M, ReadKey, WriteKey> {
    #[inline]
    pub fn downgrade(&self) -> Weak<MobBlock, ReadKey, WriteKey> {
        self.0.downgrade()
    }
}

impl<'g, M: Mob + 'static, ReadKey: ?Sized, WriteKey: ?Sized> MobRefMut<'g, M, ReadKey, WriteKey> {
    #[inline]
    pub fn downgrade(&self) -> Weak<MobBlock, ReadKey, WriteKey> {
        self.0.downgrade()
    }
}

impl<'g, M: ?Sized, ReadKey: ?Sized, WriteKey: ?Sized> std::ops::Deref for MobRef<'g, M, ReadKey, WriteKey> {
    type Target = M;
    #[inline]
    fn deref(&self) -> &M {
        &self.0.data.as_ref().mob
    }
}

impl<'g, M: ?Sized, ReadKey: ?Sized, WriteKey: ?Sized> std::ops::Deref for MobRefMut<'g, M, ReadKey, WriteKey> {
    type Target = M;
    #[inline]
    fn deref(&self) -> &M {
        &self.0.data.as_ref().mob
    }
}

impl<'g, M: ?Sized, ReadKey: ?Sized, WriteKey: ?Sized> std::ops::DerefMut for MobRefMut<'g, M, ReadKey, WriteKey> {
    #[inline]
    fn deref_mut(&mut self) -> &mut M {
        &mut unsafe { Arc::get_mut_unchecked(&mut self.0.data) }.mob
    }
}

// CoerceUnsized
// P<_MobBlock<Bio>> -> P<_MobBlock<dyn Mob>>
impl<T: ?Sized + Unsize<U>, U: ?Sized, ReadKey: ?Sized, WriteKey: ?Sized> CoerceUnsized<P<U, ReadKey, WriteKey>> for P<T, ReadKey, WriteKey> {}
impl<T: ?Sized + Unsize<U>, U: ?Sized, ReadKey: ?Sized, WriteKey: ?Sized> CoerceUnsized<Weak<U, ReadKey, WriteKey>> for Weak<T, ReadKey, WriteKey> {}
impl<'g, T: ?Sized + Unsize<U>, U: ?Sized, ReadKey: ?Sized, WriteKey: ?Sized> CoerceUnsized<MobRef<'g, U, ReadKey, WriteKey>> for MobRef<'g, T, ReadKey, WriteKey> {}
impl<'g, T: ?Sized + Unsize<U>, U: ?Sized, ReadKey: ?Sized, WriteKey: ?Sized> CoerceUnsized<MobRefMut<'g, U, ReadKey, WriteKey>> for MobRefMut<'g, T, ReadKey, WriteKey> {}

// DispatchFromDyn
// fn hear(self: MobRef<Self>, ...);
// let mob: MobRef<dyn Mob> = ...;
// mob.hear(...);
impl<T: ?Sized + Unsize<U>, U: ?Sized, ReadKey: ?Sized, WriteKey: ?Sized> DispatchFromDyn<P<U, ReadKey, WriteKey>> for P<T, ReadKey, WriteKey> {}
// impl<T: ?Sized + Unsize<U>, U: ?Sized, ReadKey: ?Sized, WriteKey: ?Sized> DispatchFromDyn<Weak<U, ReadKey, WriteKey>> for Weak<T, ReadKey, WriteKey> {}
impl<'g, T: ?Sized + Unsize<U>, U: ?Sized, ReadKey: ?Sized, WriteKey: ?Sized> DispatchFromDyn<MobRef<'g, U, ReadKey, WriteKey>> for MobRef<'g, T, ReadKey, WriteKey> {}
impl<'g, T: ?Sized + Unsize<U>, U: ?Sized, ReadKey: ?Sized, WriteKey: ?Sized> DispatchFromDyn<MobRefMut<'g, U, ReadKey, WriteKey>> for MobRefMut<'g, T, ReadKey, WriteKey> {}

// prevent cloning
impl<ReadKey: ?Sized> !Clone for ReadGuard<ReadKey> {}
impl<WriteKey: ?Sized> !Clone for WriteGuard<WriteKey> {}
impl<'g, M: ?Sized, ReadKey: ?Sized, WriteKey: ?Sized> !Clone for MobRef<'g, M, ReadKey, WriteKey> {}
impl<'g, M: ?Sized, ReadKey: ?Sized, WriteKey: ?Sized> !Clone for MobRefMut<'g, M, ReadKey, WriteKey> {}

