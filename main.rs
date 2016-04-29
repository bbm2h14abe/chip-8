/*
    CHIP - 8 interpreter using rust
    @ Paul Bernitz
    @ Begin: 04/18/2016
*/
// Define screen size.
const W : usize = 0x40;
const H : usize = 0x20;

// 80 8-bit values for built-in font.
const FONTSET : [u8; 0x50] =
[
    0xF0, 0x90, 0x90, 0x90, 0xF0, // 0
    0x20, 0x60, 0x20, 0x20, 0x70, // 1
    0xF0, 0x10, 0xF0, 0x80, 0xF0, // 2
    0xF0, 0x10, 0xF0, 0x10, 0xF0, // 3
    0x90, 0x90, 0xF0, 0x10, 0x10, // 4
    0xF0, 0x80, 0xF0, 0x10, 0xF0, // 5
    0xF0, 0x80, 0xF0, 0x90, 0xF0, // 6
    0xF0, 0x10, 0x20, 0x40, 0x40, // 7
    0xF0, 0x90, 0xF0, 0x90, 0xF0, // 8
    0xF0, 0x90, 0xF0, 0x10, 0xF0, // 9
    0xF0, 0x90, 0xF0, 0x90, 0x90, // A
    0xE0, 0x90, 0xE0, 0x90, 0xE0, // B
    0xF0, 0x80, 0x80, 0x80, 0xF0, // C
    0xE0, 0x90, 0x90, 0x90, 0xE0, // D
    0xF0, 0x80, 0xF0, 0x80, 0xF0, // E
    0xF0, 0x80, 0xF0, 0x80, 0x80, // F
];

struct CPU
{
    mem      : [u8; 0x1000],    // 4096 bytes of RAM.
    v        : [u8; 0x10],      // 16 general purpose 8-bit registers called V.
    i        : u16,             // 16-bit register called I.
    dt       : u8,              // Delay timer.
    st       : u8,              // Sound timer.
    pc       : u16,             // Program counter.
    sp       : u16,             // Stack pointer.
    stack    : [u16; 0x10],     // Stack. Array of 16 16-bit values.
    keyboard : [u8; 0x10],      // Keyboard. 16 key hexadecimal layout.
    display  : [u8; W*H / 8],   // Monochrome display with 64 x 32.
}
impl CPU
{
    /* ************************* Initializer *************************  */
    fn init(&mut self)
    {
        // Set program counter to starting position.
        self.pc = 0x200;

        // Load built-in fontset into memory.
        for i in 0..0x50{
			self.mem[i] = FONTSET[i];
		}

		// XXX TEST SPRITE 
		self.mem[0x300] = 0x3C;
		self.mem[0x301] = 0xC3;
		self.mem[0x302] = 0xFF;
		// XXX TEST SPRITE
		// XXX Test set of instructions XXX
		self.mem[0x200] = 0x61;
		self.mem[0x201] = 0x04;
		self.mem[0x202] = 0xF1;
		self.mem[0x203] = 0x18;	
		self.mem[0x204] = 0xF1;
		self.mem[0x205] = 0x0A;
		self.mem[0x206] = 0xF1;
		self.mem[0x207] = 0x0A;
		self.mem[0x208] = 0xA3;
		self.mem[0x209] = 0x00;
		self.mem[0x20A] = 0xD0;
		self.mem[0x20B] = 0x03;
		// XXX Test set of instructions XXX
	}
    /* ************************* End initializer *************************  */

	/* ************************* Timer *************************  */
	fn update_timers(&mut self){
		if(self.dt > 0){ 
			self.dt -= 1;
		}
		if(self.st > 0){
			if(self.st == 1){
				println!("BEEEEP!");
			}
			self.st -= 1;
		}
	}
	/* ************************* End timer *************************  */

	/* ************************* Draw screen *************************  */
	fn draw_screen(&mut self){
		// TODO
		for y in 0..H{
			for x in 0..W{
				// difficult !
				if (self.display[y * W + x] & (0x80 >> x)) == 0{
					print!(".");
				}else{
					print!("#");
				}
			}
			println!("");
		}
	}
	/* ************************* End draw screen *************************  */
	
