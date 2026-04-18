-- B. Spreadsheets
-- http://codeforces.com/problemset/problem/1/B?locale=en

-- In the popular spreadsheets systems (for example, in Excel) the following numeration of columns is used.
-- The first column has number A, the second — number B, etc. till column 26 that is marked by Z.
-- Then there are two-letter numbers: column 27 has number AA, 28 — AB, column 52 is marked by AZ.
-- After ZZ there follow three-letter numbers, etc.

-- The rows are marked by integer numbers starting with 1.
-- The cell name is the concatenation of the column and the row numbers.
-- For example, BC23 is the name for the cell that is in column 55, row 23. 

-- Sometimes another numeration system is used: RXCY, where X and Y are integer numbers, showing the column and the row numbers respectfully.
-- For instance, R23C55 is the cell from the previous example.

-- Your task is to write a program that reads the given sequence of cell coordinates and produce each item written according to the rules of another numeration system.

-- Input: The first line of the input contains integer number n (1 ≤ n ≤ 10^5), the number of coordinates in the test.
-- Then there follow n lines, each of them contains coordinates.
-- All the coordinates are correct, there are no cells with the column and/or the row numbers larger than 10^6

-- Output: Write n lines, each line should contain a cell coordinates in the other numeration system.

-- Examples
-- Input:
-- 2
-- R23C55
-- BC23

-- Output:
-- BC23
-- R23C55

import Data.Char (ord)
import Control.Monad (replicateM, mapM_)
import Text.Regex.Posix ( (=~) ) -- doesn't work on codeforces :(


-- |Regular expression for R15C20 like format
regExpRowColFormat = "R([0-9]+)C([0-9]+)"

-- |Regular expression for Excel like column number format
regExpExcelFormat = "([A-Z]+)([0-9]+)"

-- |Represets cell in excel table: (row, column)
type Cell = (Int, Int)

-- |Prints cell coords in R15C20 format
toRowClumnFormat :: Cell -> String
toRowClumnFormat (r, c) = "R" ++ show r ++ "C" ++ show c

-- |Prints cell coords in Excel format
toExcelFormat :: Cell -> String
toExcelFormat (r, c) = convertToLiteral c ++ show r


main = do
  numberOfInputRows <- getLine
  inputs <- replicateM (read numberOfInputRows) getLine
  mapM_ putStrLn $ map transformToAnotherFormat inputs

-- |Transforms cell to opposite format: from R10C12 to Excel and vice versa
transformToAnotherFormat :: String -> String
transformToAnotherFormat str =
  if str =~ regExpRowColFormat :: Bool
    then toExcelFormat $ parseRowColumnCell str
    else toRowClumnFormat $ parseExcelCell str

-- |Parses cell coords from string with R2C2 format
parseRowColumnCell :: String -> Cell
parseRowColumnCell str =
  let (_, _, _, [row, col]) = str =~ regExpRowColFormat :: (String,String,String,[String])
  in ((read row :: Int), (read col :: Int))

-- |Parses cell coords from string with Excel format
parseExcelCell :: String -> Cell
parseExcelCell str =
  let (_, _, _, [col, row]) = str =~ regExpExcelFormat :: (String,String,String,[String])
  in ((read row :: Int), fromLiteralToDecimal col)

-- |Converts decimal number to Excel columns literal enumeration
convertToLiteral :: Int -> String
convertToLiteral i = convertToLiteral' i ""
  where
    convertToLiteral' :: Int -> String -> String
    convertToLiteral' 0 result = result
    convertToLiteral' i result = convertToLiteral' (i `div` 26) (decDigittoLiteral (i `mod` 26) ++ result)


-- |Returns corresponding latin character for a given 26-based digit
decDigittoLiteral :: Int -> String
decDigittoLiteral i = [ ( toEnum ( i + 64 ) ) :: Char ] -- 65 is code of "A" symbol in ASCII table

-- |Parses decimal number from Excel literal cilumns enumeration
fromLiteralToDecimal :: String -> Int
fromLiteralToDecimal str = fromLiteralToDecimal' str 0
  where
    fromLiteralToDecimal' :: String -> Int -> Int
    fromLiteralToDecimal' [] acc = acc
    fromLiteralToDecimal' (x:xs) acc = fromLiteralToDecimal' xs (acc * 26 + fromLiteralToDecimalDigit x)

    fromLiteralToDecimalDigit :: Char -> Int
    fromLiteralToDecimalDigit c = ord c - ord 'A' + 1

