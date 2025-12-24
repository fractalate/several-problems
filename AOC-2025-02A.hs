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

halfWord :: String -> (String, String)
halfWord text = splitAt tip text
  where tip = div (length text) 2

isFake :: Int -> Bool
isFake n = tip == antitip
  where (tip, antitip) = halfWord $ show n

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
