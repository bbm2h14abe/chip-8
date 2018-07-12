#pragma once
#include "Types.hpp" // ch8::u8, ch8::u16

namespace ch8
{
	struct Opcode
	{			
		const u16 BITS;			// Complete opcode.							XXXXXXXXXXXXXXXX

		const u16 NNN;			// nnn - The lowest 12 bits.				----XXXXXXXXXXXX
		const u8 N;				// n - The lowest 4 bits.					------------XXXX
		const u8 X;				// x - The lower 4 bits of the high byte.	----XXXX--------
		const u8 Y;				// y -  The upper 4 bits of the low byte.	--------XXXX----
		const u8 KK;			// kk - The lowest 8 bits.					--------XXXXXXXX
								
		const u8 HIGHEST4BITS; 	// The highest 4 bits.						XXXX------------
		const u8 LOWEST4BITS;	// The lowest 4 bits.						------------XXXX

		Opcode(const u16 bits) : 
			BITS(bits),							
			NNN(BITS & 0xFFF),					
			N(BITS & 0xF),						
			X((BITS >> 8) & 0xF),				
			Y((BITS >> 4) & 0xF),				
			KK(BITS & 0xFF),					
			HIGHEST4BITS((BITS >> 12) & 0xF),	
			LOWEST4BITS(BITS & 0xF)				
		{
		}
	};
}
