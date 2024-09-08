use arrseq_lightning::instruction::Instruction;

fn main() {
    let mut count = 0u32;
    loop {
        let decoded = Instruction::decode(count);
        dbg!(decoded);
        count += 1;
    }
}