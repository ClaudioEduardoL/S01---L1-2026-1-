package main

import (
	"bufio"
	"fmt"
	"os"
	"strings"
)

// ValidarUsuario verifica se o nome de usuário é válido.
func ValidarUsuario(nome string) (bool, string) {
	if len(nome) >= 8 {
		return true, "Usuário criado com sucesso!"
	}
	return false, "Erro: O nome de usuário é muito curto"
}

func main() {
	scanner := bufio.NewScanner(os.Stdin)

	for {
		fmt.Print("Digite um nome de usuário: ")
		scanner.Scan()
		nome := strings.TrimSpace(scanner.Text())

		valido, mensagem := ValidarUsuario(nome)
		fmt.Println(mensagem)

		if valido {
			break
		}
	}
}