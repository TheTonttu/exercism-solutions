module CarsAssemble

let (|InRange|_|) low high x =
  if low <= x && x <= high then Some () else None

let successRate (speed: int): float =
    match speed with
    | 0 -> 0.0
    | InRange 1 4 -> 1.0
    | InRange 5 8 -> 0.9
    | 9 -> 0.8
    | 10 -> 0.77
    | _ -> failwith $"Speed of {speed} is out of the question"

let productionRatePerHour (speed: int): float =
    let baseProductionPerHour = 221
    float (speed * baseProductionPerHour) * successRate(speed)

let workingItemsPerMinute (speed: int): int =
    int (productionRatePerHour speed / 60.0)
