#include <string>

namespace log_line {
	const std::string separator = ": ";

	std::string message(std::string line) {
		size_t separatorIdx = line.find(separator);
		return separatorIdx == std::string::npos
			? ""
			: line.substr(separatorIdx + separator.length());
	}

	std::string log_level(std::string line) {
		size_t separatorIdx = line.find(separator);
		return separatorIdx == std::string::npos
			? ""
			: line.substr(1, separatorIdx - 2);
	}

	std::string reformat(std::string line) {
		return message(line) + " (" + log_level(line) + ")";
	}
} // namespace log_line
