type Range = (Int, Int)
type Problem = [Range]

commaToSpace c = case c of
  ',' -> ' '
  _ -> c

splitLine :: String -> [String]
splitLine text = words $ map commaToSpace text

readRange :: String -> Range
readRange text = (read $ fst parts, read $ tail $ snd parts)
  where parts = break (== '-') text

readProblem :: IO Problem
readProblem = do
  contents <- getContents
  let lineStrings = lines contents
      rangeStrings = map splitLine lineStrings
      ranges = map readRange $ concat rangeStrings
  return ranges

isRepetition :: String -> String -> Bool
isRepetition needle ending = case modulus of
  0 -> (needle == ending) || needle == (take characters ending) && (isRepetition needle $ drop characters ending)
  _ -> False
  where characters = length needle
        modulus = mod (length ending) characters

isRepeatedCharacters :: String -> Int -> Bool
isRepeatedCharacters text characters = isRepetition needle ending
  where needle = take characters text
        ending = drop characters text

isFake :: Int -> Bool
isFake n = any (isRepeatedCharacters text) [1..max_characters]
  where text = show n
        max_characters = div (length text) 2

getFakes :: Range -> [Int]
getFakes (a, b) = filter isFake [a..b]

getAllFakes :: [Range] -> [Int]
getAllFakes [] = []
getAllFakes ranges =
  foldr (\range result -> result ++ getFakes range) [] ranges

main :: IO ()
main = do
  problem <- readProblem
  let solution = sum $ getAllFakes problem
  print solution
