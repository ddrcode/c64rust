use std::rc::Rc;

use crate::utils::if_else;

use super::Pin;

pub trait Pins {
    fn pins(&self) -> &[Rc<Pin>];

    fn size(&self) -> usize {
        self.pins().len()
    }

    fn by_id(&self, id: usize) -> Option<&Pin> {
        if_else(
            id > 0 && id <= self.size(),
            Some(&self.pins()[id - 1]),
            None,
        )
    }

    fn by_name(&self, name: &str) -> Option<&Pin> {
        self.pins()
            .iter()
            .find(|&pin| pin.name() == name)
            .map(|pin| pin.as_ref())
    }

    fn set_all_tri_state(&self, enable: bool) {
        self.pins()
            .iter()
            .filter(|&pin| pin.tri_state())
            .for_each(|pin| {
                pin.set_enable(enable).unwrap();
            });
    }
}
