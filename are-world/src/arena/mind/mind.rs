use crate::arena::Cosmos;
use crate::PKey;

pub trait Mind: Send {
    fn observe(&mut self, cosmos: &Cosmos, pk: &PKey) -> Result<(), ()>;

    fn make_move(&mut self, cosmos: &Cosmos, pk: &PKey) -> Result<(), ()>;

    fn set_cosmos(&mut self, cosmos: &mut Cosmos) -> Result<(), ()>;

    fn name(&self) -> String;
}
