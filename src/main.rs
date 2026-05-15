use new_project::cpu::{
    create_new_cpu,
    load_rom,
    read_new_file,
    map_keys_for_games,
    create_new_window,
    cycle,
};
use minifb::{Key};


fn main() {
    let mut cpu = create_new_cpu();
    
    match read_new_file("C:/My_VS_code_Projects/Rust/new_project/src/ROM_Files/Pong (1 player).ch8") {
        Ok(rom) => load_rom(&mut cpu, &rom),
        Err(e) => println!("{}", e),
    }

    let mut window = create_new_window().unwrap();
        
    // 500Hz = каждые 2ms один опкод
    window.set_target_fps(30);
    let opcodes_per_frame = 15; // 10 опкодов за кадр = ~600Hz

    while window.is_open() && !window.is_key_down(Key::Escape) {
        map_keys_for_games(&window, &mut cpu);
        
        // выполняем несколько опкодов за кадр
        cycle(&mut cpu, opcodes_per_frame);

        let buffer: Vec<u32> = cpu.display.iter()
            .map(|&p| if p { 0xFFFFFFFF } else { 0x00000000 })
            .collect();
        window.update_with_buffer(&buffer, 64, 32).unwrap();
    }
}