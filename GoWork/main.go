package main

import "fmt"

func main() {
	
	var  m , n , o int
	fmt.Println("Enter the row and colums of matrix 1: ")
	fmt.Scanln(&m,&n)
	fmt.Println("Enter the row for matrix 2 : ")
	fmt.Scanln(&o)

	matrix_one := make([][]int , m)
	matrix_two := make([][]int , n)
	for i:= 0 ; i < m ; i++ {
		matrix_one[i] = make([]int , n)
	}
	for i := 0 ;i < n ; i++ {
		matrix_two[i] = make([]int , o)
	}

	result := make([][]int , m)
	for i:= 0 ; i < m ; i++ {
		result[i] = make([]int , o)
	}
	fmt.Println("Enter value are matrix A: ")
	for i := 0 ; i < m ; i++ {
		for j := 0 ; j < n ; j++ {
			fmt.Printf("A[%d][%d] : " , i , j)
			fmt.Scan(&matrix_one[i][j])
		}
	}
	fmt.Println("Enter value for matrix B: ")
	for i := 0 ; i < n ; i++ {
		for j := 0 ; j < o ; j++ {
			fmt.Printf("B[%d][%d] : " , i , j)
			fmt.Scan(&matrix_two[i][j])
		}
	}

	fmt.Println("The matrix multiplication values are")
	for i := 0 ; i < m ; i++ {
		for j := 0 ; j < o ; j++ {
			for k := 0 ; k < n ; k++ {
				result[i][j] += matrix_one[i][k] * matrix_two[k][j]
			}
		}
	}

	for i := 0 ; i < m ; i++ {
		for j := 0 ; j < o ; j++ {
			fmt.Printf("%d " , result[i][j])
		}
		fmt.Println()
	}

	
}

