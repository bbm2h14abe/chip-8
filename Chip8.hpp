#pragma once
#include <algorithm> // std::fill
#include "Types.hpp" // ch8::u8, ch8::u16

namespace ch8
{
	class Chip8
	{
		u8 mem[0x1000];	// 4KB RAM

		u8 regV[16];	// 16 general purpose 8 bit registers (v[0xF] = flag register).
		u16 regI;		// 16 bit register, used to save addresses.

		u16 pc;			// Program-counter.

		u16 stack[16];	// Stack.
		u8 sp;			// Points to the top of the stack.

	public:
		Chip8();

		void initialize();
		void executeInstruction();
	};
}
