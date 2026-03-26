package main

import "fmt"

func verificarLogin(usuario, senha string) bool {
	return usuario == "senha" && senha == "admin"
}

func main() {
	var usuario, senha string

	for {
		_, err := fmt.Scan(&usuario, &senha)
		if err != nil {
			fmt.Println("Erro de leitura de entrada.")
			return
		}

		if verificarLogin(usuario, senha) {
			fmt.Println("Login bem-sucedido!")
			break
		} else {
			fmt.Println("Usuário ou senha incorretos. Tente novamente.")
		}
	}
}