	/* ************************* List of instructions *************************  */
	/*
	    0XXX
	*/
	// 00E0 - CLS - Clear Screen
	fn inst_00e0(&mut self){ println!("inst_00e0");
	    // TODO clear screen (?)
	}
	// 00EE - RET - Return from a subroutine
	fn inst_00ee(&mut self){ println!("inst_00ee");   
	    self.pc = self.stack[self.sp as usize];
	    self.sp -= 1;
	}
	
	/*
	    1XXX - 7XXX
	*/
	// 1NNN - JP - Jump to location NNN
	fn inst_1nnn(&mut self, nnn : u16){ println!("inst_1nnn");
	    self.pc = nnn;
	}
    // 2NNN - CALL - Call subroutine at NNN
    fn inst_2nnn(&mut self, nnn : u16){ println!("inst_2nnn");
	    self.sp += 1;
		self.stack[self.sp as usize] = self.pc;
		self.pc = nnn;
	}
    // 3XKK - SE - Skip next instruction if V[X] = KK
    fn inst_3xkk(&mut self, x : u8, kk : u8){ println!("inst_3xkk");
	    if self.v[x as usize] == kk { 
	        self.pc += 2; 
	    }
	}
    // 4XKK - SNE - Skip next instruction if V[X] != KK
    fn inst_4xkk(&mut self, x : u8, kk : u8){ println!("inst_4xkk");
	    if self.v[x as usize] != kk {
	        self.pc += 2; 
	    }
	}
    // 5XY0 - SE - Skip next instruction if V[X] = V[Y]
    fn inst_5xy0(&mut self, x : u8, y : u8){ println!("inst_5xy0");
	    if self.v[x as usize] == self.v[y as usize] { 
	        self.pc += 2; 
	    }
	}
    // 6XKK - LD - Set V[X] = KK
    fn inst_6xkk(&mut self, x : u8, kk : u8){ println!("inst_6xkk");
	    self.v[x as usize] = kk; 
	}
    // 7XKK - ADD - Set V[X] = V[x] + KK
    fn inst_7xkk(&mut self, x : u8, kk : u8){ println!("inst_7xkk");
	    self.v[x as usize] += kk;
	}
	
	/*
	    8XXX
    */
	// 8XY0 - LD - Set V[X] = V[Y]
	fn inst_8xy0(&mut self, x : u8, y :u8){ println!("inst_8xy0"); 
		self.v[x as usize] = self.v[y as usize]; 
	}
	// 8XY1 - OR - Set V[X] = V[X] OR V[Y]
	fn inst_8xy1(&mut self, x : u8, y :u8){ println!("inst_8xy1");
		self.v[x as usize] |= self.v[y as usize]; 
	}
	// 8XY2 - AND - Set V[X] = V[X] AND V[Y]
	fn inst_8xy2(&mut self, x : u8, y :u8){ println!("inst_8xy2");
		self.v[x as usize] &= self.v[y as usize]; 
	}
	// 8XY3 - XOR - Set V[X] = V[X] XOR V[Y]
	fn inst_8xy3(&mut self, x : u8, y :u8){ println!("inst_8xy3");
		self.v[x as usize] ^= self.v[y as usize]; 
	}
	// 8XY4 - ADD - Set V[X] = V[X] + V[Y], set V[F] = carry
	fn inst_8xy4(&mut self, x : u8, y :u8){ println!("inst_8xy4");
	    // Set carry if addition is > 255, then add V[Y] to V[X]
	    if self.v[x as usize] as u16 + self.v[y as usize] as u16 > 0xFF{
	        self.v[0xF] = 1;
	    }else{
	        self.v[0xF] = 0; 
	    }
	    self.v[x as usize] += self.v[y as usize];
	}
	// 8XY5 - SUB - Set V[X] = V[X] - V[Y], set V[F] = NOT borrow
	fn inst_8xy5(&mut self, x : u8, y :u8){ println!("inst_8xy5");
	    if self.v[x as usize] > self.v[y as usize]{
	        self.v[0xF] = 1;
	    }else{
	        self.v[0xF] = 0;
	    }
	    self.v[x as usize] -= self.v[y as usize];
    }
	// 8XY6 - SHR - Set V[X] = V[X] SHR 1
	fn inst_8xy6(&mut self, x : u8, y :u8){ println!("inst_8xy6");
	    // Check least significant bit
	    if self.v[x as usize] & 0x1 > 0{
	        self.v[0xF] = 1;
	    }else{
	        self.v[0xF] = 0;
	    }
	    self.v[x as usize] /= 2;
    }
	// 8XY7 - SUBN - Set V[X] = V[Y] - V[X], set V[F] = NOT borrow
	fn inst_8xy7(&mut self, x : u8, y :u8){ println!("inst_8xy7");
	    if self.v[y as usize] > self.v[x as usize]{
	        self.v[0xF] = 1;
	    }else{
	        self.v[0xF] = 0;
	    }
	    self.v[x as usize] -= self.v[x as usize];
    }
	// 8XYE - SHL - Set V[X] = V[X] SHL 1
	fn inst_8xye(&mut self, x : u8, y :u8){ println!("inst_8xye");
	    // Check most significant bit
	    if self.v[x as usize] & 0x80 > 0{
	        self.v[0xF] = 1;
	    }else{
	        self.v[0xF] = 0;
	    }
	    self.v[x as usize] *= 2;
	}
	
