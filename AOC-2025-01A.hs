type State = Int

data Direction = L | R
  deriving Show

type Move = (Direction, State)

type Problem = [Move]

dialSize = 100

initialState :: State
initialState = 50

turn :: [State] -> Move -> [State]
turn [] move = turn [0] move
turn (state:states) (dir, count) = mod value dialSize : state : states
  where
    value = case dir of
      L -> state - count
      R -> state + count

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
solveProblem problem = length $
  filter (== 0) $
    foldl turn [initialState] problem

main :: IO ()
main = do
  problem <- readProblem
  let solution = solveProblem problem
  print solution
