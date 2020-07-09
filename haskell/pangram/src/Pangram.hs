module Pangram (isPangram) where

import qualified Data.Char as Char
import qualified Data.HashSet as HashSet

isPangram :: String -> Bool
isPangram text = 26 == HashSet.size (HashSet.fromList (keepLetters (stringToLower text)))

stringToLower :: String -> String
stringToLower = Prelude.map Char.toLower

keepLetters :: String -> String
keepLetters = filter Char.isAsciiLower
