use core::marker::Unsize;
use core::ops::{CoerceUnsized, DispatchFromDyn};
use std::marker::PhantomData;
use std::sync::{self, Arc};
use std::any::Any;
use std::intrinsics::{likely, unlikely};

use crate::arena::cosmos::{MobBlock, PKey, _MobBlock};
use crate::arena::defs::CrdI;
use crate::arena::mob::Mob;

pub struct Weak<Element, AccessKey = PKey>
where
    Element: ?Sized,
    AccessKey: ?Sized,
{
    data: sync::Weak<Element>,
    _key: PhantomData<AccessKey>,
}

impl<Element, AccessKey> Clone for Weak<Element, AccessKey>
where
    Element: ?Sized,
    AccessKey: ?Sized,
{
    fn clone(&self) -> Self {
        Self {
            data: self.data.clone(),
            _key: PhantomData::default(),
        }
    }
}

impl<Element, AccessKey> PartialEq for Weak<Element, AccessKey>
where
    Element: ?Sized,
    AccessKey: ?Sized,
{
    fn eq(&self, other: &Weak<Element, AccessKey>) -> bool {
        sync::Weak::ptr_eq(&self.data, &other.data)
    }
}

impl<Element, AccessKey> Eq for Weak<Element, AccessKey>
where
    Element: ?Sized,
    AccessKey: ?Sized,
{
}

impl<Element, AccessKey> std::hash::Hash for Weak<Element, AccessKey>
where
    Element: ?Sized,
    AccessKey: ?Sized,
{
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        sync::Weak::as_ptr(&self.data).hash(state)
    }
}

impl<Element, AccessKey> Weak<Element, AccessKey>
where
    AccessKey: ?Sized,
{
    pub fn new() -> Self {
        sync::Weak::new().into()
    }
}

impl<Element, AccessKey> From<sync::Weak<Element>> for Weak<Element, AccessKey>
where
    Element: ?Sized,
    AccessKey: ?Sized,
{
    fn from(data: sync::Weak<Element>) -> Self {
        Self {
            data,
            _key: PhantomData::default(),
        }
    }
}

impl<Element, AccessKey> From<&Arc<Element>> for Weak<Element, AccessKey>
where
    Element: ?Sized,
    AccessKey: ?Sized,
{
    fn from(data: &Arc<Element>) -> Self {
        Arc::downgrade(data).into()
    }
}

impl<Element, AccessKey> Weak<Element, AccessKey>
where
    Element: ?Sized,
    AccessKey: ?Sized,
{
    // #[inline]
    // pub fn upgrade(self, _key: &AccessKey) -> Option<Arc<Element>> {
    //     self.data.upgrade()
    // }

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
// NEVER pass AccessKey to code you don't trust, the ref of key can be copied & reused, then ruin everything.
pub struct ReadGuard<AccessKey: ?Sized>(PhantomData<AccessKey>);
pub struct WriteGuard<AccessKey: ?Sized>(PhantomData<AccessKey>);

impl<AccessKey: ?Sized> ReadGuard<AccessKey> {
    // unsafe, you should drop() it manually to terminate the lifetime of the references it returned
    #[inline]
    pub unsafe fn new(_key: &AccessKey) -> Self {
        Self(PhantomData::default())
    }

    #[inline]
    pub fn with<T, F: FnOnce(&Self) -> T>(key: &AccessKey, f: F) -> T {
        let guard = unsafe { Self::new(key) };
        let ret = f(&guard);
        drop(guard); // useless, but a guarantee that it is not moved away(to extend its lifetime)
        ret
    }

    #[inline]
    pub fn wrap<'g, M: ?Sized>(&'g self, p: Arc<_MobBlock<M>>) -> MobRef<'g, M, AccessKey> {
        MobRef(p, PhantomData::default())
    }

    #[inline]
    pub fn wrap_weak<'g, M: ?Sized>(
        &'g self,
        weak: Weak<_MobBlock<M>, AccessKey>,
    ) -> Option<MobRef<'g, M, AccessKey>> {
        let p = weak.data.upgrade()?;
        if unlikely(!p.on_plate()) {
            return None;
        }
        Some(self.wrap(p))
    }

    #[inline]
    pub unsafe fn wrap_weak_unchecked<'g, M: ?Sized>(
        &'g self,
        weak: Weak<_MobBlock<M>, AccessKey>,
    ) -> Option<MobRef<'g, M, AccessKey>> {
        Some(self.wrap(weak.data.upgrade().unwrap_unchecked()))
    }
}

impl<AccessKey: ?Sized> WriteGuard<AccessKey> {
    #[inline]
    pub unsafe fn new(_key: &AccessKey) -> Self {
        Self(PhantomData::default())
    }

    #[inline]
    pub fn with<T, F: FnOnce(&Self) -> T>(key: &AccessKey, f: F) -> T {
        let guard = unsafe { Self::new(key) };
        let ret = f(&guard);
        drop(guard); // useless, but a guarantee that it is not moved away(to extend its lifetime)
        ret
    }

