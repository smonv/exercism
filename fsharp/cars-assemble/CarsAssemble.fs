module CarsAssemble

let successRate (speed: int) : float =
    match speed with
    | 0 -> float 0
    | 1
    | 2
    | 3
    | 4 -> float 1
    | 5
    | 6
    | 7
    | 8 -> 0.9
    | 9 -> 0.8
    | 10 -> 0.77
    | _ -> float 0

let productionRatePerHour (speed: int) : float =
    (float speed) * float 221 * successRate (speed)

let workingItemsPerMinute (speed: int) : int =
    let itemsPerMinute = productionRatePerHour (speed) / 60.0
    System.Convert.ToInt32(System.Math.Floor(itemsPerMinute))
