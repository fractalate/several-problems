to_ints :: [String] -> [Int]
to_ints ws = map read ws

parse_lists ls = map to_ints (map words ls)

left ws = map head ws
right ws = map head $ map tail ws

calculate_similarity [] ys = 0
calculate_similarity (x:xs) ys = 
  let count = length $ filter (x ==) ys
  in x * count + (calculate_similarity xs ys)

main :: IO ()
main = do
  content <- getContents
  let lists = parse_lists (lines content)
  let lefts = left lists
  let rights = right lists

  let score = calculate_similarity lefts rights
  print score
