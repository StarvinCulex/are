use core::marker::Unsize;
use core::ops::{CoerceUnsized, DispatchFromDyn};
use std::collections::{HashMap, HashSet};
use std::intrinsics::{likely, unlikely};
use std::marker::PhantomData;
use std::ptr::NonNull;
use std::sync::{self, Arc, RwLock};

use crate::arena::cosmos::{MobBlock, PKey, _MobBlock};
use crate::arena::defs::CrdI;
use crate::arena::mob::Mob;
use crate::observe::logger::Logger;

#[repr(transparent)]
pub struct CheapMobArc<M: ?Sized>(NonNull<_MobBlock<M>>);
impl<M: ?Sized> CheapMobArc<M> {
    #[inline]
    pub unsafe fn from_arc(arc: Arc<_MobBlock<M>>) -> Self {
        Self(unsafe { NonNull::new_unchecked(Arc::into_raw(arc) as *mut _) })
    }

    #[inline]
    pub unsafe fn from_arc_ref(arc: &Arc<_MobBlock<M>>) -> Self {
        Self(unsafe { NonNull::new_unchecked(Arc::as_ref(arc) as *const _ as *mut _) })
    }

    #[inline]
    pub fn as_ptr(self) -> *mut _MobBlock<M> {
        self.0.as_ptr()
    }

    #[inline]
    pub unsafe fn as_ref(&self) -> &_MobBlock<M> {
        unsafe { self.0.as_ref() }
    }

    #[inline]
    pub unsafe fn as_mut(&mut self) -> &mut _MobBlock<M> {
        unsafe { self.0.as_mut() }
    }

    #[inline]
    pub unsafe fn into_arc(self) -> Arc<_MobBlock<M>> {
        unsafe { Arc::from_raw(self.0.as_ptr()) }
    }

    #[inline]
    pub unsafe fn strong_count(self) -> usize {
        let arc = unsafe { self.into_arc() };
        let cnt = Arc::strong_count(&arc);
        std::mem::forget(arc);
        cnt
    }

    #[inline]
    pub unsafe fn weak_count(self) -> usize {
        let arc = unsafe { self.into_arc() };
        let cnt = Arc::weak_count(&arc);
        std::mem::forget(arc);
        cnt
    }
}

impl<M: Mob + ?Sized + Unsize<dyn Mob>> CheapMobArc<M> {
    #[inline]
    pub unsafe fn make_weak<AccessKey: ?Sized>(self) -> Weak<MobBlock, AccessKey> {
        let mob: Arc<MobBlock> = unsafe { self.into_arc() };
        let weak = (&mob).into();
        std::mem::forget(mob);
        weak
    }
}

impl<M: ?Sized> Clone for CheapMobArc<M> {
    fn clone(&self) -> Self {
        Self(self.0)
    }
}

impl<M: ?Sized> Copy for CheapMobArc<M> {}

unsafe impl<M: ?Sized> Send for CheapMobArc<M> {}

unsafe impl<M: ?Sized> Sync for CheapMobArc<M> {}

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
            _key: PhantomData,
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
            _key: PhantomData,
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

    #[inline]
    pub fn as_ptr(&self) -> *const Element {
        self.data.as_ptr()
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
        Self(PhantomData)
    }

    #[inline]
    pub fn with<T, F: FnOnce(&Self) -> T>(key: &AccessKey, f: F) -> T {
        let guard = unsafe { Self::new(key) };
        let ret = f(&guard);
        drop(guard); // useless, but a guarantee that it is not moved away(to extend its lifetime)
        ret
    }

    #[inline]
    pub unsafe fn wrap_cheap<'g, M: ?Sized>(
        &'g self,
        p: CheapMobArc<M>,
    ) -> MobRef<'g, M, AccessKey> {
        MobRef(p, PhantomData)
    }

    #[inline]
    pub fn wrap<'g, M: ?Sized>(&'g self, p: &Arc<_MobBlock<M>>) -> MobRef<'g, M, AccessKey> {
        unsafe { self.wrap_cheap(CheapMobArc::from_arc_ref(p)) }
    }

    #[inline]
    pub fn wrap_weak<'g, M: ?Sized>(
        &'g self,
        weak: &Weak<_MobBlock<M>, AccessKey>,
    ) -> Option<MobRef<'g, M, AccessKey>> {
        let p = weak.data.upgrade()?;
        if unlikely(!p.on_plate()) {
            return None;
        }
        Some(self.wrap(&p))
    }

    #[inline]
    pub unsafe fn wrap_weak_unchecked<'g, M: ?Sized>(
        &'g self,
        weak: &Weak<_MobBlock<M>, AccessKey>,
    ) -> Option<MobRef<'g, M, AccessKey>> {
        Some(self.wrap(&unsafe { weak.data.upgrade().unwrap_unchecked() }))
    }
}

impl<AccessKey: ?Sized> WriteGuard<AccessKey> {
    #[inline]
    pub unsafe fn new(_key: &AccessKey) -> Self {
        Self(PhantomData)
    }

