//
pub struct Deamon<'c> {
    pub angelos: Angelos<'c>,
    plate: &'c mut Matrix<Block, 1, 1>,
    bound: CrdI,
}

impl<'c> Deamon<'c> {
    pub fn new(cosmos: &'c mut Cosmos, bound: CrdI) -> Self {
        Self {
            angelos: cosmos.angelos.make_worker(),
            plate: &mut cosmos.plate,
            bound,
        }
    }

    pub fn set(
        &mut self,
        mob: ArcBox<MobBlock>,
        at: CrdI,
    ) -> Result<P<MobBlock>, ArcBox<MobBlock>> {
        if !self.contains(at) {
            return Err(mob);
        }

        // check if the plate is empty
        if self.plate.area(at).scan().any(|(_, grid)| grid.mob.is_some()) {
            return Err(mob);
        }
        // set the plate
        let mob: P<MobBlock> = mob.into();
        for (_, grid) in self.plate.area_mut(at) {
            grid.mob = Some(mob.clone());
        }
        Ok(mob)
    }

    pub fn take(&mut self, mob: Weak<MobBlock>) -> Result<ArcBox<MobBlock>, ()> {
        let mob = mob.upgrade().ok_or(())?;
        let at = mob.at();

        // check if the mob.at() is valid
        if &mob != self.plate[at.from()].mob.as_ref().ok_or(())? {
            return Err(());
        }
        let scan = self.plate.area_mut(at).scan();
        // quick fail: check if it's unique after clearing the plate
        if scan.len() + 1 < mob.strong_count() {
            return Err(());
        }
        // clear the plate
        for (_, grid) in scan {
            grid.mob = None;
        }
        // convert
        mob.try_into_box(&self.angelos.major.pkey)
            .map_err(|_| unreachable!())
    }

    /// 尝试把[`mob`]移动到[`new_at`]的位置。
    pub fn reset(&mut self, mob: Weak<MobBlock>, new_at: CrdI) -> Result<(), ()> {
        if !self.contains(new_at) {
            return Err(());
        }
        let mut mob = mob.upgrade().ok_or(())?;
        let at = mob.at();
        // check if there is another mob
        if self.plate
            .area(at)
            .scan()
            .any(|(_, grid)| grid.mob.is_some_with(|pos_mob| pos_mob != &mob))
        {
            return Err(());
        }
        // clear the old grids
        for (pos, grid) in self.plate.area_mut(at) {
            if !new_at.contains(&pos.try_into().unwrap()) {
                grid.mob = None;
            }
        }
        // set the new grids
        for (_, grid) in self.plate.area_mut(new_at) {
            if grid.mob.is_none() {
                grid.mob = Some(mob.clone());
            }
        }
        // resetZ
        WriteGuard::with(&self.angelos.major.pkey, |g| {
            unsafe { mob.get_mut_unchecked(g) }.at = new_at
        });
        Ok(())
    }

    #[inline]
    fn contains(&self, mut p: CrdI) -> bool {
        p = self.angelos.major.normalize_area(p);
        self.bound.contains_coord_interval(&p, self.angelos.major.plate_size)
    }
}
