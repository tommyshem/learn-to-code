package main

import (
	"fmt"
	"sync"
)

// function called in thread
func foo(wg *sync.WaitGroup) {
	defer wg.Done() // is all ways called at the end of the function
	fmt.Println("New routine ")

}

func main() {
	var wg sync.WaitGroup
	// number of goroutines to wait for, we only have 1
	wg.Add(1)
	// goroutine
	go foo(&wg)
	// main thread
	fmt.Println("Waiting for subroutine")
	// block main thread and wait for goroutine to finish
	wg.Wait()
	// main thread continues
	fmt.Println("Main Routine ")
}
