
(def input (slurp *in*))
(def lines (clojure.string/split-lines input))
(def pairs (map (fn [line] (clojure.string/split line #"\s+")) lines))

(def lefts (sort (map (fn [pair] (Integer/parseInt (nth pair 0))) pairs)))
(def rights (sort (map (fn [pair] (Integer/parseInt (nth pair 1))) pairs)))

(defn count_and_trim [needle haystack]
  (if (empty? haystack)
    [0 haystack]
    (if (>= needle (first haystack))
      (let [
        impact (if (= needle (first haystack)) 1 0)
        [count remainder] (count_and_trim needle (rest haystack))
      ]
        [(+ count impact) remainder])
      [0 haystack]
    )
  )
)

(defn calculate_score [xs ys]
  (if (empty? xs)
    0
    (if (empty? ys)
      0
      (let [
        x (first xs)
        [count_xs remainder_xs] (count_and_trim x (rest xs))
        [count_ys remainder_ys] (count_and_trim x ys)
      ]
        (+
          (* x (inc count_xs) count_ys)
          (calculate_score remainder_xs remainder_ys)
        )
      )
    )
  )
)

(println (calculate_score lefts rights))
