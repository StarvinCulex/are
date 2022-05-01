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

    /// 把 cosmos 分成 divide.0 * divide.1 块
    pub fn by_divide(cosmos: &'c mut Cosmos, divide: (Idx, Idx)) -> Vec<Self> {
        let plate_size: Coord<Idx> = cosmos.angelos.plate_size.try_into().unwrap();
        debug_assert!(plate_size.0 % divide.0 == 0 && plate_size.1 % divide.1 == 0);
        debug_assert!(divide.0 >= 3 && divide.1 >= 3);
        let mut chunks = Vec::with_capacity((divide.0 * divide.1) as usize);
        for i in 0..divide.0 {
            for j in 0..divide.1 {
                let bound = Coord(i * divide.0, j * divide.1) | Coord((i + 1) * divide.0 - 1, (j + 1) * divide.1 - 1);
                chunks.push(Self::new(unsafe { std::mem::transmute(&cosmos) }, bound));
            }
        }
        chunks
    }

    /// 把 cosmos 以 chunk_size.0 * chunk_size.1 分块
    pub fn by_chunk_size(cosmos: &'c mut Cosmos, chunk_size: (Idx, Idx)) -> Vec<Self> {
        let plate_size: Coord<Idx> = cosmos.angelos.plate_size.try_into().unwrap();
        debug_assert!(plate_size.0 % chunk_size.0 == 0 && plate_size.1 % chunk_size.1 == 0);
        Self::by_divide(cosmos, (plate_size.0 / chunk_size.0, plate_size.1 / chunk_size.1))
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
        // lock() before upgrade(), as it can be take()-n between upgrade() and lock() otherwise
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
    fn contains(&self, p: CrdI) -> bool {
        self.bound.contains_coord_interval(&p, self.angelos.major.plate_size.try_into().unwrap())
    }
}
