extern crate arrseq_lightning;

use arrseq_lightning::instruction::{Instruction, LargeImmediate, register, SegmentCode};
use arrseq_lightning::instruction::load_immediate::{Immediate, Segment};

fn main() {
    let c_max = 10u16;

    let program = [
        Instruction::LoadImmediate {
            destination: register::Code::new(0),
            segment: Segment::Segment0(Immediate::new(c_max as u32))
        }                                                                                                                                    // hlt

        // pseudo code:
        //
        // ```
        // let count = c_max;
        // loop {
        //     count -= 1;
        //     if count == 0 { return 0 }
        // }
        // ```
    ];
}