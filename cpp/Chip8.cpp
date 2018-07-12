#include "Chip8.hpp"
using namespace ch8;

#include "Instruction.hpp"

Chip8::Chip8()
{
	initialize();
}

void Chip8::initialize()
{
	std::fill(mem, mem + 0x1000, 0);
	std::fill(regV, regV + 16, 0);
	regI = 0;
	pc = 0;
	std::fill(stack, stack + 16, 0);
	sp = 0;

	// 0x200 - Starting address of most chip8-programs.
	pc = 0x200;

	// TEST
	mem[pc] = 0x00;
	mem[pc + 1] = 0xEE;
}

void Chip8::executeInstruction()
{
	// Read opcode from memory and increment the program counter.
	u16 opcode = (mem[pc] << 8) | mem[pc + 1]; 
	pc += 2;

	// Create references (aliases) for the accessed registers.
	// u8& vx = _v[x], &vy = _v[y], &_vf = _v[0xF];

	const Instruction InstructionList[] =
	{
		Instruction("cls", Opcode(0x00E0), [](Opcode op){ return op.HIGHEST4BITS == 0x0 && op.NNN == 0xE0; }, /* Clear display	*/
					[](){ std::cout << "Cls"; }),		
		Instruction("ret", Opcode(0x00EE), [](Opcode op){ return op.HIGHEST4BITS == 0x0 && op.NNN == 0xEE; }, /* Set pc = stack[sp--]	*/
					[](){ std::cout << "Ret"; })	

					// ...continue here
	};

	// Loop through each instruction and test if it should be executed.
	for(unsigned i=0; i<2; ++i)
	{
		if(InstructionList[i].TEST(opcode))
		{
			InstructionList[i].EXECUTE();
			break;
		}	
	}
}
