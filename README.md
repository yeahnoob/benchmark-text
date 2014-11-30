benchmark-text
==============

Brief
==============
a Benchmark, analysing the normal text, 
with Different programming language.

Blog
==============
- <h4>In Clojure</h4>
    Date 2014-11-29. Writing in Clojure, without any performance tweak. Function `user=> (wordpairs.core/wp "wordpairs.txt")`

    Date 2014-11-30. Made a little optimization. Function `user=> (wordpairs.core/wp-opt "wordpairs")`
- <h4>In Julialang</h4>
    Date 2014-11-30. Original from https://gist.github.com/jfsantos/8412953. Fix for the Julialang v0.3.3 & night-build version. Now I dont know how to tweak it, and I would take some time for it later.
- <h4>In Golang</h4>
    Date 2014-11-30. Copy my old Golang wordpairs to here. At a glance, the code seems optimizable. Benchmark it with `go test --bench .` .
