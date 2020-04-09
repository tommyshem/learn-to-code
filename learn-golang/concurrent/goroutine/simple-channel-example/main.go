package main

import (
	"fmt"
)

// simple function using channels to pass message back from go routine
func prod(v1 int, v2 int, c chan int) {
	c <- v1 * v2 // pass message though channel
}

func main() {
	c := make(chan int) // make channel for go routines to pass messages
	go prod(1, 2, c)    // pass in chanel to use
	go prod(3, 4, c)    // pass in chanel to use
	// blocks until message is received
	a := <-c // message from go routine
	// blocks until message is received
	b := <-c // message from go routine
	// print results
	fmt.Println("a * b = ", a*b)
}
