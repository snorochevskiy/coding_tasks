import Data.List

main = do
    numOfArgsLine <- getLine
    argsLine      <- getLine
    let numLen = (read numOfArgsLine::Int)
        args   = map (\s->read s::Int) $ words argsLine
        tree   = buildSumTree args
        res    = foldl (\acc i -> if (sumOf tree 0 i) == (sumOf tree (i+1) (numLen-1)) then (acc+1) else acc) 0 [0 .. (numLen-2)]
    putStrLn $ show res

-- Node  sum lIndex lIndex left right
-- Leaf sum lIndex rIndex left right
data SumTree = Node Int Int Int SumTree SumTree
             | Leaf Int Int Int Int Int
             | Empt
             deriving Show

-- tree and depth
buildSumTree :: [Int] -> SumTree
buildSumTree nums =
  let leaves = buildFirstLevel nums
  in buildLevels (buildFirstLevel nums 0 [])

-- level 
buildLevels :: [SumTree] -> SumTree
buildLevels (node:[]) = node
buildLevels nodes = buildLevels (buildLevel nodes [])

-- nums -> nextIndex -> acc -> result
buildFirstLevel :: [Int] -> Int -> [SumTree] -> [SumTree]
buildFirstLevel (a:b:xs) ind leaves = buildFirstLevel xs (ind+2) ((Leaf (a+b) ind (ind+1) a b):leaves)
buildFirstLevel (a:[]) ind leaves = reverse $ (Leaf a ind ind a 0):leaves
buildFirstLevel [] ind leaves = reverse leaves

buildLevel :: [SumTree] -> [SumTree] -> [SumTree]
buildLevel (a@(Leaf s1 lInd1 rInd1 _ _):b@(Leaf s2 lInd2 rInd2 _ _):xs) res = buildLevel xs ((Node (s1 + s2) lInd1 rInd2 a b):res)
buildLevel (a@(Leaf s1 lInd1 rInd1 _ _):[]) res = reverse $ (Node s1 lInd1 rInd1 a Empt):res
buildLevel (a@(Node s1 lInd1 rInd1 _ _):b@(Node s2 lInd2 rInd2 _ _):xs) res = buildLevel xs ((Node (s1 + s2) lInd1 rInd2 a b):res)
buildLevel (a@(Node s1 lInd1 rInd1 _ _):[]) res = reverse $ (Node s1 lInd1 rInd1 a Empt):res
buildLevel [] res = reverse res

--           1              2
--          / \            /
--        /     \         /
--       1       2       3
--      / \     / \     /
--     1   2   3   4   5
--    / \ / \ / \ / \ /
--    1 2 3 4 5 6 7 8 9

sumOf :: SumTree -> Int -> Int -> Int
sumOf (Node s lInd rInd  left@(Node s1 lInd1 rInd1 left1 right1) right@(Node s2 lInd2 rInd2 left2 right2)) start end =
  if start == lInd && end == rInd then s else
   if end <= rInd1  then sumOf left start end else
     if start >= lInd2 then sumOf right start end else
       (sumOf left start rInd1) + (sumOf right lInd2 end)
sumOf (Node s lInd rInd  left@(Node s1 lInd1 rInd1 left1 right1) Empt) start end =
  if start == lInd && end == rInd then s else sumOf left start end
sumOf (Node s lInd rInd  left@(Leaf s1 lInd1 rInd1 left1 right1) right@(Leaf s2 lInd2 rInd2 left2 right2)) start end =
  if start == lInd && end == rInd then s else
   if end <= rInd1  then sumOf left start end else
     if start >= lInd2 then sumOf right start end else
       (sumOf left start rInd1) + (sumOf right lInd2 end)
sumOf (Node s lInd rInd left@(Leaf s1 lInd1 rInd1 left1 right1) Empt) start end =
  if start == lInd && end == rInd then s else sumOf left start end
sumOf (Leaf s lInd rInd left right) start end =
  if start == lInd && end == rInd then s else
    if end == lInd then left else right
sumOf Empt start end = 0

