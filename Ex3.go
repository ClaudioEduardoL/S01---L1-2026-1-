package main

import "fmt"

func fibonacci(n int) {
	a, b := 0, 1

	for i := 0; i < n; i++ {
		fmt.Printf("%d ", a)
		a, b = b, a+b
	}
	fmt.Println()
}

func main() {
	var n int

	fmt.Print("Quantos números da sequência de Fibonacci você quer? ")
	fmt.Scan(&n)

	if n <= 0 {
		fmt.Println("Erro: informe um número maior que 0.")
		return
	}

	fibonacci(n)
}