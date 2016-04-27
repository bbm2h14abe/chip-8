/*
    CHIP - 8 interpreter using rust
    @ Paul Bernitz
    @ Begin: 04/18/2016
    @ Opcode-Table finished: 04/22/2016
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
        for i in 0..0x50
		{
			self.mem[i] = FONTSET[i];
		}

		// XXX Test set of instructions XXX
		self.mem[0x200] = 0xFE;
		self.mem[0x201] = 0x55;

		self.mem[0x202] = 0x87;
		self.mem[0x203] = 0x10;

		self.mem[0x204] = 0x81;
		self.mem[0x205] = 0x27;
	}
    /* ************************* End initializer *************************  */


	/* ************************* List of instructions *************************  */
	/*
	    0XXX
	*/
	// 00E0 - CLS - Clear Screen
	fn inst_00e0(&mut self){
	    println!("inst_00e0");
	}
	// 00EE - RET - Return from a subroutine
	fn inst_00ee(&mut self){
	    println!("inst_00ee");
	    self.pc = self.stack[self.sp as usize];
	    self.sp -= 1;
	}

/*
    START UNFINISHED XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
*/	
	/*
	    1XXX - 7XXX
	*/
	// 1NNN - JP - Jump to location NNN
	fn inst_1nnn(&mut self, nnn : u16){
	    
	}
    // 2NNN - CALL - Call subroutine at NNN
    fn inst_2nnn(&mut self, nnn : u16){
	    
	}
    // 3XKK - SE - Skip next instruction if V[X] = KK
    fn inst_3xkk(&mut self, x : u8, kk : u8){
	    if self.v[x as usize] == kk { 
	        self.pc += 2; 
	    }
	}
    // 4XKK - SNE - Skip next instruction if V[X] != KK
    fn inst_4xkk(&mut self, x : u8, kk : u8){
	    if self.v[x as usize] != kk {
	        self.pc += 2; 
	    }
	}
    // 5XY0 - SE - Skip next instruction if V[X] = V[Y]
    fn inst_5xy0(&mut self, x : u8, y : u8){
	    if self.v[x as usize] == self.v[y as usize] { 
	        self.pc += 2; 
	    }
	}
    // 6XKK - LD - Set V[X] = KK
    fn inst_6xkk(&mut self, x : u8, kk : u8){
	    self.v[x as usize] = kk; 
	}
    // 7XKK - ADD - Set V[X] = V[x] + KK
    fn inst_7xkk(&mut self, x : u8, kk : u8){
	    self.v[x as usize] += kk;
	}
