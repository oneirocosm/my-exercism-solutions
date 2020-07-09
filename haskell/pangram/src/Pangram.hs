module Pangram (isPangram) where

import Data.Char (toLower)
import qualified Data.HashSet as HashSet

isPangram :: String -> Bool
isPangram text = 26 == HashSet.size (HashSet.fromList (keepLetters (stringToLower text)))

stringToLower :: String -> String
stringToLower = Prelude.map toLower

keepLetters :: String -> String
keepLetters = filter (`elem` "abcdefghijklmnopqrstuvwxyz")
