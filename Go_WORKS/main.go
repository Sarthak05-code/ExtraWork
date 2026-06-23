package main

import "fmt" 

func main() {
	var op  string
	var a , b int

	fmt.Println("Enter operation: ")
	fmt.Scan(&op)

	fmt.Println("Enter a and b: ")
	fmt.Scan(&a , &b)

	if op == "add" {
		fmt.Print("The sum is : " , adder(a,b))
	} else if op == "sub" {
		fmt.Print("The subtraction is : ", subractor(a ,b))
	} else {
		fmt.Print("The answer was unkown. ",op)
	}
}


func adder(a int , b int) int {
	return a + b
}

func subractor(a int  , b int) int {
	return  a - b
}

