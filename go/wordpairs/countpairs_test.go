package golangperf

import (
	"testing"
)

func BenchmarkCountpairs(b *testing.B) {
	var t int
	for i := 0; i < b.N; i++ {
		t, _ = Countpairs("her", "word-pairs.txt")
		if t == 0 {
			b.Errorf("Benchmark error")
		}
	}
}