    #[inline]
    pub fn read<'g>(&'g self) -> &'g ReadGuard<AccessKey> {
        &ReadGuard::<AccessKey>(PhantomData)
    }

    #[inline]
    pub fn with<T, F: FnOnce(&Self) -> T>(key: &AccessKey, f: F) -> T {
        let guard = unsafe { Self::new(key) };
        let ret = f(&guard);
        drop(guard); // useless, but a guarantee that it is not moved away(to extend its lifetime)
        ret
    }

    #[inline]
    pub unsafe fn wrap_cheap_mut<'g, M: ?Sized>(
        &'g self,
        p: CheapMobArc<M>,
    ) -> MobRefMut<'g, M, AccessKey> {
        MobRefMut(p, PhantomData)
    }

    #[inline]
    pub unsafe fn wrap_mut<'g, M: ?Sized>(
        &'g self,
        p: &Arc<_MobBlock<M>>,
    ) -> MobRefMut<'g, M, AccessKey> {
        unsafe { self.wrap_cheap_mut(CheapMobArc::from_arc_ref(p)) }
    }

    #[inline]
    pub unsafe fn wrap_weak_mut<'g, M: Mob + ?Sized>(
        &'g self,
        weak: &Weak<_MobBlock<M>, AccessKey>,
    ) -> Option<MobRefMut<'g, M, AccessKey>> {
        Some(unsafe { self.wrap_mut(&weak.data.upgrade()?) })
    }
}

#[repr(transparent)]
pub struct MobRef<'g, M: ?Sized, AccessKey: ?Sized = PKey>(
    CheapMobArc<M>,
    PhantomData<&'g ReadGuard<AccessKey>>,
);

#[repr(transparent)]
pub struct MobRefMut<'g, M: ?Sized, AccessKey: ?Sized = PKey>(
    CheapMobArc<M>,
    PhantomData<&'g WriteGuard<AccessKey>>,
);

#[repr(transparent)]
pub struct MobBox<M: ?Sized, AccessKey: ?Sized = PKey>(Arc<_MobBlock<M>>, PhantomData<AccessKey>);

impl<'g, M: ?Sized, AccessKey: ?Sized> MobRef<'g, M, AccessKey> {
    #[inline]
    pub fn at(&self) -> CrdI {
        unsafe { self.0.as_ref() }.at
    }

    #[inline]
    pub fn log(&self) -> &Logger {
        &unsafe { self.0.as_ref() }.log
    }

    #[inline]
    pub fn get_inner(&self, _key: &AccessKey) -> CheapMobArc<M> {
        self.0
    }

    #[inline]
    pub fn strong_count(&self) -> usize {
        unsafe { self.0.strong_count() }
    }

    #[inline]
    pub fn weak_count(&self) -> usize {
        unsafe { self.0.weak_count() }
    }

    #[inline]
    pub fn as_ptr(self) -> *const _MobBlock<M> {
        self.0.as_ptr()
    }
}

impl<'g, M: ?Sized, AccessKey: ?Sized> MobRefMut<'g, M, AccessKey> {
    #[inline]
    pub fn at(&self) -> CrdI {
        unsafe { self.0.as_ref() }.at
    }

    #[inline]
    pub fn log(&self) -> &Logger {
        &unsafe { self.0.as_ref() }.log
    }

    #[inline]
    pub fn log_mut(&mut self) -> &mut Logger {
        &mut unsafe { self.0.as_mut() }.log
    }

    #[inline]
    pub fn get_inner(&self, _key: &AccessKey) -> CheapMobArc<M> {
        self.0
    }

    #[inline]
    pub fn strong_count(&self) -> usize {
        unsafe { self.0.strong_count() }
    }

    #[inline]
    pub fn weak_count(&self) -> usize {
        unsafe { self.0.weak_count() }
    }

    #[inline]
    pub fn as_ptr(self) -> *const _MobBlock<M> {
        self.0.as_ptr()
    }
}

impl<M: ?Sized, AccessKey: ?Sized> MobBox<M, AccessKey> {
    #[inline]
    pub fn new(p: Arc<_MobBlock<M>>) -> Option<Self> {
        if unlikely(Arc::strong_count(&p) != 1 || p.on_plate()) {
            return None;
        }
        Some(unsafe { Self::new_unchecked(p) })
    }

    #[inline]
    pub unsafe fn new_unchecked(p: Arc<_MobBlock<M>>) -> Self {
        MobBox(p, PhantomData)
    }

    #[inline]
    pub fn into_inner(self, _key: &AccessKey) -> Arc<_MobBlock<M>> {
        self.0
    }

    #[inline]
    pub fn weak_count(&self) -> usize {
        Arc::weak_count(&self.0)
    }

    #[inline]
    pub fn as_ptr(self) -> *const _MobBlock<M> {
        Arc::as_ptr(&self.0)
    }
}