    #[inline]
    pub fn wrap<'g, M: ?Sized>(&'g self, p: Arc<_MobBlock<M>>) -> MobRef<'g, M, AccessKey> {
        MobRef(p, PhantomData::default())
    }

    #[inline]
    pub fn wrap_weak<'g, M: ?Sized>(
        &'g self,
        weak: Weak<_MobBlock<M>, AccessKey>,
    ) -> Option<MobRef<'g, M, AccessKey>> {
        let p = weak.data.upgrade()?;
        if unlikely(!p.on_plate()) {
            return None;
        }
        Some(self.wrap(p))
    }

    #[inline]
    pub unsafe fn wrap_weak_unchecked<'g, M: ?Sized>(
        &'g self,
        weak: Weak<_MobBlock<M>, AccessKey>,
    ) -> Option<MobRef<'g, M, AccessKey>> {
        Some(self.wrap(weak.data.upgrade().unwrap_unchecked()))
    }

    #[inline]
    pub unsafe fn wrap_mut<'g, M: ?Sized>(
        &'g self,
        p: Arc<_MobBlock<M>>,
    ) -> MobRefMut<'g, M, AccessKey> {
        MobRefMut(p, PhantomData::default())
    }

    #[inline]
    pub unsafe fn wrap_weak_mut<'g, M: ?Sized>(
        &'g self,
        weak: Weak<_MobBlock<M>, AccessKey>,
    ) -> Option<MobRefMut<'g, M, AccessKey>> {
        Some(MobRefMut(weak.data.upgrade()?, PhantomData::default()))
    }
}

#[repr(transparent)]
pub struct MobRef<'g, M: ?Sized, AccessKey: ?Sized = PKey>(
    Arc<_MobBlock<M>>,
    PhantomData<&'g ReadGuard<AccessKey>>,
);

#[repr(transparent)]
pub struct MobRefMut<'g, M: ?Sized, AccessKey: ?Sized = PKey>(
    Arc<_MobBlock<M>>,
    PhantomData<&'g WriteGuard<AccessKey>>,
);

#[repr(transparent)]
pub struct MobBox<M: ?Sized, AccessKey: ?Sized = PKey>(
    Arc<_MobBlock<M>>,
    PhantomData<AccessKey>,
);

impl<'g, M: ?Sized, AccessKey: ?Sized> MobRef<'g, M, AccessKey> {
    #[inline]
    pub fn at(&self) -> CrdI {
        self.0.as_ref().at
    }

    #[inline]
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.0)
    }

    #[inline]
    pub fn weak_count(&self) -> usize {
        Arc::weak_count(&self.0)
    }
}

impl<'g, M: ?Sized + Mob, AccessKey: ?Sized> MobRefMut<'g, M, AccessKey> {
    #[inline]
    pub fn at(&self) -> CrdI {
        self.0.as_ref().at
    }

    #[inline]
    pub fn get_inner(&self, _key: &AccessKey) -> Arc<_MobBlock<M>> {
        self.0.clone()
    }

    #[inline]
    pub fn into_inner(self, _key: &AccessKey) -> Arc<_MobBlock<M>> {
        self.0
    }

    #[inline]
    pub fn strong_count(&self) -> usize {
        Arc::strong_count(&self.0)
    }

    #[inline]
    pub fn weak_count(&self) -> usize {
        Arc::weak_count(&self.0)
    }
}

impl<M: ?Sized + Mob, AccessKey: ?Sized> MobBox<M, AccessKey> {

    #[inline]
    pub fn new(p: Arc<_MobBlock<M>>) -> Option<Self> {
        if unlikely(Arc::strong_count(&p) != 1 || p.on_plate()) {
            return None;
        }
        Some(unsafe { Self::new_unchecked(p) })
    }

    #[inline]
    pub unsafe fn new_unchecked(p: Arc<_MobBlock<M>>) -> Self {
        MobBox(p, PhantomData::default())
    }

    #[inline]
    pub fn into_inner(self, _key: &AccessKey) -> Arc<_MobBlock<M>> {
        self.0
    }

    #[inline]
    pub fn weak_count(&self) -> usize {
        Arc::weak_count(&self.0)
    }
}

impl<'g, M: Mob + Unsize<dyn Mob> + ?Sized, AccessKey: ?Sized> MobRef<'g, M, AccessKey> {
    #[inline]
    pub fn downgrade(&self) -> Weak<MobBlock, AccessKey> {
        Weak::<_MobBlock<M>, _>::from(&self.0)
    }
}

impl<'g, M: Mob + Unsize<dyn Mob> + ?Sized, AccessKey: ?Sized> MobRefMut<'g, M, AccessKey> {
    #[inline]
    pub fn downgrade(&self) -> Weak<MobBlock, AccessKey> {
        Weak::<_MobBlock<M>, _>::from(&self.0)
    }
}

impl<M: Mob + Unsize<dyn Mob> + ?Sized, AccessKey: ?Sized> MobBox<M, AccessKey> {
    #[inline]
    pub fn downgrade(&self) -> Weak<MobBlock, AccessKey> {
        Weak::<_MobBlock<M>, _>::from(&self.0)
    }
}

