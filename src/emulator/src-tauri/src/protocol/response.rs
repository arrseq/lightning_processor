use fastuuid::Generator;

pub struct IdGenerator {
    pub in_use: Vec<u32>,
    generator: Generator
}

impl IdGenerator {
    pub fn new() -> Self {
        Self {
            in_use: vec![0u32; 0],
            generator: Generator::new()
        }
    }

    pub fn generate(&mut self) -> u32 {
        let id_a = self.generator.next();
        let id = u32::from_le_bytes([id_a[3], id_a[2], id_a[1], id_a[0]]);

        if !self.in_use.contains(&id) {
            self.in_use.push(id);
            return id;
        }

        self.generate()
    }

    pub fn retire(&mut self, dead: u32) {
        self.in_use.retain(|&x| x != dead);
    }
}

pub struct MessageManager {
    pub idg: IdGenerator
}

impl MessageManager {
    pub fn new() -> Self {
        Self {
            idg: IdGenerator::new()
        }
    }
}