	/*
	    9XXX - DXXX
	*/
     // 9XY0 - SNE - Skip next instruction if V[X] != V[Y]
     fn inst_9xy0(&mut self, x : u8, y : u8){ println!("inst_9xy0");
        if self.v[x as usize] != self.v[y as usize] {
            self.pc += 2;
        }
    }
    // ANNN - LD - Set I = NNN
    fn inst_annn(&mut self, nnn : u16){ println!("inst_annn");
        self.i = nnn;
    }
    // BNNN - JP - Jump to location NNN + V[0]
    fn inst_bnnn(&mut self, nnn : u16){ println!("inst_bnnn");
		self.pc = nnn + self.v[0] as u16;
    }
	// CXKK - RND - Set V[X] = random byte AND KK
	fn inst_cxkk(&mut self, x : u8, kk : u8){ println!("inst_cxkk");
		// TODO generate random number here. 
		self.v[x as usize] = 8/*replace '8' with random-byte*/ & kk;
    }
	// DXYN - DRW - Draw n-byte sprite, start I at (V[X],V[Y]), V[F] = collision
    fn inst_dxyn(&mut self, x : u8, y : u8, n : u8){ println!("inst_dxyn");
		// preset carry-flag to 0
		self.v[0xF] = 0;
		
		// loop through height of sprite
		for iy in 0..n{
			// get current row of sprite (1 byte)
			let line : u8 = self.mem[self.i as usize + iy as usize]; 
			
			// loop through each bit of the current row
			for ix in 0..8{
				// check if active bit is set to 1. 
				// TODO bitshift error !
				if (line & (0x80 >> ix)) != 0{
					// check if pixel on display is currently 1. If so set v[x] to 1.
					if self.display[(x + ix + ((y + iy) * W as u8)) as usize] == 1{
						self.v[0xF] = 1;
					}
					// set pixel-value with XOR.
					self.display[(x + ix + ((y + iy) * W as u8)) as usize] ^= 1;
				}
			}
		}
		
		self.draw_screen();
	}

    /*
        EXXX
    */
    // EX9E - SKP - Skip next instruction if key[V[X]] is pressed
    fn inst_ex9e(&mut self, x : u8){ println!("inst_ex9e");
        // TODO get keyboard input here
    }
    // EXA1 - SKNP - Skip next instruction if key[V[X]] is not pressed
    fn inst_exa1(&mut self, x : u8){ println!("inst_exa1");
		 // TODO get keyboard input here
    }

