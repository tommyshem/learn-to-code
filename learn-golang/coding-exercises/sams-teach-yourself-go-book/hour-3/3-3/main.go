// Assigning an Incorect Type to a Varaible
// Page 29
// Note this example will not compile

package main

import (
	"fmt"
)

func main() {
	var i int
	i = "one"
	fmt.Println(i)
}
