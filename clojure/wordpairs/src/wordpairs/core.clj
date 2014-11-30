(ns wordpairs.core
  (:require [clojure.string :as string]))

(defn wp
  "I don't do a whole lot."
  [& args]
  (let [filename (first args)
              lines (string/split (slurp filename) #"\n")
              pairs (map #(string/split % #" ") lines)
              result (group-by first pairs)]
          nil))

(defn wp-her-opt
  "count word pairs starting with \"her\", using a file read buffer."
  [& args]
  (with-open [wp-buffer (clojure.java.io/reader (first args))]
          (println (count (get (group-by first (map #(string/split % #" ")
                                                    (line-seq wp-buffer)))
                               "her")))))
(defn wp-opt-java
  "processing the word pairs, using a file read buffer."
  [& args]
  (with-open [wp-buffer (clojure.java.io/reader (first args))]
          (let [result (group-by first
                                 (map #(let [a (.split % " ")]
                                         [(aget a 0) (aget a 1)])
                                      (line-seq wp-buffer)))]
            nil)))

(defn wp-opt
  "processing the word pairs, using a file read buffer."
  [& args]
  (with-open [wp-buffer (clojure.java.io/reader (first args))]
          (let [result (group-by first
                                 (map #(string/split % #" ")
                                      (line-seq wp-buffer)))]
            nil)))

