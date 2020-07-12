module Pangram (isPangram) where

import qualified Data.Char as Char
import qualified Data.HashSet as HashSet
import           Data.HashSet (HashSet)

isPangram :: String -> Bool
isPangram text = HashSet.null (foldr deleteLetters lowerAlpha text)

deleteLetters :: Char -> HashSet Char -> HashSet Char
deleteLetters letter set = HashSet.delete (Char.toLower letter) set

lowerAlpha :: HashSet Char
lowerAlpha = HashSet.fromList ['a'..'z']
