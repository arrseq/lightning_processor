use em_instruction::{absolute, Destination, Instruction};
use em_instruction::absolute::Data;
use em_instruction::operand::{AllPresent, Dynamic, Operands, Static};
use em_instruction::operation::arithmetic::Arithmetic;
use em_instruction::operation::Extension;

fn exec_instruction(ins: Instruction) {
	println!("Op: {:?}", ins.operation);
	
	match ins.operation {
		Extension::Arithmetic(a) => {
			println!("-- Arithmetic extension detected");
			println!("-- Operation: {:?}", a);
		},
		_ => {}
	}
}

fn main() {
	// add br0, 5 ; Store 5 in byte register 0
	let operation = Instruction {
		operation:     Extension::Arithmetic(Arithmetic::Add),
		width:         absolute::Type::Byte,
		destination:   Destination::Static, // Store value in r0
		operands:      Operands::AllPresent(AllPresent {
			x_static:  0, // r0 target
			x_dynamic: Dynamic::Constant(Data::Byte(5))
		})
	};
	
	exec_instruction(operation);
	
	let x_dynamic = Dynamic::Constant(Data::Byte(5));
	
	let all = Operands::AllPresent(AllPresent {
	    x_static: 10,
	    x_dynamic: x_dynamic.clone()
	});
	
	let static_only = Operands::Static(10);
	let dynamic_only = Operands::Dynamic(x_dynamic.clone());
	let none = Operands::None;
	
	assert_eq!(*all.try_x_dynamic().unwrap(), x_dynamic);
	assert_eq!(*dynamic_only.try_x_dynamic().unwrap(), x_dynamic);
	assert!(static_only.try_x_dynamic().is_none());
	assert!(none.try_x_dynamic().is_none());
}