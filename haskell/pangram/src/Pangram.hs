module Pangram (isPangram) where

import qualified Data.Char as Char
import qualified Data.HashSet as HashSet
import           Data.HashSet (HashSet)

isPangram :: String -> Bool
isPangram text = all (\letters -> letters `elem` map Char.toLower text) lowerAlpha

lowerAlpha :: HashSet Char
lowerAlpha = HashSet.fromList ['a'..'z']
