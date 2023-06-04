(ns log-levels)
(use '[clojure.string :only (index-of, lower-case, trim)])

(defn message
  "Takes a string representing a log line
   and returns its message with whitespace trimmed."
  [s]
  (let [separator (index-of s ":")]
    (if (<= 0 separator)
      (trim (subs s (+ 1 separator)))
      (trim s)))
  )

(defn log-level
  "Takes a string representing a log line
   and returns its level in lower-case."
  [s]
  (lower-case
    (let [start (index-of s "[")
          end (index-of s "]")]
      (if (and (<= 0 start) (<= 0 end))
        (subs s (+ 1 start) end)
        (s)
        )))
  )

(defn reformat
  "Takes a string representing a log line and formats it
   with the message first and the log level in parentheses."
  [s]
  (format "%s (%s)" (message s) (log-level s))
  )
