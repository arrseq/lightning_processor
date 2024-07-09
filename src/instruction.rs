use std::io;
use std::io::{Read, Write};
use instruction::operand::GetConfiguration;
use instruction::operand::register::Register;
use instruction::operand::registers::Registers;
use instruction::operation::{Extension, Operation};
use instruction::prefix::{ExecutionMode, Prefixes};
use number::low::{LowNumber, LowSize};

pub mod operand;
pub mod operation;
pub mod prefix;

#[derive(Debug, Clone, Copy)]
pub struct Instruction {
    pub branch_likely_taken: Option<bool>,
    pub execution_mode: Option<ExecutionMode>,
    pub operation: Operation
}

#[derive(Debug)]
pub enum DecodeError {
    Prefix(prefix::DecodeError),
    Read(io::Error),
    InvalidOperationCode
}

impl Instruction {
    pub fn encode<Output: Write + Extend<u8>>(self, output: &mut Output) {
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
        
        prefixes.encode(output);
        match op_code {
            LowNumber::Byte(x) => output.extend([ x ]),
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
            output.extend([ encoded ]);
            
            // Registers.
            let dynamic = configuration.get_dynamic();
            let static_register = configuration.get_static_register().unwrap_or(Register::default());
            let dynamic_register = if let Some(dynamic) = dynamic { dynamic.get_register().unwrap_or(Register::default()) } else { Register::default() };
            let registers = Registers { dynamic: dynamic_register, r#static: static_register };
            
            output.extend([ registers.encode() ]);
            
            // Immediate.
            if let Some(dynamic) = dynamic {
                if let Some(address_constant) = dynamic.get_address_constant() { output.extend(address_constant.to_le_bytes()); }
                else if let Some(constant) = dynamic.get_constant() {
                    // Ensure the constant fits the correct size of the data.
                    let sized_constant = constant.resize(configuration.get_size());
                    output.extend(sized_constant.to_le_bytes());
                }
            }
        }
        // endregion
    }

    pub fn decode<Input: Read>(source: &mut Input) -> Result<Self, DecodeError> {
        // region: Backend.
        let prefixes = Prefixes::decode(source).map_err(DecodeError::Prefix)?;

        dbg!(prefixes);
        // endregion
        
        // region: Front end.
        
        // Convert opcode to a word.
        let opcode = match prefixes.escape {
            LowSize::Byte => {
                let mut buffer = [0u8; 1];
                source.read_exact(&mut buffer).map_err(DecodeError::Read)?;
                buffer[0] as u16
            },
            LowSize::Word => {
                let mut buffer = [0u8; 2];
                source.read_exact(&mut buffer).map_err(DecodeError::Read)?;
                u16::from_le_bytes(buffer)
            }
        };
        
        let extension = prefixes.extension.unwrap_or(Extension::default());
        let operation = operation::Code::from_extension_and_operation(extension, opcode).ok_or(DecodeError::InvalidOperationCode)?;
        
        dbg!(operation);
        
        // endregion
        todo!()
    }
}