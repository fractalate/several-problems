type State = Int

data Direction = L | R
  deriving Show

type Move = (Direction, State)

type Problem = [Move]

dialSize = 100

initialState :: State
initialState = 50

turn :: [(State, Int)] -> Move -> [(State, Int)]
turn ((state, count):states) (dir, amount) = (newState, m1 + m2) : (state, count) : states
  where
    delta = case dir of
      L -> - (mod amount dialSize)
      R -> mod amount dialSize
    m1 = div amount dialSize
    m2 = case dir of
      L -> if state > 0 && state <= -delta then 1 else 0
      R -> if state >= dialSize - delta then 1 else 0
    newState = mod (state + delta) dialSize

parseLine :: String -> Move
parseLine (c:cs)
  | c == 'L' = (L, read cs)
  | c == 'R' = (R, read cs)
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

