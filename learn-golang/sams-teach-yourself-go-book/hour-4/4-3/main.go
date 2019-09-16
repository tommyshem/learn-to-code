// Using a Variadic Function
//Page 46

package main

import (
	"fmt"
)

func sumNumbers(numbers ...int) int {
	total := 0
	for _, number := range numbers {
		total += number
	}
	return total
}

func main() {
	result := sumNumbers(1, 2, 3, 4, 5)
	fmt.Printf("The result is %v\n", result)
}
