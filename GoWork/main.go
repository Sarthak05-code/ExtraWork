package main

import (
	"fmt"
	"strings" 
)

type Info struct {
	name      string
	age       int
	bloodType string
	gender    string
}

func main() {
	var person Info

	fmt.Println("Enter your name:")
	fmt.Scanln(&person.name)

	// --- Age Validation ---
	for {
		fmt.Println("Enter your age:")
		// Capture errors in case they type a letter instead of a number
		_, err := fmt.Scanln(&person.age) 
		
		if err == nil && person.age > 0 && person.age < 130 {
			break // Age is valid, exit the loop
		}
		fmt.Println("Invalid age! Please enter a realistic number.")
		
		// If they typed letters, we need to clear the invalid input
		if err != nil {
			var discard string
			fmt.Scanln(&discard)
		}
	}

	fmt.Println("Enter your bloodType:")
	fmt.Scanln(&person.bloodType)

	// --- Gender Validation ---
	for {
		fmt.Println("Enter your Gender (M/F):")
		fmt.Scanln(&person.gender)
		
		// Convert whatever they typed to uppercase
		person.gender = strings.ToUpper(person.gender) 

		if person.gender == "M" || person.gender == "F" {
			break // Input is valid, exit the loop
		}
		
		// If it wasn't M or F, this message prints and the loop starts over
		fmt.Println("Invalid input! Please type exactly 'M' or 'F'.")
	}

	fmt.Printf("\n--- User Profile ---\n")
	fmt.Printf("Name: %v\nAge: %d\nBlood Group: %v\nGender: %v\n", person.name, person.age, person.bloodType, person.gender)
}