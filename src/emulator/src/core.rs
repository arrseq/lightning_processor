use crate::register;

pub enum Permission {
    None,
    NonSafe,
    All
}

pub struct Core {
    registers: register::File,
    safe: bool
}

impl Core {
    pub fn new() -> Self {
        Core {
            safe: false,
            registers: register::File::new()
        }
    }

    pub fn perform_register(&mut self) {
        let safe = self.is_safe();
        let qwrd = self.registers.find_mut(register::Codes::General00).unwrap();
        match qwrd.set_value(safe, 20) {
            Ok(_) => {},
            Err(_) => panic!("Failed to set register")
        }
        println!("QuadWord Register (value): {:?}", qwrd.get_value(safe));
    }

    pub fn is_safe(&self) -> bool {
        self.safe
    }

    pub fn set_safe(&mut self, safe: bool) {
        todo!()
    }
}