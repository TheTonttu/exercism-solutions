class SpaceAge:
    __SECONDS_IN_EARTH_YEAR = 31_557_600

    __years_on_earth: float

    def __init__(self, seconds: int):
        self.__years_on_earth = seconds / self.__SECONDS_IN_EARTH_YEAR

    def on_earth(self) -> float:
        return round(self.__years_on_earth, 2)

    def on_mercury(self) -> float:
        return round(self.__years_on_earth / 0.2408467, 2)

    def on_venus(self) -> float:
        return round(self.__years_on_earth / 0.61519726, 2)

    def on_mars(self) -> float:
        return round(self.__years_on_earth / 1.8808158, 2)

    def on_jupiter(self) -> float:
        return round(self.__years_on_earth / 11.862615, 2)

    def on_saturn(self) -> float:
        return round(self.__years_on_earth / 29.447498, 2)

    def on_uranus(self) -> float:
        return round(self.__years_on_earth / 84.016846, 2)

    def on_neptune(self) -> float:
        return round(self.__years_on_earth / 164.79132, 2)
