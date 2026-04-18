-- Mike and palindrome
-- http://codeforces.com/problemset/problem/798/A

-- Mike has a string s consisting of only lowercase English letters.
-- He wants to change exactly one character from the string so that the resulting one is a palindrome.

-- A palindrome is a string that reads the same backward as forward,
-- for example strings "z", "aaa", "aba", "abccba" are palindromes, but strings "codeforces", "reality", "ab" are not.

-- Input: The first and single line contains string s (1 ≤ |s| ≤ 15).
-- Output: Print "YES" (without quotes) if Mike can change exactly one character so that the resulting string is palindrome or "NO" (without quotes) otherwise. 

-- Example 1
-- Input: abccaa
-- Output: YES

-- Example 2
-- Input: abbcca
-- Output: NO

-- Example 2
-- Input: abcda
-- Output: YES

main = do
  line <- getLine
  let
    numberOfDiffChars = differentChars line
    -- Note that we want to know If we can get a palindrome after 1 symbol is changed.
    -- "abba" should result in false since after any symbol is changed, we don't have a palindrome anymore 
    result = if ((numberOfDiffChars == 1) || (numberOfDiffChars == 0 && (odd $ length line)))
      then "YES"
      else "NO"
  putStr result
  

differentChars :: [Char] -> Int
differentChars [] = 0
differentChars (x:[]) = 0
differentChars (xs) =
  let l = head xs
      r = last xs
      adding = if l == r then 0 else 1
  in adding + (differentChars $ (tail . init) xs)


