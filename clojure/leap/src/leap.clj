(ns leap)

(defn leap-year? [year]
  ;; on every year that is evenly divisible by 4
  ;; except every year that is evenly divisible by 100
  ;; unless the year is also evenly divisible by 400
  (or
    (and (= (mod year 4) 0) (not= (mod year 100) 0))
    (= (mod year 400) 0)
    )
  )