use std::{collections::HashMap, cell::RefCell, rc::Rc};

use super::{Component, PinStateChange, Pin};

pub struct Circuit {
    pins: HashMap<u32, (String, String)>,
    connections: HashMap<u32, u32>,
    // QUESTION #1
    // I use RefCell, because I need a mutable reference to
    // components - to call on_pin_state_change.
    // Are there any other ways to avoid it?
    components: HashMap<String, RefCell<Box<dyn Component>>>,
}

impl Circuit {
    pub fn component(&self, name: &str) -> &RefCell<Box<dyn Component>> {
        &self.components[name]
    }
}

// --------------------------------------------------------------------
// Circuit Pin handler
// This is where the magic happens - it makes the circuit "reactive"
// for every pin state change.

struct CircuitPinHandler(Rc<Circuit>);

impl PinStateChange for CircuitPinHandler {
    fn on_state_change(&mut self, pin: &Pin) {
        let id = pin.inner_id().unwrap();

        let (component_id, reader_pin_name) = {
            let circuit = &self.0;
            println!("Changing pin: {}, {}", pin.name(), id);
            let reader_id = circuit.connections[&id];
            circuit.pins[&reader_id].clone()
        };

        let rpin = {
            let c = self.0.components[&component_id].borrow();
            let p = c.get_pin(&reader_pin_name).unwrap();
            p.set_val(pin.state());
            p.clone()
        };

        // Would be great if I could use hasmap's get_mut here,
        // but I can't, because self.0 is a Rc<Circuit>
        // QUESTION #2: is there any option to fix it?
        let mut component = self.0.components[&component_id].borrow_mut();
        println!(
            "Reader pin: {}, {}",
            rpin.name(),
            rpin.inner_id().unwrap()
        );

        println!("Updating compoent {}", component_id);
        component.on_state_change(&rpin);
    }
}

// --------------------------------------------------------------------
// CircuitBuilder
// Helps building circuits

pub struct CircuitBuilder {
    components: Option<HashMap<String, RefCell<Box<dyn Component>>>>,
    pins: HashMap<u32, (String, String)>,
    last_pin_id: u32,
    connections: HashMap<u32, u32>,
}

impl CircuitBuilder {
    pub fn new() -> Self {
        CircuitBuilder {
            components: Some(HashMap::new()),
            pins: HashMap::new(),
            last_pin_id: 0,
            connections: HashMap::new(),
        }
    }

    pub fn add_component(&mut self, name: &str, cmp: impl Component + 'static) -> &mut Self {
        self.components
            .as_mut()
            .unwrap()
            .insert(name.to_string(), RefCell::new(Box::new(cmp)));
        self
    }

    fn add_pin(&mut self, component_name: &str, pin_name: &str) -> u32 {
        self.pins.insert(
            self.last_pin_id,
            (component_name.to_string(), pin_name.to_string()),
        );
        self.last_pin_id += 1;
        self.last_pin_id - 1
    }

    fn add_connection(&mut self, writer_id: u32, reader_id: u32) {
        self.connections.insert(writer_id, reader_id);
    }

    pub fn link(
        &mut self,
        writer_name: &str,
        writer_pin_name: &str,
        reader_name: &str,
        reader_pin_name: &str,
    ) -> &mut Self {
        let writer_id = self.add_pin(writer_name, writer_pin_name);
        let reader_id = self.add_pin(reader_name, reader_pin_name);
        self.add_connection(writer_id, reader_id);

        self
    }

    pub fn build(&mut self) -> Rc<Circuit> {
        let c = Circuit {
            pins: self.pins.clone(),
            connections: self.connections.clone(),
            components: self.components.take().unwrap(),
        };

        // QUESTION #3
        // Is there a way to make it more elegant?
        // I need to use Rc, as I need to pass the reference to the entire Circuit
        // to the CircuitPinHandler handler. And there is RefCell, as
        // on_pin_state_change requires mutable self
        let cref = Rc::new(c);
        let handler = Rc::new(RefCell::new(CircuitPinHandler(Rc::clone(&cref))));

        cref.connections.iter().for_each(|(key, rkey)| {
            let data = &cref.pins[key];
            let component = cref.components[&data.0].borrow();
            let pin = component.get_pin(&data.1).unwrap();

            // "injecting" handler to all pins
            // QUESTION #4
            // Achieving the same functionality without callback would, most likely,
            // result in a cleaner code. But is there an alternative to it?
            pin.set_handler(Rc::clone(&handler) as Rc<RefCell<dyn PinStateChange>>);
            pin.set_inner_id(*key);

            let data = &cref.pins[rkey];
            let component = cref.components[&data.0].borrow();
            let pin = component.get_pin(&data.1).unwrap();
            pin.set_inner_id(*rkey);
        });

        cref
    }
}

