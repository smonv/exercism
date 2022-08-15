module LeapYear (isLeapYear) where

isLeapYear :: Integer -> Bool
isLeapYear year =
  (i == 0) && (x /= 0 || (y == 0))
  where
    i = year `mod` 4
    x = year `mod` 100
    y = year `mod` 400
