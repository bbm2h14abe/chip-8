#include <iostream>	// std::cin XXX
#include "Chip8.hpp"

int main()
{
	ch8::Chip8 interpreter;

	interpreter.executeInstruction(); // test
	interpreter.executeInstruction(); // test
	
	std::cin.get();// XXX
	return 0;
}
