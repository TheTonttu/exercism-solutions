module InterestIsInteresting

let interestRate (balance: decimal): single =
    match balance with
    | b when b < 0m -> 3.213f
    | b when b < 1000m -> 0.5f
    | b when b < 5000m -> 1.621f
    | _ -> 2.475f

let interestPercentage (balance: decimal): single =
    interestRate balance / 100.0f

let interest (balance: decimal): decimal =
   balance * decimal(interestPercentage balance)

let annualBalanceUpdate(balance: decimal): decimal =
   balance + interest balance

let amountToDonate(balance: decimal) (taxFreePercentage: float): int =
    let donationPercentage = 
        if balance > 0m
        then taxFreePercentage * 2.0 / 100.0 
        else 0.0
    int(balance * decimal(donationPercentage))
