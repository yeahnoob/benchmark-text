//package main
package golangperf

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

func Countpairs(groupid string, filename string) (count int, err error) {
	count = 0

	result := make(map[string]([]string))

	//srcfile, errfile := os.Open("/home/yeahnoob/.gvm/pkgsets/go1.3/global/src/github.com/yeahnoob/golang-perf/word-pairs.txt")
	srcfile, errfile := os.Open(filename)
	if errfile != nil {
		fmt.Println(errfile)
		err = errfile
		return
	}

	scanner := bufio.NewScanner(srcfile)
	for scanner.Scan() {
		p := strings.Split(scanner.Text(), " ")
		result[p[0]] = append(result[p[0]], p[1])
	}
	if errscan := scanner.Err(); errscan != nil {
		fmt.Println(errscan)
		err = errscan
		return
	}

	errfile = srcfile.Close()
	if errfile != nil {
		fmt.Println(errfile)
		err = errfile
		return
	}

	count = len(result[groupid])
	/*
	   fmt.Println("Count of ", groupid, " = ", len(result[groupid]))
	*/

	return
}

/*
func main() {
	groupid, fname := os.Args[1], os.Args[2]
	count, err := countpairs(groupid, fname)
	if err != nil {
		fmt.Println(err)
		return
	}

	fmt.Println("Count of ", groupid, " = ", count)
	//fmt.Println("Count of \"her\" = ", len(mainresult["her"]))
	return
}
*/