impl<'g, AccessKey: ?Sized> MobRef<'g, dyn Mob, AccessKey> {
    #[inline]
    pub fn downcast<T: Mob>(self) -> Result<MobRef<'g, T, AccessKey>, Self> {
        if likely((*self).is::<T>()) {
            Ok(MobRef(unsafe {
                Arc::from_raw(Arc::into_raw(self.0) as _)
            }, PhantomData::default()))
        } else {
            Err(self)
        }
    }
}

impl<'g, AccessKey: ?Sized> MobRefMut<'g, dyn Mob, AccessKey> {
    #[inline]
    pub fn downcast<T: Mob>(self) -> Result<MobRefMut<'g, T, AccessKey>, Self> {
        if likely((*self).is::<T>()) {
            Ok(MobRefMut(unsafe {
                Arc::from_raw(Arc::into_raw(self.0) as _)
            }, PhantomData::default()))
        } else {
            Err(self)
        }
    }
}

impl<AccessKey: ?Sized> MobBox<dyn Mob, AccessKey> {
    #[inline]
    pub fn downcast<T: Mob>(self) -> Result<MobBox<T, AccessKey>, Self> {
        if likely(self.mob.is::<T>()) {
            Ok(MobBox(unsafe {
                Arc::from_raw(Arc::into_raw(self.0) as _)
            }, PhantomData::default()))
        } else {
            Err(self)
        }
    }
}

impl<'g, M: ?Sized, AccessKey: ?Sized> std::ops::Deref for MobRef<'g, M, AccessKey> {
    type Target = M;
    #[inline]
    fn deref(&self) -> &M {
        &self.0.as_ref().mob
    }
}

impl<'g, M: ?Sized, AccessKey: ?Sized> std::ops::Deref for MobRefMut<'g, M, AccessKey> {
    type Target = M;
    #[inline]
    fn deref(&self) -> &M {
        &self.0.as_ref().mob
    }
}

impl<'g, M: ?Sized, AccessKey: ?Sized> std::ops::DerefMut for MobRefMut<'g, M, AccessKey> {
    #[inline]
    fn deref_mut(&mut self) -> &mut M {
        &mut unsafe { Arc::get_mut_unchecked(&mut self.0) }.mob
    }
}

impl<M: ?Sized, AccessKey: ?Sized> std::ops::Deref for MobBox<M, AccessKey> {
    type Target = _MobBlock<M>;
    #[inline]
    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

impl<M: ?Sized, AccessKey: ?Sized> std::ops::DerefMut for MobBox<M, AccessKey> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        unsafe { Arc::get_mut_unchecked(&mut self.0) }
    }
}

// CoerceUnsized
// Weak<_MobBlock<Bio>> -> Weak<_MobBlock<dyn Mob>>
impl<T: ?Sized + Unsize<U>, U: ?Sized, AccessKey: ?Sized> CoerceUnsized<Weak<U, AccessKey>>
    for Weak<T, AccessKey>
{
}

impl<'g, T: ?Sized + Unsize<U>, U: ?Sized, AccessKey: ?Sized>
    CoerceUnsized<MobRef<'g, U, AccessKey>> for MobRef<'g, T, AccessKey>
{
}

impl<'g, T: ?Sized + Unsize<U>, U: ?Sized, AccessKey: ?Sized>
    CoerceUnsized<MobRefMut<'g, U, AccessKey>> for MobRefMut<'g, T, AccessKey>
{
}

impl<T: ?Sized + Unsize<U>, U: ?Sized, AccessKey: ?Sized>
    CoerceUnsized<MobBox<U, AccessKey>> for MobBox<T, AccessKey>
{
}

// DispatchFromDyn
// fn hear(self: MobRef<Self>, ...);
// let mob: MobRef<dyn Mob> = ...;
// mob.hear(...);
// impl<T: ?Sized + Unsize<U>, U: ?Sized, AccessKey: ?Sized> DispatchFromDyn<Weak<U, AccessKey>> for Weak<T, AccessKey> {}
impl<'g, T: ?Sized + Unsize<U>, U: ?Sized, AccessKey: ?Sized>
    DispatchFromDyn<MobRef<'g, U, AccessKey>> for MobRef<'g, T, AccessKey>
{
}

impl<'g, T: ?Sized + Unsize<U>, U: ?Sized, AccessKey: ?Sized>
    DispatchFromDyn<MobRefMut<'g, U, AccessKey>> for MobRefMut<'g, T, AccessKey>
{
}

impl<T: ?Sized + Unsize<U>, U: ?Sized, AccessKey: ?Sized>
    DispatchFromDyn<MobBox<U, AccessKey>> for MobBox<T, AccessKey>
{
}

// prevent cloning
impl<AccessKey: ?Sized> !Clone for ReadGuard<AccessKey> {}

impl<AccessKey: ?Sized> !Clone for WriteGuard<AccessKey> {}

impl<'g, M: ?Sized, AccessKey: ?Sized> !Clone for MobRef<'g, M, AccessKey> {}

impl<'g, M: ?Sized, AccessKey: ?Sized> !Clone for MobRefMut<'g, M, AccessKey> {}

impl<M: ?Sized, AccessKey: ?Sized> !Clone for MobBox<M, AccessKey> {}
