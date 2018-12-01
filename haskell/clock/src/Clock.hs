module Clock
    ( addDelta
    , fromHourMin
    , toString
    )
where

import           Text.Printf

data Clock = Clock { hour :: Int, minute :: Int } deriving (Eq, Show)

fromHourMin :: Int -> Int -> Clock
fromHourMin h m = Clock (convertHour (h + (m `div` 60))) (m `mod` 60)

toString :: Clock -> String
toString clock = printf "%.2d:%.2d" (hour clock) (minute clock)

addDelta :: Int -> Int -> Clock -> Clock
addDelta h m clock = fromHourMin (h + hour clock) (m + minute clock)

convertHour :: Int -> Int
convertHour h | h >= 24 || h <= -24 = h `mod` 24
              | h < 0 && h > -24    = h + 24
              | otherwise           = h
