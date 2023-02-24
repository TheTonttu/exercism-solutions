#include "triangle.h"
#include <stdexcept>

namespace triangle {
	using namespace std;

	flavor kind(float a, float b, float c)
	{
		// All sides have to be of length > 0.
		if (a <= 0 || b <= 0 || c <= 0) {
			throw domain_error("All sides have to be greater than 0.");
		}

		// Sum of the lengths of any two sides must be greater than or equal to the length of the third side.
		if (a + b < c ||
			a + c < b ||
			b + c < a) {
			throw domain_error("Triangle inequality violation.");
		}

		if (a == b && a == c && b == c) {
			return flavor::equilateral;
		}

		if (a == b || a == c || b == c) {
			return flavor::isosceles;
		}

		return flavor::scalene;
	}
}  // namespace triangle
