module LuciansLusciousLasagna

type NumberOfLayers = int
type Minutes = int

let expectedMinutesInOven : Minutes = 40

let remainingMinutesInOven timeInOven = expectedMinutesInOven - timeInOven

let preparationTimeInMinutes (layers: NumberOfLayers) : Minutes =
    let prepTimePerLayer : Minutes = 2
    prepTimePerLayer * layers

let elapsedTimeInMinutes (layers: NumberOfLayers) (timeInOven: Minutes) : Minutes =
    preparationTimeInMinutes layers + timeInOven