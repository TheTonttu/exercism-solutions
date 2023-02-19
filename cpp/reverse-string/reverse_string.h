#if !defined(REVERSE_STRING_H)
#define REVERSE_STRING_H

#include <string>

using namespace std;

namespace reverse_string {
	/// <summary>
	/// Creates reversed copy of the given string.
	/// </summary>
	/// <param name="str"></param>
	/// <returns>Reversed string copy.</returns>
	string reverse_string(string str);
}  // namespace reverse_string

#endif // REVERSE_STRING_H