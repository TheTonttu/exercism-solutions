module LuciansLusciousLasagna

let expectedMinutesInOven = 40

let remainingMinutesInOven minutesInOven = expectedMinutesInOven - minutesInOven

let preparationTimeInMinutes numberOfLayers =
    let minutesPerLayer = 2
    minutesPerLayer * numberOfLayers

let elapsedTimeInMinutes numberOfLayers minutesInOven =
    preparationTimeInMinutes numberOfLayers + minutesInOven