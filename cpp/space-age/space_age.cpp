#include "space_age.h"

namespace space_age {
	constexpr int64_t seconds_in_earth_year{ 31'557'600 };

	space_age::space_age(int64_t seconds)
	{
		m_age_in_seconds = seconds;
	}

	int64_t space_age::seconds() const
	{
		return m_age_in_seconds;
	}

	double space_age::on_earth() const
	{
		constexpr long double seconds_in_year{ seconds_in_earth_year };
		return m_age_in_seconds / seconds_in_year;
	}

	double space_age::on_jupiter() const
	{
		constexpr double orbital_period{ 11.862615 };
		constexpr double seconds_in_year { seconds_in_earth_year * orbital_period };
		return m_age_in_seconds / seconds_in_year;
	}

	double space_age::on_mars() const
	{
		constexpr double orbital_period{ 1.8808158 };
		constexpr double seconds_in_year{ seconds_in_earth_year * orbital_period };
		return m_age_in_seconds / seconds_in_year;
	}

	double space_age::on_mercury() const
	{
		constexpr double orbital_period{ 0.2408467 };
		constexpr double seconds_in_year{ seconds_in_earth_year * orbital_period };
		return m_age_in_seconds / seconds_in_year;
	}

	double space_age::on_neptune() const
	{
		constexpr double orbital_period{ 164.79132 };
		constexpr double seconds_in_year{ seconds_in_earth_year * orbital_period };
		return m_age_in_seconds / seconds_in_year;
	}

	double space_age::on_saturn() const
	{
		constexpr double orbital_period{ 29.447498 };
		constexpr double seconds_in_year{ seconds_in_earth_year * orbital_period };
		return m_age_in_seconds / seconds_in_year;
	}

	double space_age::on_uranus() const
	{
		constexpr double orbital_period{ 84.016846 };
		constexpr double seconds_in_year{ seconds_in_earth_year * orbital_period };
		return m_age_in_seconds / seconds_in_year;
	}

	double space_age::on_venus() const
	{
		constexpr double orbital_period{ 0.61519726 };
		constexpr double seconds_in_year{ seconds_in_earth_year * orbital_period };
		return m_age_in_seconds / seconds_in_year;
	}
}  // namespace space_age
