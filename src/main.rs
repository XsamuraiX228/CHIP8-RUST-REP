use minifb::{Key};
use new_project::cpu::{create_emulator, read_rom, map_keys, create_window};
use new_project::chip8::module::Versions;

const VERSION: Versions = Versions::SuperChip8;

fn main() {
    let mut cpu = create_emulator(VERSION);

    let args: Vec<String> = std::env::args().collect();
    let path = match args.get(1) {
        Some(p) => p,
        None => {
            eprintln!("Укажи путь к ROM: cargo run -- path/to/rom.ch8");
            std::process::exit(1);
        }
    };
    
    match read_rom(path) {
        Ok(rom) => cpu.load_rom(&rom),
        Err(e) => println!("{}", e),
    }

    let mut window = create_window().unwrap();
    window.set_target_fps(60);
    let mut buffer = [0u32; 64 * 32];

    while window.is_open() && !window.is_key_down(Key::Escape) {
        map_keys(&window, &mut cpu);

        for _ in 0..10 {
            cpu.cycle();
        }
        cpu.decrement_timers();

        buffer.copy_from_slice(&cpu.render());
        window.update_with_buffer(&buffer, 64, 32).unwrap();
    }
}