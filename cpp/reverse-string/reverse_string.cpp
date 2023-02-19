#include <algorithm>
#include "reverse_string.h"

namespace reverse_string {
	string reverse_string(string str) {
		string reverse_copy = str;
		std::reverse(reverse_copy.begin(), reverse_copy.end());
		return reverse_copy;
	}
}  // namespace reverse_string
