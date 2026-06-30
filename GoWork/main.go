package main

import (
	"fmt"
	"time"
)

func caller() {
	time.Sleep(2000 * time.Millisecond)
}

func main() {
	fmt.Println("Hello , ")
	caller()
	fmt.Print("...Sarthak")
	
	
	
}