	/*
	    FXXX
    */
    // FX07 - LD - Set V[X] = delay timer value
    fn inst_fx07(&mut self, x : u8){ println!("inst_fx07");
        self.v[x as usize] = self.dt;
    }
    // FX0A - LD - Wait for key-press, store key-value in V[X]
    fn inst_fx0a(&mut self, x : u8){ println!("inst_fx0a");
        // TODO try to get keypress here    
    }
    // FX15 - LD - Set delay timer = V[X]
    fn inst_fx15(&mut self, x : u8){ println!("inst_fx15");
        self.dt = self.v[x as usize];        
    }
    // FX18 - LD - Set sound timer = V[X]
    fn inst_fx18(&mut self, x : u8){ println!("inst_fx18");
        self.st = self.v[x as usize];   
    }
    // FX1E - ADD - Set I = I + V[X]
    fn inst_fx1e(&mut self, x : u8){ println!("inst_fx1e");
        self.i += self.v[x as usize] as u16; 
    }
    // FX29 - LD - Set I = location of sprite for digit V[X]
    fn inst_fx29(&mut self, x : u8){  println!("inst_fx29");
        // TODO is this correct?
		self.i = self.mem[self.v[x as usize] as usize] as u16;//?
    }
    // FX33 - LD - Store BCD representation of V[X] in I, I+1 and I+2
    fn inst_fx33(&mut self, x : u8){ println!("inst_fx33");
		// TODO this is wrong
		self.i = self.v[x as usize] as u16;// !   
    }
    // FX55 - LD - Store registers V[0] through V[X] in memory, start at I
    fn inst_fx55(&mut self, x : u8){ println!("inst_fx55");
        // TODO check if correct
		for j in 0..x{
			self.mem[self.i as usize] = self.v[j as usize];
			self.i += 1;
		}
    }
    // FX65 - LD - Read registers V[0] through V[X] from memory, start at I
    fn inst_fx65(&mut self, x : u8){ println!("inst_fx65");
		// TODO check if correct
		for j in 0..x{
			self.v[j as usize] = self.mem[self.i as usize];
			self.i += 1;
		} 
    }
    
    /* ************************* End list of instructions *************************  */


