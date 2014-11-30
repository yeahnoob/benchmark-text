# wordpairs
Multiple programming language word pairs benchmark

## Usage
```
$ lein run -m wordpairs.core/wp-her-opt wordpairs.txt
```
Or in lein repl
```
user=> (dotimes [_ 10] (time (wordpairs.core/wp "wordpairs.txt")))
"Elapsed time: 318.96977 msecs"
"Elapsed time: 294.977724 msecs"
"Elapsed time: 354.231497 msecs"
"Elapsed time: 287.591107 msecs"
"Elapsed time: 286.470695 msecs"
"Elapsed time: 287.648544 msecs"
"Elapsed time: 286.513015 msecs"
"Elapsed time: 313.978012 msecs"
"Elapsed time: 286.1419 msecs"
"Elapsed time: 289.825509 msecs"
nil
user=> (dotimes [_ 10] (time (wordpairs.core/wp-opt "wordpairs.txt")))
"Elapsed time: 225.584117 msecs"
"Elapsed time: 232.012549 msecs"
"Elapsed time: 246.286792 msecs"
"Elapsed time: 217.975768 msecs"
"Elapsed time: 216.913982 msecs"
"Elapsed time: 216.133158 msecs"
"Elapsed time: 217.228052 msecs"
"Elapsed time: 228.407519 msecs"
"Elapsed time: 217.076183 msecs"
"Elapsed time: 217.939893 msecs"
nil
user=> (dotimes [_ 10] (time (wordpairs.core/wp-her-opt "wordpairs.txt")))
1036
"Elapsed time: 223.376115 msecs"
1036
"Elapsed time: 215.184385 msecs"
1036
"Elapsed time: 235.357932 msecs"
1036
"Elapsed time: 216.230703 msecs"
1036
"Elapsed time: 213.888681 msecs"
1036
"Elapsed time: 214.466944 msecs"
1036
"Elapsed time: 215.554332 msecs"
1036
"Elapsed time: 225.519872 msecs"
1036
"Elapsed time: 214.941382 msecs"
1036
"Elapsed time: 214.278864 msecs"
nil
```
