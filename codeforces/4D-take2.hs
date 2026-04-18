{-

4D. Mysterious Present
http://codeforces.com/problemset/problem/4/D?locale=en

Peter decided to wish happy birthday to his friend from Australia and send him a card.
To make his present more mysterious, he decided to make a chain.
Chain here is such a sequence of envelopes A = {a1,  a2,  ...,  an},
where the width and the height of the i-th envelope is strictly higher than the width and the height of the (i  -  1)-th envelope respectively.
Chain size is the number of envelopes in the chain. 

Peter wants to make the chain of the maximum size from the envelopes he has, the chain should be such, that he'll be able to put a card into it.
The card fits into the chain if its width and height is lower than the width and the height of the smallest envelope in the chain respectively.
It's forbidden to turn the card and the envelopes.

Peter has very many envelopes and very little time, this hard task is entrusted to you.

Input:
The first line contains integers n, w, h (1  ≤ n ≤ 5000, 1 ≤ w,  h  ≤ 106) — amount of envelopes Peter has, the card width and height respectively.
Then there follow n lines, each of them contains two integer numbers wi and hi — width and height of the i-th envelope (1 ≤ wi,  hi ≤ 10^6).

Output:
In the first line print the maximum chain size.
In the second line print the numbers of the envelopes (separated by space), forming the required chain, starting with the number of the smallest envelope.
Remember, please, that the card should fit into the smallest envelope.
If the chain of maximum size is not unique, print any of the answers.
If the card does not fit into any of the envelopes, print number 0 in the single line.

Example 1
Input:
2 1 1
2 2
2 2
Output:
1
1 

Example 2
Input:
3 3 3
5 4
12 11
9 8
Output:
3
1 3 2 

-}

import Data.Function (on)
import Data.Ord (Ord)
import Data.List (filter, sort, length, maximumBy)
import Control.Monad (replicateM, zipWithM, mapM_)

type EnvelopeNumber = Int
type Width = Int
type Height = Int

-- |Envelop representation.
newtype Envelope = Envelope (EnvelopeNumber, Width, Height)  deriving (Show)

instance Eq Envelope where
  Envelope (n1, w1, h1) == Envelope (n2, w2, h2) = (Envelope (n1, w1, h1) `compare` Envelope (n2, w2, h2)) == EQ

instance Ord Envelope where
  Envelope (n1, w1, h1) `compare` Envelope (n2, w2, h2)
    | (w1 < w2) && (h1 < h2) = LT
    | (w1 > w2) && (h1 > h2) = GT
    | otherwise = EQ

canBeWrappedBy :: Envelope -> Envelope -> Bool
Envelope (n1, w1, h1) `canBeWrappedBy` Envelope (n2, w2, h2) = (w2 > w1) && (h2 > h1)
  

getNum :: Envelope -> EnvelopeNumber
getNum (Envelope (n, w, h)) = n

main = do
  line <- getLine -- read envelopes quantity and size of the postcard
  let [num, cardWidth, cardHeight] = map (\s->read s::Int) $ words line

  inputs <- (replicateM num getLine) -- read envelopes sizes
  let 
    envelopes = zipWith (\n [w, h] -> Envelope (n, w, h))        -- construct envelopes
      [1..]                                                      -- from indexes   
      (map (\arr -> map (\s -> read s::Int) $ words arr) inputs) -- and [width, height]

    -- filter out envelopes that are smaller than postcard
    filteredEnvelopes = filter ( > (Envelope (0, cardWidth, cardHeight))) envelopes

    selectedEnvelopes = foldr (\prv nxt -> if length nxt > length prv then nxt else prv ) [] $ buildChains $ sort filteredEnvelopes
  putStrLn $ show $ length selectedEnvelopes
  mapM_ (putStr . (++ " " ) . show . getNum) selectedEnvelopes

buildChains :: Ord a => [a] -> [[a]]
buildChains [] = []
buildChains (x:xs)
  | (length chain) > 0 = [chain] ++ buildChains xs
  | otherwise = buildChains xs
  where chain = buildChain x xs

buildChain :: Ord a => a -> [a] -> [a]
buildChain e [] = [e]
buildChain e all@(x:xs) = [e] ++ buildChain' e all
  where
    buildChain' e' [] = []
    buildChain' e' (x':xs')
      | e' < x'   = [x'] ++ buildChain' x' xs'
      | otherwise = buildChain' e' xs'


