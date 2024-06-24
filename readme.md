# Processor
Execute instructions, manage memory, run programs. Provides hardware, compiler, and assembler.

![image](https://github.com/tfe-exr/Processor/assets/163682431/b50ffeb9-3125-4285-bcdd-c1681266fe38)

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