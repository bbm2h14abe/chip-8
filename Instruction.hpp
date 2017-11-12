#pragma once
#include <iostream>		// std::cout XXX
#include <functional>	// std::function<>
#include <string>		// std::string
#include "Opcode.hpp"	// ch8::Opcode

namespace ch8
{
	struct Instruction
	{
		const std::string MNEMONIC;
		const Opcode OPCODE;
		const std::function<const bool(Opcode op)> TEST;
		const std::function<void()> EXECUTE;

		Instruction(const std::string mnemonic, const Opcode opcode, const std::function<const bool(Opcode op)>& test, const std::function<void()>& execute) :
			MNEMONIC(mnemonic),
			OPCODE(opcode),
			TEST(test),
			EXECUTE(execute)	
		{
		}
	};
}