    /* ************************* Execute instruction *************************  */
    fn process_instruction(&mut self)
    {
        // Read 2 byte opcode from memory at pc and pc + 1, which point to an 8bit register each.
		// Store the first part in the higher byte and second in the lower.
		let opcode : u16 = ((self.mem[self.pc as usize] as u16) << 8) | (self.mem[self.pc as usize + 1] as u16);
		if opcode == 0{
			panic!("Empty memory address!");
		}
		
		// Extract the bitfields from the instruction.
		let nnn : u16 = opcode & 0xFFF;				// Lowest 12 bits.
		let n : u8 = (opcode & 0xF) as u8;			// Lowest 4 bits.
		let x : u8 = ((opcode >> 8) & 0xF) as u8;	// The lower 4 bits of the high byte.
		let y : u8 = ((opcode >> 4) & 0xF) as u8;	// The upper 4 bits of the low byte.
		let kk : u8 = (opcode & 0xFF) as u8;		// The low byte.

		// Execute opcodes.
	    match opcode & 0xF000
	    {	// Check highest 4 bits.
	        0x0 => match opcode & 0xF
	        {	// Check lowest 4 bits
	            0x0 => {println!("CLS"); self.inst_00e0(); },	// 00E0 - CLS - Clear Screen
	            0xE => {println!("RET"); self.inst_00ee(); },	// 00EE - RET - Return from a subroutine
	            _ => panic!("Cant find Opcode 0x{:0x}", opcode),
	        },
			0x1000 => {println!("JP"); 		self.inst_1nnn(nnn);}, 		// 1NNN - JP - Jump to location NNN
			0x2000 => {println!("CALL"); 	self.inst_2nnn(nnn);}, 		// 2NNN - CALL - Call subroutine at NNN
			0x3000 => {println!("SE"); 		self.inst_3xkk(x, kk); }, 	// 3XKK - SE - Skip next instruction if V[X] = KK
			0x4000 => {println!("SNE"); 	self.inst_4xkk(x, kk); }, 	// 4XKK - SNE - Skip next instruction if V[X] != KK
			0x5000 => {println!("SE"); 		self.inst_5xy0(x, y); }, 	// 5XY0 - SE - Skip next instruction if V[X] = V[Y]
			0x6000 => {println!("LD"); 		self.inst_6xkk(x, kk); }, 	// 6XKK - LD - Set V[X] = KK
			0x7000 => {println!("ADD"); 	self.inst_7xkk(x, kk); }, 	// 7XKK - ADD - Set V[X] = V[x] + KK
			0x8000 => match opcode & 0xF
			{	// Check lowest 4 bits
				0x0 => {println!("LD");  	self.inst_8xy0(x, y); }, // 8XY0 - LD - Set V[X] = V[Y]
				0x1 => {println!("OR"); 	self.inst_8xy1(x, y); }, // 8XY1 - OR - Set V[X] = V[X] OR V[Y]
				0x2 => {println!("AND");  	self.inst_8xy2(x, y); }, // 8XY2 - AND - Set V[X] = V[X] AND V[Y]
				0x3 => {println!("XOR");  	self.inst_8xy3(x, y); }, // 8XY3 - XOR - Set V[X] = V[X] XOR V[Y]
				0x4 => {println!("ADD"); 	self.inst_8xy4(x, y); }, // 8XY4 - ADD - Set V[X] = V[X] + V[Y], set V[F] = carry
				0x5 => {println!("SUB"); 	self.inst_8xy5(x, y); }, // 8XY5 - SUB - Set V[X] = V[X] - V[Y], set V[F] = NOT borrow
				0x6 => {println!("SHR");	self.inst_8xy6(x, y); }, // 8XY6 - SHR - Set V[X] = V[X] SHR 1
				0x7 => {println!("SUBN");	self.inst_8xy7(x, y); }, // 8XY7 - SUBN - Set V[X] = V[Y] - V[X], set V[F] = NOT borrow
				0xE => {println!("SHL");	self.inst_8xye(x, y); }, // 8XYE - SHL - Set V[X] = V[X] SHL 1
				_ => panic!("Cant find Opcode 0x{:0x}", opcode),
			},
			0x9000 => {println!("SNE"); 	self.inst_9xy0(x, y); }, 	// 9XY0 - SNE - Skip next instruction if V[X] != V[Y]
			0xA000 => {println!("LD"); 		self.inst_annn(nnn); },		// ANNN - LD - Set I = NNN
			0xB000 => {println!("JP"); 		self.inst_bnnn(nnn); },		// BNNN - JP - Jump to location NNN + V[0]
			0xC000 => {println!("RND"); 	self.inst_cxkk(x, kk); },	// CXKK - RND - Set V[X] = random byte AND KK
			0xD000 => {println!("DRW"); 	self.inst_dxyn(x, y, n); },	// DXYN - DRW - Draw n-byte sprite, start I at (V[X],V[Y]), V[F] = collision
			0xE000 => match opcode & 0xFF
			{	// Check lowest 8 bits
				0x9E => {println!("SKP"); 	self.inst_ex9e(x); },		// EX9E - SKP - Skip next instruction if key[V[X]] is pressed
				0xA1 => {println!("SKNP"); 	self.inst_exa1(x); },		// EXA1 - SKNP - Skip next instruction if key[V[X]] is not pressed
				 _ => panic!("Cant find Opcode 0x{:0x}", opcode),
			},
			0xF000 => match opcode & 0xFF
			{	// Check lowest 8 bits
				0x07 => {println!("LD"); self.inst_fx07(x); },	// FX07 - LD - Set V[X] = delay timer value
				0x0A => {println!("LD"); self.inst_fx0a(x); },  // FX0A - LD - Wait for key-press, store key-value in V[X]
				0x15 => {println!("LD"); self.inst_fx15(x); },	// FX15 - LD - Set delay timer = V[X]
				0x18 => {println!("LD"); self.inst_fx18(x); },	// FX18 - LD - Set sound timer = V[X]
				0x1E => {println!("ADD");self.inst_fx1e(x); },  // FX1E - ADD - Set I = I + V[X]
				0x29 => {println!("LD"); self.inst_fx29(x); },  // FX29 - LD - Set I = location of sprite for digit V[X]
				0x33 => {println!("LD"); self.inst_fx33(x); },  // FX33 - LD - Stire BCD representation of V[X] in I, I+1 and I+2
				0x55 => {println!("LD"); self.inst_fx55(x); },  // FX55 - LD - Store registers V[0] through V[X] in memory, start at I
				0x64 => {println!("LD"); self.inst_fx65(x); },	// FX65 - LD - Read registers V[0] through V[X] from memory, start at I
				 _ => panic!("Cant find Opcode 0x{:0x}", opcode),
			},
	        _ => panic!("Cant find Opcode 0x{:0x}", opcode),
	    }

		println!("opcode 0x{:0x}", opcode);
		self.pc +=2;//XXX
    }
    /* ************************* End execute instruction *************************  */
}

fn main()
{
    let mut cpu = CPU{  mem: [0; 0x1000], v: [0; 0x10], i: 0, dt: 0, st: 0, pc: 0, sp: 0,
                        stack: [0; 0x10], keyboard: [0; 0x10], display: [0; W*H / 8] };

    cpu.init();
	// TODO here: cpu.loadProgram("pong");
	
	let is_active = true;
	while(is_active){
		cpu.process_instruction();
		cpu.update_timers();
	}
}
