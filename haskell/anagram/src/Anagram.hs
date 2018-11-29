module Anagram
    ( anagramsFor
    )
where

import           Data.Char                      ( toLower )
import           Data.List                      ( sort )

anagramsFor :: String -> [String] -> [String]
anagramsFor word = filter (isAnagram word)

toLower' :: String -> String
toLower' = map toLower

isAnagram :: String -> String -> Bool
isAnagram x xs = x' /= xs' && sort x' == sort xs'
  where
    x'  = toLower' x
    xs' = toLower' xs
