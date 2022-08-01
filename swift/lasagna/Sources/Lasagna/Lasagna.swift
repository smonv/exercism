let expectedMinutesInOven = 40

// TODO: define the 'remainingMinutesInOven' function
func remainingMinutesInOven(elapsedMinutes x:Int)->Int {
    return expectedMinutesInOven - x
}

// TODO: define the 'preparationTimeInMinutes' function
func preparationTimeInMinutes(layers x:Int)-> Int {
    return x*2
}

// TODO: define the 'totalTimeInMinutes' function
func totalTimeInMinutes(layers x:Int, elapsedMinutes y:Int) -> Int {
    return preparationTimeInMinutes(layers: x) + y
}