impl<'g, M: Mob + Unsize<dyn Mob> + ?Sized, AccessKey: ?Sized> MobRef<'g, M, AccessKey> {
    #[inline]
    pub fn downgrade(&self) -> Weak<MobBlock, AccessKey> {
        unsafe { self.0.make_weak() }
    }
}

impl<'g, M: Mob + Unsize<dyn Mob> + ?Sized, AccessKey: ?Sized> MobRefMut<'g, M, AccessKey> {
    #[inline]
    pub fn downgrade(&self) -> Weak<MobBlock, AccessKey> {
        unsafe { self.0.make_weak() }
    }

    #[inline]
    pub fn get_const(&self) -> MobRef<M, AccessKey> {
        MobRef(CheapMobArc(self.0 .0), PhantomData)
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
            Ok(MobRef(CheapMobArc(self.0 .0.cast()), PhantomData))
        } else {
            Err(self)
        }
    }
}

impl<'g, AccessKey: ?Sized> MobRefMut<'g, dyn Mob, AccessKey> {
    #[inline]
    pub fn downcast<T: Mob>(self) -> Result<MobRefMut<'g, T, AccessKey>, Self> {
        if likely((*self).is::<T>()) {
            Ok(MobRefMut(CheapMobArc(self.0 .0.cast()), PhantomData))
        } else {
            Err(self)
        }
    }
}

impl<AccessKey: ?Sized> MobBox<dyn Mob, AccessKey> {
    #[inline]
    pub fn downcast<T: Mob>(self) -> Result<MobBox<T, AccessKey>, Self> {
        if likely(self.mob.is::<T>()) {
            Ok(MobBox(
                unsafe { Arc::from_raw(Arc::into_raw(self.0) as _) },
                PhantomData,
            ))
        } else {
            Err(self)
        }
    }
}

impl<'g, M: ?Sized, AccessKey: ?Sized> std::ops::Deref for MobRef<'g, M, AccessKey> {
    type Target = M;
    #[inline]
    fn deref(&self) -> &M {
        &unsafe { self.0.as_ref() }.mob
    }
}

impl<'g, M: ?Sized, AccessKey: ?Sized> std::ops::Deref for MobRefMut<'g, M, AccessKey> {
    type Target = M;
    #[inline]
    fn deref(&self) -> &M {
        &unsafe { self.0.as_ref() }.mob
    }
}

impl<'g, M: ?Sized, AccessKey: ?Sized> std::ops::DerefMut for MobRefMut<'g, M, AccessKey> {
    #[inline]
    fn deref_mut(&mut self) -> &mut M {
        &mut unsafe { self.0.as_mut() }.mob
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
impl<T: ?Sized + Unsize<U>, U: ?Sized> CoerceUnsized<CheapMobArc<U>> for CheapMobArc<T> {}

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

impl<T: ?Sized + Unsize<U>, U: ?Sized, AccessKey: ?Sized> CoerceUnsized<MobBox<U, AccessKey>>
    for MobBox<T, AccessKey>
{
}

// DispatchFromDyn
// fn hear(self: MobRef<Self>, ...);
// let mob: MobRef<dyn Mob> = ...;
// mob.hear(...);
// impl<T: ?Sized + Unsize<U>, U: ?Sized, AccessKey: ?Sized> DispatchFromDyn<Weak<U, AccessKey>> for Weak<T, AccessKey> {}
impl<'g, T: ?Sized + Unsize<U>, U: ?Sized> DispatchFromDyn<CheapMobArc<U>> for CheapMobArc<T> {}

impl<'g, T: ?Sized + Unsize<U>, U: ?Sized, AccessKey: ?Sized>
    DispatchFromDyn<MobRef<'g, U, AccessKey>> for MobRef<'g, T, AccessKey>
{
}

impl<'g, T: ?Sized + Unsize<U>, U: ?Sized, AccessKey: ?Sized>
    DispatchFromDyn<MobRefMut<'g, U, AccessKey>> for MobRefMut<'g, T, AccessKey>
{
}

impl<T: ?Sized + Unsize<U>, U: ?Sized, AccessKey: ?Sized> DispatchFromDyn<MobBox<U, AccessKey>>
    for MobBox<T, AccessKey>
{
}

// prevent cloning
impl<AccessKey: ?Sized> !Clone for ReadGuard<AccessKey> {}

impl<AccessKey: ?Sized> !Clone for WriteGuard<AccessKey> {}

impl<'g, M: ?Sized, AccessKey: ?Sized> Clone for MobRef<'g, M, AccessKey> {
    fn clone(&self) -> Self {
        MobRef(self.0, PhantomData)
    }
}

impl<'g, M: ?Sized, AccessKey: ?Sized> Copy for MobRef<'g, M, AccessKey> {}

impl<'g, M: ?Sized, AccessKey: ?Sized> !Clone for MobRefMut<'g, M, AccessKey> {}

impl<M: ?Sized, AccessKey: ?Sized> !Clone for MobBox<M, AccessKey> {}
