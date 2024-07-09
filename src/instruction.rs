use instruction::operand::GetConfiguration;
use instruction::operand::register::Register;
use instruction::operand::registers::Registers;
use instruction::operation::{Extension, Operation};
use instruction::prefix::{ExecutionMode, Prefixes};
use number::low::{LowNumber, LowSize};
use utility::{Encode, EncodeDynamic};

pub mod operand;
pub mod operation;
pub mod prefix;

pub struct Instruction {
    pub branch_likely_taken: Option<bool>,
    pub execution_mode: Option<ExecutionMode>,
    pub operation: Operation
}

impl EncodeDynamic for Instruction {
    fn encode_dyn(&self, output: &mut Vec<u8>) {
        // region: Backend.
        let op_code =  self.operation.to_smallest_code();
        let extension = Extension::from(self.operation);
        
        let prefixes = Prefixes {
            escape: LowSize::from(op_code),
            // The default extension is always the basic extension, so to avoid adding unnecessary prefixes, add the
            // extension prefix indicator only if It's something other than the default.
            extension: if !matches!(extension, Extension::Basic) { Some(extension) } else { None },
            execution_mode: self.execution_mode,
            branch_likely_taken: self.branch_likely_taken
        };
        
        prefixes.encode_dyn(output);
        match op_code {
            LowNumber::Byte(x) => output.push(x),
            LowNumber::Word(x) => output.extend(x.to_le_bytes())
        };
        // endregion
        
        // region: Front end.
        if let Some(configuration) = self.operation.get_configuration() {
            let encoded = match configuration {
                operand::Configuration::Dual(x) => x.encode(),
                operand::Configuration::Dynamic(x) => x.encode(),
                operand::Configuration::Static(x) => x.encode()
            };
            
            // Operand information.
            output.push(encoded);
            
            // Registers.
            let dynamic = configuration.get_dynamic();
            let static_register = configuration.get_static_register().unwrap_or(Register::default());
            let dynamic_register = if let Some(dynamic) = dynamic { dynamic.get_register().unwrap_or(Register::default()) } else { Register::default() };
            let registers = Registers { dynamic: dynamic_register, r#static: static_register };
            
            output.push(registers.encode());
            
            // Immediate.
            if let Some(dynamic) = dynamic { if let Some(constant) = dynamic.get_constant() { output.extend(constant.to_le_bytes()); } }
        }
        // endregion
    }
}