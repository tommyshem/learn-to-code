// Attempting to Change a Constant
//Page 38
// Note This example does not compile
package main

import (
	"fmt"
)

const greeting string = "Hello from GO"

func main() {
	greeting = "Goodbye from GO"
	fmt.Println(greeting)
}
