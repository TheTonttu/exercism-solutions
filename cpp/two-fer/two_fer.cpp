#include "two_fer.h"
#include <string>

namespace two_fer
{
	std::string two_fer(std::string your_name)
	{
		if (your_name.empty()) {
			your_name = "you";
		}

		return "One for " + your_name + ", one for me.";
	}
} // namespace two_fer

