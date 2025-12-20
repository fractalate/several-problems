type State = Int
type Move = Int
type Problem = [Move]

dialSize = 100

initialState :: State
initialState = 50

turn :: [(State, Int)] -> Move -> [(State, Int)]
turn ((state, count):states) amount = (newState, zeros) : (state, count) : states
  where
    newState = mod (state + amount) dialSize
    mainZeros = div (abs amount) dialSize
    extraZeros
      | state == 0 = if amount == 0 then 1 else 0
      | amount < 0 = if newState >= state || newState == 0 then 1 else 0
      | amount > 0 = if newState <= state then 1 else 0
      | otherwise = 0
    zeros = mainZeros + extraZeros

parseLine :: String -> Move
parseLine (c:cs)
  | c == 'L' = -read cs
  | c == 'R' = read cs
  | otherwise = error "invalid input line"

readProblem :: IO Problem
readProblem = do
  contents <- getContents
  let moves = map parseLine $ lines contents
  return moves

solveProblem :: Problem -> Int
solveProblem problem = sum $
  map snd $
    foldl turn [(initialState, 0)] problem

main :: IO ()
main = do
  problem <- readProblem
  let solution = solveProblem problem
  print solution

