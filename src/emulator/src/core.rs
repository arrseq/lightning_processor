use crate::register;

pub enum Permission {
    None,
    NonSafe,
    All
}

pub struct Core {
    pub registers: register::File,
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
        // self.registers.
        unimplemented!()
    }

    pub fn is_safe(&self) -> bool {
        self.safe
    }

    pub fn set_safe(&mut self, safe: bool) {
        unimplemented!()
    }
}