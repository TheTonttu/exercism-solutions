(ns cars-assemble)

(def base-production-rate 221.0)

(defn success-rate [speed]
  (cond
    (< speed 1) 0.0
    (<= speed 4) 1.0
    (<= speed 8) 0.9
    (= speed 9) 0.8
    (= speed 10) 0.77
    :else 0.0)
  )

(defn production-rate
  "Returns the assembly line's production rate per hour,
   taking into account its success rate"
  [speed]
  (* (success-rate speed)
     (* speed base-production-rate))
  )

(defn working-items
  "Calculates how many working cars are produced per minute"
  [speed]
  (int (/ (production-rate speed) 60))
  )
