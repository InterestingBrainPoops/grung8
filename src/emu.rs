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
        let expanded = (0..4)
            .map(|x| ((current_instruction << (4 * x)) >> 12) as u8)
            .collect::<Vec<u8>>();
        let expanded = [expanded[0], expanded[1], expanded[3], expanded[4]];
        const A: u8 = 10;
        const B: u8 = 11;
        const C: u8 = 12;
        const D: u8 = 13;
        const E: u8 = 14;
        const F: u8 = 15;
        let x = expanded[1];
        let y = expanded[2];
        let n = expanded[3];
        let nn = ((y) << 4) | (n);
        let nnn = ((x as u16) << 8) | ((y as u16) << 4) | (n as u16);
        println!("{:?}", expanded);
        match expanded {
            [0, 0, E, 0] => {
                // zero out the vram
                self.vram.fill([0; 64]);
            }
            [1, _, _, _] => {
                // jump to NNN
                self.program_counter = nnn as usize;
            }
            [6, _, _, _] => {
                // set VX to NN.
                self.general_regs[x as usize] = nn;
                self.program_counter += 2;
            }
            [7, _, _, _] => {
                // increment VX by NN
                self.general_regs[x as usize] += nn;
                self.program_counter += 2;
            }
            [A, _, _, _] => {
                // set the index register to NNN
                self.index_reg = nnn;
                self.program_counter += 2;
            }
            [D, _, _, _] => {
                println!("Display / Draw");
                let x = self.general_regs[x as usize] % 64;
                let y = self.general_regs[y as usize] % 32;
                let sprite_data =
                    &self.memory[(self.index_reg as usize)..(self.index_reg as usize + n as usize)];
                for (idx, data) in sprite_data.iter().enumerate() {
                    if y as usize + idx < 32 {
                        self.vram[y as usize + idx][x as usize] ^= *data;
                    }
                }
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
    pub fn display(&self, canvas: &mut [u8]) {}
}
