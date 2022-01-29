use crate::arena::Cosmos;

pub trait Mind {
    fn observe(&mut self, cosmos: &Cosmos) -> Result<(), ()>;

    fn make_move(&mut self, cosmos: &Cosmos) -> Result<(), ()>;
}

pub trait MindFactory {
    fn try_creating_mind(&mut self) -> Option<Box<dyn Mind>>;
}
