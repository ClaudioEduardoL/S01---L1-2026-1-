#include <iostream>
using namespace std;

int main() {
    float saldo, valor;
    int opcao;

    cout << "Informe o saldo inicial: R$ ";
    cin >> saldo;

    do {
        cout << "\n--- MENU ---\n";
        cout << "1. Ver Saldo\n";
        cout << "2. Depositar\n";
        cout << "3. Sacar\n";
        cout << "4. Sair\n";
        cout << "Escolha uma opcao: ";
        cin >> opcao;

        switch (opcao) {
            case 1:
                cout << "Saldo atual: R$ " << saldo << endl;
                break;

            case 2:
                cout << "Digite o valor do deposito: R$ ";
                cin >> valor;
                saldo += valor;
                cout << "Deposito realizado com sucesso.\n";
                break;

            case 3:
                cout << "Digite o valor do saque: R$ ";
                cin >> valor;
                if (valor > saldo) {
                    cout << "Saldo Insuficiente\n";
                } else {
                    saldo -= valor;
                    cout << "Saque realizado com sucesso.\n";
                }
                break;

            case 4:
                cout << "Encerrando o programa...\n";
                break;

            default:
                cout << "Opcao invalida.\n";
        }

    } while (opcao != 4);

    return 0;
}