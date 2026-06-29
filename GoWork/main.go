package main

import (
	"fmt"
	"time"
)



func number() {
	for i:= 1 ; i <= 5 ; i++ {
		fmt.Println("Number: ",i)
		time.Sleep(500 * time.Millisecond)
	}
}


func alphabets() {
	for ch:= 'A' ; ch <= 'E' ; ch++ {
		fmt.Printf("Lettr  : %c\n" , ch)
		time.Sleep(500 * time.Millisecond)
	}
}

func main() {
	go number()
	go alphabets()

	time.Sleep(3 * time.Second)
}