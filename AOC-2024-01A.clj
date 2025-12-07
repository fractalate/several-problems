
(def input (slurp *in*))
(def lines (clojure.string/split-lines input))
(def pairs (map (fn [line] (clojure.string/split line #"\s+")) lines))

(def lefts (sort (map (fn [pair] (Integer/parseInt (nth pair 0))) pairs)))
(def rights (sort (map (fn [pair] (Integer/parseInt (nth pair 1))) pairs)))
(def diffs (map (fn [a b] (abs (- a b))) lefts rights))
(def total (reduce + diffs))

(println total)