/*
    END UNFINISHED XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
*/
	
	/*
	    8XXX
    */
	// 8XY0 - LD - Set V[X] = V[Y]
	fn inst_8xy0(&mut self, x : u8, y :u8){ self.v[x as usize] = self.v[y as usize]; }
	// 8XY1 - OR - Set V[X] = V[X] OR V[Y]
	fn inst_8xy1(&mut self, x : u8, y :u8){ self.v[x as usize] |= self.v[y as usize]; }
	// 8XY2 - AND - Set V[X] = V[X] AND V[Y]
	fn inst_8xy2(&mut self, x : u8, y :u8){ self.v[x as usize] &= self.v[y as usize]; }
	// 8XY3 - XOR - Set V[X] = V[X] XOR V[Y]
	fn inst_8xy3(&mut self, x : u8, y :u8){ self.v[x as usize] ^= self.v[y as usize]; }
	// 8XY4 - ADD - Set V[X] = V[X] + V[Y], set V[F] = carry
	fn inst_8xy4(&mut self, x : u8, y :u8){
	    // Set carry if addition is > 255, then add V[Y] to V[X]
	    if self.v[x as usize] as u16 + self.v[y as usize] as u16 > 0xFF{
	        self.v[0xF] = 1;
	    }else{
	        self.v[0xF] = 0; 
	    }
	    self.v[x as usize] += self.v[y as usize];
	}
	// 8XY5 - SUB - Set V[X] = V[X] - V[Y], set V[F] = NOT borrow
	fn inst_8xy5(&mut self, x : u8, y :u8){
	    if self.v[x as usize] > self.v[y as usize]{
	        self.v[0xF] = 1;
	    }else{
	        self.v[0xF] = 0;
	    }
	    self.v[x as usize] -= self.v[y as usize];
    }
	// 8XY6 - SHR - Set V[X] = V[X] SHR 1
	fn inst_8xy6(&mut self, x : u8, y :u8){
	    // Check least significant bit
	    if self.v[x as usize] & 0x1 > 0{
	        self.v[0xF] = 1;
	    }else{
	        self.v[0xF] = 0;
	    }
	    self.v[x as usize] /= 2;
    }
	// 8XY7 - SUBN - Set V[X] = V[Y] - V[X], set V[F] = NOT borrow
	fn inst_8xy7(&mut self, x : u8, y :u8){
	    if self.v[y as usize] > self.v[x as usize]{
	        self.v[0xF] = 1;
	    }else{
	        self.v[0xF] = 0;
	    }
	    self.v[x as usize] -= self.v[x as usize];
    }
	// 8XYE - SHL - Set V[X] = V[X] SHL 1
	fn inst_8xye(&mut self, x : u8, y :u8){
	    // Check most significant bit
	    if self.v[x as usize] & 0x80 > 0{
	        self.v[0xF] = 1;
	    }else{
	        self.v[0xF] = 0;
	    }
	    self.v[x as usize] *= 2;
	}

/*
    START UNFINISHED XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
*/
	
	/*
	    9XXX - DXXX
	*/

     // 9XY0 - SNE - Skip next instruction if V[X] != V[Y]
     fn inst_9xy0(&mut self, x : u8, y : u8){
        if self.v[x as usize] != self.v[y as usize] {
            self.pc += 2;
        }
     }
    // ANNN - LD - Set I = NNN
    fn inst_annn(&mut self, nnn : u16){
        self.i = nnn;
     }
    // BNNN - JP - Jump to location NNN + V[0]
    fn inst_bnnn(&mut self, nnn : u16){
     }
	// CXKK - RND - Set V[X] = random byte AND KK
	fn inst_cxkk(&mut self, x : u8, kk : u8){
     }
	// DXYN - DRW - Draw n-byte sprite, start I at (V[X],V[Y]), V[F] = collision
    fn inst_dxyn(&mut self, x : u8, y : u8, n : u8){
     }


    /*
        EXXX
    */
    // EX9E - SKP - Skip next instruction if key[V[X]] is pressed
    fn inst_ex9e(&mut self, x : u8){
        
    }
    // EXA1 - SKNP - Skip next instruction if key[V[X]] is not pressed
    fn inst_exa1(&mut self, x : u8){
    
    }

	/*
	    FXXX
    */
    // FX07 - LD - Set V[X] = delay timer value
    fn inst_fx07(&mut self, x : u8){
        self.v[x as usize] = self.dt;
        println!("inst_fx07");
    }
    // FX0A - LD - Wait for key-press, store key-value in V[X]
    fn inst_fx0a(&mut self, x : u8){
        // HOW TO WAIT ON KEYPRESS!? XXX
        // USE EXTERN CRATE TO GET KEYBOARD! CHANGE COMPILER !
        println!("inst_fx0a");
    }
    // FX15 - LD - Set delay timer = V[X]
    fn inst_fx15(&mut self, x : u8){
        self.dt = self.v[x as usize];
        println!("inst_fx15");
    }
    // FX18 - LD - Set sound timer = V[X]
    fn inst_fx18(&mut self, x : u8){
        self.st = self.v[x as usize];
        println!("inst_fx18");
    }
    // FX1E - ADD - Set I = I + V[X]
    fn inst_fx1e(&mut self, x : u8){
        self.i += self.v[x as usize] as u16;
        println!("inst_fx1e");
    }
    // FX29 - LD - Set I = location of sprite for digit V[X]
    fn inst_fx29(&mut self, x : u8){
        // HOW TO GET LOCATION OF SPRITE??! XXX
        println!("inst_fx29");
    }
    // FX33 - LD - Store BCD representation of V[X] in I, I+1 and I+2
    fn inst_fx33(&mut self, x : u8){
        println!("inst_fx33");
    }
    // FX55 - LD - Store registers V[0] through V[X] in memory, start at I
    fn inst_fx55(&mut self, x : u8){
        // ??? XXX
        println!("inst_fx55");
    }
    // FX65 - LD - Read registers V[0] through V[X] from memory, start at I
    fn inst_fx65(&mut self, x : u8){
        println!("inst_fx65");
    }
