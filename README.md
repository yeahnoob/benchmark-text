# benchmark-text

## Brief
a Benchmark, analysing the normal text, 
with Different programming language.

## Blog

### In Clojure
- Date 2014-11-29. Writing in Clojure, without any performance tweak. Function `user=> (wordpairs.core/wp "wordpairs.txt")`

    Date 2014-11-30. Made a little optimization. Function `user=> (wordpairs.core/wp-opt "wordpairs")`

### In Julialang
- Date 2014-11-30. Original from [jfsantos' gists](https://gist.github.com/jfsantos/8412953) . Copy to here and fix for the Julialang v0.3.3 & night-build version. Now I don't know how to tweak it, and I would take some time for it later.

### In Golang
- Date 2014-11-30. Copy my old Golang wordpairs to here. At a glance, the code seems optimizable. Benchmark it with `go test --bench .` .

### In C
- Date 2014-11-30. Original from [maxhutch/fast_text_C](https://github.com/maxhutch/fast_text_C). Use `gcc` or `mpicc` , with `-O3` CFLAGS.

## Result
On my i7-2600K PC with SSD storage, running Ubuntu 14.04.

|Programming Language | Versions & Options | Time Cost |
| ------- | ------- | ------- |
|Clojure| v1.6.0, openjdk-7u71-2.5.3 |about 300 ms |
|Julialang| v0.3.3 |about 200 ms |
|Golang | v1.3.3 |about 60 ms |
|C | gcc v4.8.2, openmpi v1.6.5, `-O3` | about 30 ms |
