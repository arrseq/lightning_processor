# Processor
Execute instructions, manage memory, run programs. Provides hardware, compiler, and assembler.

![image](https://github.com/tfe-exr/Processor/assets/163682431/6c68452f-4fdf-4cf1-bc2c-18b1f40e3f2e)

# Specifications
- 64-bit data bus with the ability to downsize to 32 bits. 
- Complex instruction set (CISC)
- Versatile instruction encoding and x86 translatable.

# Required tools
- Cargo and other Rust tools (Setup with Rustup)
- Windows C++ build tools
- Python

# Started
- [x] Instruction decoder
- [x] Emulation 
- [x] Assembler 
- [ ] Syntax parser 
- [ ] Compiler

# x54 Protocol
Emulation server protocol used for interfacing with an emulation core with hardware extensions built in.
- Uses address 127.0.0.1:15147 and is unprotected.
