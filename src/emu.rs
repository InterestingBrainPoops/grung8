pub struct Emulator {
    vram: [[u8; 64]; 32],
    call_stack: Vec<u16>,
    memory: [u8; 4096],
    program_counter: usize,
    index_reg: u16,
    delay_tim: u16,
    sound_tim: u16,
    general_regs: [u8; 16],
}

impl Emulator {
    pub fn load(&mut self, path: String) {
        let things = std::fs::read(path).unwrap();
        for (idx, item) in things.iter().enumerate() {
            self.memory[0x200 + idx] = *item;
        }
    }
    pub fn new() -> Self {
        Self {
            vram: [[0; 64]; 32],
            call_stack: vec![],
            memory: [0; 4096],
            program_counter: 0x200,
            index_reg: 0,
            delay_tim: 0,
            sound_tim: 0,
            general_regs: [0; 16],
        }
    }
    pub fn step(&mut self) {
        let current_instruction = self.fetch();
        println!("{:#02x}", current_instruction);
        println!("{}", current_instruction);
        match current_instruction {
            0x00e0 => {
                println!("clear screen");
            }
            _ => {
                panic!("invalid instr");
            }
        }
    }
    fn fetch(&self) -> u16 {
        (self.memory[self.program_counter] as u16) << 8
            | self.memory[self.program_counter + 1] as u16
    }
}
