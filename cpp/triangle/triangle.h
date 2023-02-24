#if !defined(TRIANGLE_H)
#define TRIANGLE_H

namespace triangle {
	/// <summary>
	/// Triangle flavors.
	/// </summary>
	enum class flavor {
		/// <summary>
		/// Triangle has all three sides the same length.
		/// </summary>
		equilateral,

		/// <summary>
		/// Triangle has at least two sides the same length.
		/// </summary>
		isosceles,

		/// <summary>
		/// Triangle has all sides of different lengths.
		/// </summary>
		scalene
	};

	flavor kind(float, float, float);
}  // namespace triangle

#endif // TRIANGLE_H