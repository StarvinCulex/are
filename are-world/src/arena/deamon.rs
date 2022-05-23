use core::marker::Unsize;
use std::iter::Map;

use crate::meta::gnd::Ground;

//
pub struct Deamon<'c, 'a> {
    pub angelos: &'a mut Angelos<'c>,
    plate: &'c mut Matrix<Block, 1, 1>,
    bound: CrdI,
}

impl<'c, 'a> Deamon<'c, 'a> {
    pub fn get_ground(&self, p: Crd) -> Result<&Ground, ()> {
        let pos = self.plate.normalize(p.into());
        let bound: Coord<Interval<isize>> = self.bound.into();
        if !bound.contains(&pos) {
            Err(())
        } else {
            Ok(&self.plate[pos].ground)
        }
    }
    pub fn get_ground_mut(&mut self, p: Crd) -> Result<&mut Ground, ()> {
        let pos = self.plate.normalize(p.into());
        let bound: Coord<Interval<isize>> = self.bound.into();
        if !bound.contains(&pos) {
            Err(())
        } else {
            Ok(&mut self.plate[pos].ground)
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
        if !self.contains(p) {
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
        if !self.contains(p) {
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
        mut mob: ArcBox<_MobBlock<M>>,
    ) -> Result<Weak<MobBlock>, ArcBox<_MobBlock<M>>> {
        let at = mob.at;
        if !self.contains(at) {
            return Err(mob);
        }

        Self::set_plate(self.plate, self.angelos.major, mob)
    }

    pub fn take<'g, M: Mob + ?Sized>(
        &mut self,
        mob: MobRefMut<'g, M>,
    ) -> Result<ArcBox<_MobBlock<M>>, MobRefMut<'g, M>> {
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
        if !self.contains(new_at) {
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
    pub fn set_plate<M: Mob + Unsize<dyn Mob> + ?Sized>(
        plate: &mut Matrix<Block, 1, 1>,
        mut major: &MajorAngelos,
        mut mob: ArcBox<_MobBlock<M>>,
    ) -> Result<Weak<MobBlock>, ArcBox<_MobBlock<M>>> {
        // check if the plate is empty
        let at = mob.at;
        if plate.area(at).scan().any(|(_, grid)| grid.mob.is_some()) {
            return Err(mob);
        }
        // set the plate
        mob.at = major.normalize_area(at);
        let mob: Arc<_MobBlock<M>> = mob.into();
        let mob: Arc<MobBlock> = mob;

        for (_, grid) in plate.area_mut(at) {
            grid.mob = Some(mob.clone());
        }
        Ok(Weak::from(&mob))
    }

    #[inline]
    pub fn take_plate<'g, M: Mob + ?Sized>(
        plate: &mut Matrix<Block, 1, 1>,
        major: &MajorAngelos,
        mob: MobRefMut<'g, M>,
    ) -> Result<ArcBox<_MobBlock<M>>, MobRefMut<'g, M>> {
        let at = mob.at();
        let scan = plate.area_mut(at).scan();
        // quick fail: check if it's unique after clearing the plate
        if scan.len() + 1 < mob.strong_count() {
            return Err(mob);
        }
        // clear the plate
        for (_, grid) in scan {
            grid.mob = None;
        }
        // convert
        mob.into_inner(&major.pkey)
            .try_into()
            .map_err(|_| unreachable!())
    }

    #[inline]
    pub fn reset_plate<'g, M: Mob + Unsize<dyn Mob> + ?Sized>(
        plate: &mut Matrix<Block, 1, 1>,
        major: &MajorAngelos,
        mut mob: &mut MobRefMut<M>,
        new_at: CrdI,
    ) -> Result<(), ()> {
        let at = mob.at();
        let mut mob: Arc<MobBlock> = mob.get_inner(&major.pkey);
        // check if there is another mob
        if plate.area(at).scan().any(|(_, grid)| {
            grid.mob.is_some_with(|pos_mob| {
                Arc::as_ptr(pos_mob) as *const () != Arc::as_ptr(&mob) as *const ()
            })
        }) {
            return Err(());
        }
        // clear the old grids
        for (pos, grid) in plate.area_mut(at) {
            if !new_at.contains(&pos.try_into().unwrap()) {
                grid.mob = None;
            }
        }
        // set the new grids
        for (_, grid) in plate.area_mut(new_at) {
            if grid.mob.is_none() {
                grid.mob = Some(mob.clone());
            }
        }
        // resetZ
        unsafe { Arc::get_mut_unchecked(&mut mob) }.at = major.normalize_area(new_at);
        Ok(())
    }
}
