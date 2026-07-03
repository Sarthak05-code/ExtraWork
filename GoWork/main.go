package main

import "fmt"

func divider(a , b int) (int , error) {
	if b == 0 {
		return 0,fmt.Errorf("Cannot divide by zero")
	}
	return a / b , nil
}


func main() {
	a , b := 10 , 10
	answer , err := divider(a ,b)
	if err != nil {
		fmt.Println("Error : " , err)
		return
	}
	fmt.Printf("The value is : %d" , answer)
	
}