import Data.List (sort)

to_ints :: [String] -> [Int]
to_ints ws = map read ws

parse_lists ls = map to_ints (map words ls)

left ws = map head ws
right ws = map head $ map tail ws

main :: IO ()
main = do
  content <- getContents
  let lists = parse_lists (lines content)
  let lefts = sort $ left lists
  let rights = sort $ right lists

  let diffs = map abs $ zipWith (-) lefts rights
  let total = sum diffs
  print total
