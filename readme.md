# Processor
Execute instructions, manage memory, run programs. Provides hardware, compiler, and assembler.

![image](https://github.com/arrseq/lightning/assets/163682431/5aff3853-19ed-4c61-be24-29653e934091)

# Packages
| Name         | Description                                                                                                                                           |
|--------------|-------------------------------------------------------------------------------------------------------------------------------------------------------|
| emulator     | Desktop application that allows you to run and pupet the emulator service. Contains tools for debugging, writing high level code, compiling and more. |
| programming  | Tools for the programming language such as the compiler, parser, intellisence and more...                                                             |
| protocol     | JavaScript library which allows you to communicate with the emulator service.                                                                         |
| server       | The server for the emulator. Can be interfaced through the protocol library.                                                                          |
| src          | The main Rust project containing the emulator, high level programming language compiler, assembler, debugger, and more.                               |

```
Encoder2 Format
[...prs.help?...?] [esc_pr] [..opc..] [reg?] [indexing?] [constant?]

  - '?' suffix      Optional.        
  - '.' delimiter   Sequenced after.
        
  * 'prs'           Prefixes sequence.
  * 'help'          Byte that contains helper information for the preceding prefix.
  * 'esc_pr'        Escape prefix indicating whether 1 or 2 byte opcode should be used.
  * 'opc'           Operation code bytes, either 1 or 2.
  * 'reg'           Dual register byte.
  * 'indexing'      Used for doing address targeting from an equation.
  * 'constant'      Bytes for the constant addressing mode.

  INFO: To enter the constant addressing mode, the 'CONST_ADDR' prefix must be used to
        indicate that the dynamic register field in the 'reg' byte will not be used.
  INFO: Constant addressing for example does not have a 'help' byte, instead, it utilizes
        the dynamic field of the 'reg' byte as the selector for the size of the constant. 
  INFO: Addressing modes such as 'indexing' can also use the constant as the displacement
        field.
```