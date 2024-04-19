use emulator::register;

fn main() {
    let mut core0 = emulator::core::Core::new();

    core0.perform_register();
}