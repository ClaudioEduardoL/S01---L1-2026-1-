#include <iostream>
using namespace std;

int main() {
    int sala[5][5] = {0}; // todos os assentos começam livres
    int opcao = 0;
    int fila, coluna;
    int ocupados = 0;

    while (opcao != 3) {
        cout << "\n=== MENU PRINCIPAL ===\n";
        cout << "1. Reservar Assento\n";
        cout << "2. Ver Mapa da Sala\n";
        cout << "3. Sair\n";
        cout << "Escolha uma opcao: ";
        cin >> opcao;

        if (opcao == 1) {
            cout << "Informe a fila (0-4): ";
            cin >> fila;
            cout << "Informe a coluna (0-4): ";
            cin >> coluna;

            if (fila < 0 || fila > 4 || coluna < 0 || coluna > 4) {
                cout << "Erro: posicao invalida!\n";
            } else if (sala[fila][coluna] == 0) {
                sala[fila][coluna] = 1;
                cout << "Sucesso!\n";
            } else {
                cout << "Erro: Assento ocupado!\n";
            }
        }
        else if (opcao == 2) {
            cout << "\n=== MAPA DA SALA ===\n";
            for (int i = 0; i < 5; i++) {
                for (int j = 0; j < 5; j++) {
                    cout << "[" << sala[i][j] << "] ";
                }
                cout << endl;
            }
        }
        else if (opcao == 3) {
            cout << "Encerrando o menu...\n";
        }
        else {
            cout << "Opcao invalida!\n";
        }
    }

    // Relatório final
    for (int i = 0; i < 5; i++) {
        for (int j = 0; j < 5; j++) {
            if (sala[i][j] == 1) {
                ocupados++;
            }
        }
    }

    cout << "\n=== RELATORIO FINAL ===\n";
    cout << "Quantidade total de assentos ocupados: " << ocupados << endl;

    return 0;
}