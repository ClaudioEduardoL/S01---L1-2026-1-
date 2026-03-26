package main

import "fmt"

func main() {
	var a, b, c int

	fmt.Print("Digite os três lados do triângulo: ")
	fmt.Scan(&a, &b, &c)

	// Verifica se forma um triângulo
	if a <= 0 || b <= 0 || c <= 0 || a+b <= c || a+c <= b || b+c <= a {
		fmt.Println("Erro: os valores informados não formam um triângulo.")
		return
	}

	// Classificação com switch
	switch {
	case a == b && b == c:
		fmt.Println("Triângulo Equilátero")
	case a == b || a == c || b == c:
		fmt.Println("Triângulo Isósceles")
	default:
		fmt.Println("Triângulo Escaleno")
	}
}