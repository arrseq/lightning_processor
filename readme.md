# The Four Elements - Processor (TFEP)
A processor project focused on speed, efficiency, cost, and performance. **Current implementations will be for FPGAs and in Verilog.**

> [!NOTE]
> This is a modern CPU architecture, so you may expect a microcode engine to exist but there will never be one for this CPU. The reason is there is simply no good use or high enough benefits to use it. It reduces performance and was mainly to prevent hardwiring. But now CPUs are made in Verilog so this becomes unnecessary.
https://discord.gg/4zt8sUEbNy

### [Documentation](./docs.md)
> [!IMPORTANT] 
> The documentation is currently not finish or developed yet.

Read information for how to write code for the processor, how to emulate, and more. 

### Recommended Extensions
- TODO Highlight
- Todo Tree

### Setting up your development environment
<!-- TODO: Add OpenCL once it is used in the emulation core -->
This project is written in [Rust](https://www.rust-lang.org/). Download [Rust Up](https://rustup.rs/) to quickly setup your environment. Follow the steps below to get the processor running on your system.

Clone the processor to your computer and enter the directory.
```bash
> mkdir Silver-Skies
> cd Silver-Skies
> git clone https://github.com/Silver-Skies/Processor
> cd Processor
```

Download the dependencies and build the project.
```bash
> cargo build
```

### License
There is no official for this project. We own the code and you may clone, fork, or take the code as a copy and not face legal repercussions.