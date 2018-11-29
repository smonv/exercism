module Acronym
    ( abbreviate
    )
where

import           Data.Char                      ( isUpper
                                                , toUpper
                                                , isAlpha
                                                )

abbreviate :: String -> String
abbreviate = concatMap convert . words . map replaceNonWorld

convert :: String -> String
convert []       = []
convert (x : xs) = toUpper x : rest
    where rest = if not $ isAcronym xs then filter isUpper xs else []

isAcronym :: String -> Bool
isAcronym = all isUpper

replaceNonWorld :: Char -> Char
replaceNonWorld '\'' = '\''
replaceNonWorld x | isAlpha x = x
                  | otherwise = ' '
