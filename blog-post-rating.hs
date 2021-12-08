import Data.List

main :: IO ()
main = interact $ unlines . map show . solve . map read . tail . words

solve :: [Int] -> [Int]
solve = reverse . solve' . reverse
           where solve' [] = []
                 solve' xs = (foldl f 0 $ sort xs) : (solve' $ tail xs)
                 f a b = if b > a then a + 1 else
                         if b < a then a - 1 else
                         a
