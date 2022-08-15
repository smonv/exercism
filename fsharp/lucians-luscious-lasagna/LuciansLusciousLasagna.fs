module LuciansLusciousLasagna

let expectedMinutesInOven = 40

let remainingMinutesInOven x = expectedMinutesInOven - x

let preparationTimeInMinutes layers = 2 * layers

let elapsedTimeInMinutes layers elapsed =
    (preparationTimeInMinutes layers) + elapsed
