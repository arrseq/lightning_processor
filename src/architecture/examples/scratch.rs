extern crate architecture;

use architecture::{data::{self, ByteStreamResult}, instruction::{self, Instruction, ParserError}};

struct DataSource {
    cursor: usize,
    bytes: Vec<u8>
}

impl DataSource {
    pub fn new() -> Self {
        DataSource {
            cursor: 0,
            bytes: vec![
                // Sample
                // opcode  r0  imm   opcode  r0  imm
                   11,     2,  1, 2, 11,     15, 10, 12 
            ]
        }
    }

    fn read_at_index(&self, index: usize) -> ByteStreamResult {
        if index > self.bytes.len() - 1 {
            return Err(());
        }

        return Ok(self.bytes[index]);
    }
}

impl data::ByteStream for DataSource {
    fn get_queued(&mut self) -> ByteStreamResult {
        let result = self.read_at_index(self.cursor);
        self.cursor += 1;
        result
    }

    fn get_relative(&mut self, position: isize) -> ByteStreamResult {
        // let index = self.cursor + position;
        // self.bytes[index];
        todo!()
    }

    fn set_cursor(&mut self, cursor: usize) {
        self.cursor = cursor;
    }

    fn get_current(&mut self) -> ByteStreamResult {
        self.read_at_index(self.cursor)
    }

    fn get_at(&mut self, point: usize) -> ByteStreamResult {
        self.read_at_index(point)
    }

    fn get_cursor(&mut self) -> usize {
        self.cursor
    }
}

fn print_instruction(ins: instruction::Instruction) {
    println!("-- Operation: {}", ins.opcode);
    println!("-- Registers [{:?}, {:?}]", ins.r0, ins.r1);
    println!("-- Immediate: {:?}", ins.imm);
    println!("END");
}

fn fetch_next_instruction(mut reg_set_from_imm_parser: instruction::Parser) -> instruction::Parser {
    let res = reg_set_from_imm_parser.try_parse_queued();

    match res {
        Ok(ins) => print_instruction(ins),
        Err(err) => println!("Parser Error: {}", match err {
            ParserError::EndOfStream => "End of stream",
            ParserError::InvalidOpcode => "Unexpected opcode"
        })
    }

    reg_set_from_imm_parser
}

fn main() {
    let mut ds_store = DataSource::new();

    // sri r0 imm
    let mut reg_set_from_imm_parser = instruction::Parser {
        byte_stream: &mut ds_store,
        imm_expected: true,
        r0_expected: true,
        r1_expected: false,
        imm_size: 2,
        opcode: 11
    };

    reg_set_from_imm_parser = fetch_next_instruction(reg_set_from_imm_parser);
    reg_set_from_imm_parser = fetch_next_instruction(reg_set_from_imm_parser);
    fetch_next_instruction(reg_set_from_imm_parser);
}