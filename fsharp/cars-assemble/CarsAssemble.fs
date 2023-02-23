module CarsAssemble

let successRate (speed: int): float =
    match speed with
    | 0 -> 0.0
    | s when s >= 1 && s <= 4 -> 1.0
    | s when s >= 5 && s <= 8 -> 0.9
    | 9 -> 0.8
    | 10 -> 0.77
    | _ -> failwith $"Speed of {speed} is out of the question"

let productionRatePerHour (speed: int): float =
    let baseProductionPerHour = 221
    float (speed * baseProductionPerHour) * successRate(speed)

let workingItemsPerMinute (speed: int): int =
    int (productionRatePerHour speed / 60.0)
