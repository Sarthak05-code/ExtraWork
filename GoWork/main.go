package main

import "fmt"

func main() {
	var no_of_id int
	fmt.Print("Enter no of process : ")
	fmt.Scanln(&no_of_id)

	// Fix 1: Make a slice with 0 length but a capacity equal to no_of_id
	array := make([]int, 0, no_of_id)

	for i := 0; i < no_of_id; i++ {
		fmt.Printf("Enter your PID no %d resource: ", i+1)
		var process_id int

		// Fix 2: Actually scan the user input into process_id
		fmt.Scanln(&process_id)

		// Now append works perfectly!
		array = append(array, process_id)
	}
	total_resources := 0
	fmt.Print("Stored PIDs: ")
	// index is ignored (_), 'val' takes the actual integer value directly
	for _, val := range array {
		fmt.Printf("%d ", val)
		total_resources += val
	}
	fmt.Println()
	fmt.Printf("The total resources in the array is : %d", total_resources)
}
