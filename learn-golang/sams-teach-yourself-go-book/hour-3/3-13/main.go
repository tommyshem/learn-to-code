// Pasing Variables as Pointers
//Page 37

package main

import (
	"fmt"
)

func showMemoryAddress(x *int) {
	fmt.Println(x)
	// to access the variable then add asterisk in front of the reference.
	fmt.Println(*x)
	return
}

func main() {
	i := 1
	fmt.Println(&i)
	showMemoryAddress(&i)
}
