use core::marker::Unsize;
use std::intrinsics::unlikely;
use std::iter::Map;

use crate::meta::gnd::Ground;

//
pub struct Deamon<'c, 'a> {
    pub angelos: &'a mut Angelos<'c>,
    plate: &'c mut Plate,
    bound: CrdI,
}

impl<'c, 'a> Deamon<'c, 'a> {
    pub fn get_ground(&self, p: Crd) -> Result<&Ground, ()> {
        if unlikely(!self.contains_pos(p)) {
            Err(())
        } else {
            Ok(&self.plate[p].ground)
        }
    }
    pub fn get_ground_mut(&mut self, p: Crd) -> Result<&mut Ground, ()> {
        if unlikely(!self.contains_pos(p)) {
            Err(())
        } else {
            Ok(&mut self.plate[p].ground)
        }
    }
    pub fn get_ground_iter(
        &self,
        p: CrdI,
    ) -> Result<
        Map<
            impl Iterator<Item = (Coord<isize>, &Block)>,
            fn((Coord<isize>, &Block)) -> (Crd, &Ground),
        >,
        (),
    > {
        if unlikely(!self.contains(p)) {
            return Err(());
        }
        Ok(self
            .plate
            .area(p)
            .iter()
            .map(|(p, g)| (p.try_into().unwrap(), &g.ground)))
    }
    pub fn get_ground_iter_mut(
        &mut self,
        p: CrdI,
    ) -> Result<
        Map<
            impl Iterator<Item = (Coord<isize>, &mut Block)>,
            fn((Coord<isize>, &mut Block)) -> (Crd, &mut Ground),
        >,
        (),
    > {
        if unlikely(!self.contains(p)) {
            return Err(());
        }
        Ok(self
            .plate
            .area_mut(p)
            .iter()
            .map(|(p, b)| (p.try_into().unwrap(), &mut b.ground)))
    }

    pub fn set<M: Mob + Unsize<dyn Mob> + ?Sized>(
        &mut self,
        mob: MobBox<M>,
    ) -> Result<Weak<MobBlock>, MobBox<M>> {
        let at = mob.at;
        if unlikely(!self.contains(at)) {
            return Err(mob);
        }

        Self::set_plate(self.plate, self.angelos.major, mob)
    }

    pub fn take<'g, M: Mob + ?Sized>(
        &mut self,
        mob: MobRefMut<'g, M>,
    ) -> MobBox<M> {
        // check if the mob.at() is valid
        // no need to check, MobRefMut is trusted
        // if let Some(plate_mob) = self.plate[at.from()].mob.as_ref() {
        //     if &mob_p != plate_mob {
        //         return Err(mob);
        //     }
        // } else {
        //     return Err(mob);
        // }
        Self::take_plate(self.plate, self.angelos.major, mob)
    }

    /// 尝试把[`mob`]移动到[`new_at`]的位置。
    pub fn reset<'g, M: Mob + Unsize<dyn Mob> + ?Sized>(
        &mut self,
        mob: &mut MobRefMut<'g, M>,
        new_at: CrdI,
    ) -> Result<(), ()> {
        if unlikely(!self.contains(new_at)) {
            return Err(());
        }
        Self::reset_plate(self.plate, self.angelos.major, mob, new_at)
    }

    #[inline]
    fn contains(&self, mut p: CrdI) -> bool {
        p = self.angelos.major.normalize_area(p);
        self.bound
            .contains_coord_interval(&p, self.angelos.major.plate_size)
    }

    #[inline]
    fn contains_pos(&self, mut p: Crd) -> bool {
        p = self.angelos.major.normalize_pos(p);
        self.bound.contains(&p)
    }

    #[inline]
    pub fn set_plate<M: Mob + Unsize<dyn Mob> + ?Sized>(
        plate: &mut Plate,
        major: &MajorAngelos,
        mut mob: MobBox<M>,
    ) -> Result<Weak<MobBlock>, MobBox<M>> {
        // check if the plate is empty
        let at = mob.at;
        if unlikely(plate
            .area(at)
            .scan()
            .any(|(_, grid)| unlikely(grid.mob.is_some())))
        {
            return Err(mob);
        }
        // set the plate
        mob.at = major.normalize_area(at);
        mob.on_plate = true;
        let mob: Arc<MobBlock> = mob.into_inner(&major.pkey);
        let weak = Weak::from(&mob);
        let mob = unsafe { CheapMobArc::from_arc(mob) };

        for (_, grid) in plate.area_mut(at) {
            grid.mob = Some(mob);
        }
        debug_assert_eq!(weak.strong_count(), 1);
        Ok(weak)
    }

    #[inline]
    pub fn take_plate<'g, M: Mob + ?Sized>(
        plate: &mut Plate,
        major: &MajorAngelos,
        mob: MobRefMut<'g, M>,
    ) -> MobBox<M> {
        debug_assert_eq!(mob.strong_count(), 1);
        // clear the plate
        for (_, grid) in plate.area_mut(mob.at()).scan() {
            debug_assert!(grid.mob.is_some());
            grid.mob = None;
        }
        // convert
        debug_assert_eq!(mob.strong_count(), 1);
        let mut mob = unsafe { MobBox::new_unchecked(mob.get_inner(&major.pkey).into_arc()) };
        mob.on_plate = false;
        mob
    }

    #[inline]
    pub fn reset_plate<'g, M: Mob + Unsize<dyn Mob> + ?Sized>(
        plate: &mut Plate,
        major: &MajorAngelos,
        mob: &mut MobRefMut<M>,
        new_at: CrdI,
    ) -> Result<(), ()> {
        debug_assert_eq!(mob.strong_count(), 1);
        let at = mob.at();
        let mut inner = mob.get_inner(&major.pkey);
        let ptr = inner.as_ptr() as *const ();
        // check if there is another mob
        if unlikely(plate.area(new_at).scan().any(|(_, grid)| {
            unlikely(grid.mob.is_some_with(|pos_mob| {
                unlikely(pos_mob.as_ptr() as *const () != ptr)
            }))
        })) {
            return Err(());
        }
        // resetZ
        unsafe { inner.as_mut() }.at = major.normalize_area(new_at);
        // clear the old grids
        for (pos, grid) in plate.area_mut(at) {
            grid.mob = None;
        }
        // set the new grids
        for (_, grid) in plate.area_mut(new_at) {
            grid.mob = Some(inner);
        }
        debug_assert_eq!(mob.strong_count(), 1);
        Ok(())
    }
}
