-- A. Theatre Square
-- http://codeforces.com/problemset/problem/1/A?locale=en

-- Theatre Square in the capital city of Berland has a rectangular shape with the size n × m meters.
-- On the occasion of the city's anniversary, a decision was taken to pave the Square with square granite flagstones.
-- Each flagstone is of the size a × a.

-- What is the least number of flagstones needed to pave the Square?
-- It's allowed to cover the surface larger than the Theatre Square, but the Square has to be covered.
-- It's not allowed to break the flagstones. The sides of flagstones should be parallel to the sides of the Square.

-- Input: The input contains three positive integer numbers in the first line: n, m and a (1 ≤  n, m, a ≤ 10^9).
-- Output: Write the needed number of flagstones.

-- Examples
-- Input: 6 6 4
-- Output: 4

main = do
    line <- getLine
    let [w, h, s] = map (\s->read s::Integer) $ words line
    putStr $ show $ clacTiles w h s

-- Use Integer, not Int since codeforces uses huge numbers in some test.
clacTiles :: Integer -> Integer -> Integer -> Integer
clacTiles w h s = ceiling (fromIntegral w / fromIntegral s) * ceiling (fromIntegral h / fromIntegral s)

