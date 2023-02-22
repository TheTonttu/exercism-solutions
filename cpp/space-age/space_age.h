#if !defined(SPACE_AGE_H)
#define SPACE_AGE_H

#include <cstdint>

namespace space_age {
	struct space_age {
	public:
		space_age(int64_t seconds);

		/// <summary>
		/// Gets the age in seconds.
		/// </summary>
		/// <returns>Age in seconds.</returns>
		int64_t seconds() const;

		/// <summary>
		/// Calculates age in Earth years.
		/// </summary>
		/// <returns>Age in earth years.</returns>
		double on_earth() const;

		/// <summary>
		/// Calculates age in Earth years.
		/// </summary>
		/// <returns>Age in Jupiter years.</returns>
		double on_jupiter() const;

		/// <summary>
		/// Calculates age in Mars years.
		/// </summary>
		/// <returns>Age in Mars years.</returns>
		double on_mars() const;

		/// <summary>
		/// Calculates age in Mercury years.
		/// </summary>
		/// <returns>Age in Mercury years.</returns>
		double on_mercury() const;

		/// <summary>
		/// Calculates age in Neptune years.
		/// </summary>
		/// <returns>Age in Neptune years.</returns>
		double on_neptune() const;

		/// <summary>
		/// Calculates age in Saturn years.
		/// </summary>
		/// <returns>Age in Saturn years.</returns>
		double on_saturn() const;

		/// <summary>
		/// Calculates age in Uranus years.
		/// </summary>
		/// <returns>Age in Uranus years.</returns>
		double on_uranus() const;

		/// <summary>
		/// Calculates age in Venus years.
		/// </summary>
		/// <returns>Age in Venus years.</returns>
		double on_venus() const;

	private:
		int64_t m_age_in_seconds;
	};
}  // namespace space_age

#endif // SPACE_AGE_H