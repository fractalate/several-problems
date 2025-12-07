import Data.List (sort)

to_ints :: [String] -> [Int]
to_ints ws = map read ws

parse_lists ls = map to_ints (map words ls)

left ws = map head ws
right ws = map head $ map tail ws

count_head_then_tail :: Ord a => a -> [a] -> (Int, [a])
count_head_then_tail x [] = (0, [])
count_head_then_tail x ys =
  if x >= head ys then
    let (count, rest) = count_head_then_tail x (tail ys)
        increment = if x == head ys then 1 else 0
    in (count + increment, rest)
  else
    (0, ys)

calculate_similarity [] ys = 0
calculate_similarity (x:xs) ys = 
  let (count_x, rest_xs) = count_head_then_tail x xs
      (count_y, rest_ys) = count_head_then_tail x ys
  in (x * (count_x + 1) * count_y) + (calculate_similarity rest_xs rest_ys)

main :: IO ()
main = do
  content <- getContents
  let lists = parse_lists (lines content)
  let lefts = sort $ left lists
  let rights = sort $ right lists

  let score = calculate_similarity lefts rights
  print score
