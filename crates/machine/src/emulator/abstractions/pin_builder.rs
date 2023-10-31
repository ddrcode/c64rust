use super::{IPin, Pin, PinDirection};
use std::ops::{Range, RangeInclusive};
use std::rc::Rc;

#[derive(Default, Clone)]
struct PinBuilderItem {
    name: Option<String>,
    direction: PinDirection,
    enabled: bool,
    tri_state: bool,
    io: bool,
}

pub struct PinBuilder {
    pins: Vec<PinBuilderItem>,
    size: usize,
    elems: Vec<usize>,
}

impl PinBuilder {
    pub fn new(num_of_pins: usize) -> Self {
        let mut pins = Vec::with_capacity(num_of_pins);
        (0..num_of_pins).for_each(|_| {
            pins.push(PinBuilderItem::default());
        });
        PinBuilder {
            pins,
            size: num_of_pins,
            elems: vec![0],
        }
    }

    pub fn with_range(&mut self, range: RangeInclusive<usize>) -> &mut Self {
        self.elems = Vec::from_iter(range.start() - 1..*range.end());
        self
    }

    pub fn set_range(
        &mut self,
        range: RangeInclusive<usize>,
        prefix: &str,
        prefix_start: usize,
        direction: PinDirection,
    ) -> &mut Self {
        self.with_range(range)
            .name_prefix(prefix, prefix_start)
            .direction(direction)
    }

    pub fn with_all(&mut self, range: Range<usize>) -> &mut Self {
        self.elems = Vec::from_iter(0..self.size);
        self
    }

    pub fn direction(&mut self, direction: PinDirection) -> &mut Self {
        self.elems.iter().for_each(|idx| {
            self.pins[*idx].direction = direction;
        });
        self
    }

    pub fn input(&mut self) -> &mut Self {
        self.direction(PinDirection::Input)
    }

    pub fn output(&mut self) -> &mut Self {
        self.direction(PinDirection::Output)
    }

    pub fn tri_state(&mut self) -> &mut Self {
        self.elems.iter().for_each(|idx| {
            self.pins[*idx].tri_state = true;
        });
        self
    }

    pub fn io(&mut self) -> &mut Self {
        self.elems.iter().for_each(|idx| {
            self.pins[*idx].io = true;
        });
        self
    }

    pub fn name(&mut self, name: &str) -> &mut Self {
        self.pins[self.elems[0]].name = Some(name.to_string());
        self
    }

    pub fn name_prefix(&mut self, prefix: &str, from: usize) -> &mut Self {
        self.elems.iter().enumerate().for_each(|(idx, elem)| {
            self.pins[*elem].name = Some(format!("{prefix}{}", from + idx));
        });
        self
    }

    pub fn name_prefix_dec(&mut self, prefix: &str, from: usize) -> &mut Self {
        self.elems.iter().enumerate().for_each(|(idx, elem)| {
            self.pins[*elem].name = Some(format!("{prefix}{}", from - idx));
        });
        self
    }

    pub fn next(&mut self) -> &mut Self {
        self.elems = vec![self.elems[self.elems.len() - 1]];
        self
    }

    pub fn next_n(&mut self, n: usize) -> &mut Self {
        let x = self.elems[self.elems.len() - 1];
        self.with_range(x..=(x + n))
    }

    pub fn set(&mut self, id: usize, name: &str, direction: PinDirection) -> &mut Self {
        self.elems = vec![id - 1];
        self.pins[id - 1] = PinBuilderItem {
            name: Some(name.to_string()),
            direction,
            enabled: true,
            tri_state: false,
            io: false,
        };
        self
    }

    pub fn build(&self) -> Vec<Rc<Pin>> {
        self.pins
            .iter()
            .map(|item| {
                let pin = Pin::new(item.direction, item.tri_state, item.io);
                if let Some(name) = &item.name {
                    pin.set_name(name.clone());
                }
                pin
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::emulator::abstractions::PinDirection::*;

    #[test]
    fn test_pin_builder() {
        let pins = PinBuilder::new(40)
            .set(1, "VPB", Output)
            .set(2, "RDY", Input)
            .set(3, "PHI1O", Output)
            .set(4, "IRQB", Input)
            .set(5, "MLB", Output)
            .set(6, "NMIB", Input)
            .set(7, "SYNC", Output)
            .set(8, "VDD", Input)
            .set_range(9..=20, "A", 0, Output)
            .tri_state()
            .set(21, "VSS", Input)
            .set_range(22..=25, "A", 12, Output)
            .with_range(26..=33)
            .direction(Output)
            .name_prefix_dec("D", 7)
            .tri_state()
            .io()
            .set(34, "RWB", Output)
            .tri_state()
            .set(35, "NC", Input)
            .set(36, "BE", Input)
            .set(37, "PHI2", Input)
            .set(38, "SOB", Output)
            .set(39, "PHI2O", Output)
            .set(40, "RESB", Input)
            .build();

        assert_eq!(Input, pins[1].direction());
        assert_eq!(Output, pins[8].direction());
        assert_eq!("A0", pins[8].name().unwrap());
        assert_eq!("A11", pins[19].name().unwrap());
        assert_eq!("VSS", pins[20].name().unwrap());
        assert_eq!("A12", pins[21].name().unwrap());
        assert_eq!("A15", pins[24].name().unwrap());
        assert_eq!("D7", pins[25].name().unwrap());
        assert_eq!("D0", pins[32].name().unwrap());
        assert_eq!("RWB", pins[33].name().unwrap());
        assert_eq!(true, pins[33].tri_state());
    }
}