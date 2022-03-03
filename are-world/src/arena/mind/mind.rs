use crate::arena::Cosmos;

pub trait Mind {
    fn observe(&mut self, cosmos: &Cosmos) -> Result<(), ()>;

    fn make_move(&mut self, cosmos: &Cosmos) -> Result<(), ()>;

    fn set_cosmos(&mut self, cosmos: &mut Cosmos) -> Result<(), ()>;
}
