-- Fake NP
-- http://codeforces.com/problemset/problem/805/A

-- Tavak and Seyyed are good friends.
-- Seyyed is very funny and he told Tavak to solve the following problem instead of longest-path.

-- You are given l and r. For all integers from l to r, inclusive, we wrote down all of their integer divisors except 1.
-- Find the integer that we wrote down the maximum number of times.

-- Solve the problem to show that it's not a NP problem.

-- Input: The first line contains two integers l and r (2 ≤ l ≤ r ≤ 109).
-- Output: Print single integer, the integer that appears maximum number of times in the divisors.
-- If there are multiple answers, print any of them.

-- Example 1
-- Input: 19 29
-- Output: 2

-- Example 2
-- Input: 3 6
-- Output: 3

import Data.List
import Data.Ord
import Data.Char (digitToInt)

main = do
    line <- getLine
    let params = map (\s->read s::Int) $ words line
    putStr $ show $ findMostOftenDivider (params!!0) (params!!1)

-- NP full solution
findMostOftenDivider :: Int -> Int -> Int
findMostOftenDivider l r = snd $
    foldl1 max $                                                            -- get a tuple with maximum first element
    map (\all@(firstEl:restEl)->(length all, firstEl)) $                    -- costruct tuples (occurrences, divider)
    group . sort $                                                          -- sort and group dividers
    [n1 | n1 <- [l..r]] >>= (\el -> [n2 | n2 <- [2..el], el `mod` n2 == 0]) -- get all dividers

-- To make things clear:
-- 1) group $ sort [5,3,6,9,1,5,1,2,1]
--    [[1,1,1],[2],[3],[5,5],[6],[9]]
-- 2) map (\all@(firstEl:restEl)->(length all, firstEl)) [[1,1,1],[2],[3],[5,5],[6],[9]]
--    [(3,1),(1,2),(1,3),(2,5),(1,6),(1,9)]
-- 3) foldl1 max [(3,1),(1,2),(1,3),(2,5),(1,6),(1,9)]
--    (3,1)


