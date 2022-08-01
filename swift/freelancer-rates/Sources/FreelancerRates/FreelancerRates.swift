func dailyRateFrom(hourlyRate: Int) -> Double {
   return Double(hourlyRate*8)
}

func monthlyRateFrom(hourlyRate: Int, withDiscount discount: Double) -> Double {
    let monthlyRate = dailyRateFrom(hourlyRate:hourlyRate)*22
    let discountedMonthlyRate = monthlyRate*(discount/100)
    return Double(monthlyRate - discountedMonthlyRate).rounded(.toNearestOrEven)
}

func workdaysIn(budget: Double, hourlyRate: Int, withDiscount discount: Double) -> Double {
    let dailyRate = dailyRateFrom(hourlyRate: hourlyRate)
    let discountedDailyRate = dailyRate - dailyRate*(discount/100)
    return Double(budget/discountedDailyRate).rounded(.down)
}
