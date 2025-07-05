package main

import (
	"fmt"
)

func multiplesOf3Or5(belowThisNumber int) int {
	sum := 0

	for n := 1; n < belowThisNumber; n++ {
		if n%3 == 0 || n%5 == 0 {
			sum += n
		}
	}

	return sum
}

func main() {
	fmt.Println(multiplesOf3Or5(10))
	fmt.Println(multiplesOf3Or5(1000))
}
