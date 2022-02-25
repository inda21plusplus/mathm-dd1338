-- https://codeforces.com/contest/806/problem/E

import Data.List

main :: IO ()
main = interact $ unlines . map show . solveSlow . map read . tail . words

solveSlow :: [Int] -> [Int]
solveSlow = reverse . solve' . reverse
            where solve' [] = []
                  solve' xs = (foldl f 0 $ sort xs) : (solve' $ tail xs)
                  f a b | b > a     = a + 1
                        | b < a     = a - 1
                        | otherwise = a
