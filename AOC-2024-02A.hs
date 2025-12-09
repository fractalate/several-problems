import System.IO

data ReportClassification = Safe | Unsafe
  deriving Eq

to_int :: String -> Int
to_int x = read x

classify_report :: String -> ReportClassification
classify_report report =
  let sequence = map to_int (words report)
  in classify_report_sequence sequence

classify_report_sequence :: [Int] -> ReportClassification
classify_report_sequence sequence = 
  let differences = zipWith (-) sequence (tail sequence)
  in classify_report_based_on_differences differences

classify_report_based_on_differences :: [Int] -> ReportClassification
classify_report_based_on_differences differences = if (head differences) == 0
  then Unsafe
  else
    if (head differences) < 0
      then if (all (< 0) differences) && (all (>= -3) differences)
        then Safe
        else Unsafe
    else if (all (> 0) differences) && (all (<= 3) differences)
      then Safe
      else Unsafe

main :: IO ()
main = do
  contents <- getContents
  let result = length $ filter (== Safe) $ map classify_report (lines contents)
  putStrLn (show result)