/*
    END UNFINISHED XXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX
*/
    
    /* ************************* End list of instructions *************************  */


    /* ************************* Execute instruction *************************  */
    fn process_instruction(&mut self)
    {
        // Read 2 byte opcode from memory at pc and pc + 1, which point to an 8bit register each.
		// Store the first part in the higher byte and second in the lower.
		let opcode : u16 = ((self.mem[self.pc as usize] as u16) << 8) | (self.mem[self.pc as usize + 1] as u16);

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
			0x1000 => {println!("JP"); self.inst_1nnn(nnn);}, 	// 1NNN - JP - Jump to location NNN
			0x2000 => {println!("CALL"); self.inst_2nnn(nnn);}, // 2NNN - CALL - Call subroutine at NNN
			0x3000 => {println!("SE"); self.inst_3xkk(x, kk); }, 	// 3XKK - SE - Skip next instruction if V[X] = KK
			0x4000 => {println!("SNE"); self.inst_4xkk(x, kk); }, 	// 4XKK - SNE - Skip next instruction if V[X] != KK
			0x5000 => {println!("SE"); self.inst_5xy0(x, y); }, 	// 5XY0 - SE - Skip next instruction if V[X] = V[Y]
			0x6000 => {println!("LD"); self.inst_6xkk(x, kk); }, 	// 6XKK - LD - Set V[X] = KK
			0x7000 => {println!("ADD"); self.inst_7xkk(x, kk); }, 	// 7XKK - ADD - Set V[X] = V[x] + KK
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
			0x9000 => {println!("SNE"); self.inst_9xy0(x, y); }, 	// 9XY0 - SNE - Skip next instruction if V[X] != V[Y]
			0xA000 => {println!("LD"); self.inst_annn(nnn); },	// ANNN - LD - Set I = NNN
			0xB000 => {println!("JP"); self.inst_bnnn(nnn); },	// BNNN - JP - Jump to location NNN + V[0]
			0xC000 => {println!("RND"); self.inst_cxkk(x, kk); },	// CXKK - RND - Set V[X] = random byte AND KK
			0xD000 => {println!("DRW"); self.inst_dxyn(x, y, n); },	// DXYN - DRW - Draw n-byte sprite, start I at (V[X],V[Y]), V[F] = collision
			0xE000 => match opcode & 0xFF
			{	// Check lowest 8 bits
				0x9E => {println!("SKP"); self.inst_ex9e(x); },	// EX9E - SKP - Skip next instruction if key[V[X]] is pressed
				0xA1 => {println!("SKNP"); self.inst_exa1(x); },	// EXA1 - SKNP - Skip next instruction if key[V[X]] is not pressed
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
    /*
        TODO
        - include rust in c++-enviroment.
        - use sfml for graphical output and user-input.
        - extern "rust" not working as expected.
    */


    let mut cpu = CPU{  mem: [0; 0x1000], v: [0; 0x10], i: 0, dt: 0, st: 0, pc: 0, sp: 0,
                        stack: [0; 0x10], keyboard: [0; 0x10], display: [0; W*H / 8] };

    cpu.init();

    // XXX Program loop
    cpu.process_instruction();
	cpu.process_instruction();
	cpu.process_instruction();
}
