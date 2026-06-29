package main

import "fmt"


func main() {
	fmt.Println("Enter your name : ")
	var name string
	fmt.Scanln(&name)

	if name != "" {
		fmt.Printf("Hello , %s" ,name)
	} else {
		fmt.Println("Name can't be empty")
	}
	
	

}