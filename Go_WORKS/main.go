package main

import "fmt" 

func main() {
	var add = adder(1 , 2)
	fmt.Printf("The sum is : %d", add)
}


func adder(a int , b int) int {
	return a + b
}

