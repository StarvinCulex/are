use crate::arena::body::element;
use crate::arena::*;

pub struct Body {
    pub name: SWord,
    pub element: element::Element,
}

impl Body {
    pub fn hear(&self, cosmos: &Cosmos, self_at: Coord<isize>, message: &Message) {
        self.element.hear(cosmos, self_at, message)
    }

    pub fn watch(&self, cosmos: &Cosmos, self_at: Coord<isize>) {}

    pub fn act(&mut self, at: Coord<isize>, angelos: &Angelos) {
        element::Element::act(&mut self.element, at, angelos);
        self.name = self.element.get_name();
    }
}

impl Default for Body {
    fn default() -> Self {
        Body {
            name: "".into(),
            element: element::Element::default(),
        }
    }
}
