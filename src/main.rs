// adiciona a biblioteca std::in no programa
// senão, toda entrada de dados seria std::io::stdin()
use std::io;

fn main() {
    println!("Guessing the number!" );

    println!("Please input your guess.");

    // Cria a variavel mutavel guess e adiciona
    // uma instancia vazia do metodo estático String
    let mut guess = String::new();

    // io::stdin() é uma função padrão de entrada de dados
    // .read_line(&mut guess) entrada de dados com o teclado
    // que passa o valor como referencia para guess.
    // .expect("") é continuação da mesma linha, mas foi quebrada para
    // uma melhor visualização. 
    // .expect("") recebe um Ok ou um Err ou está tudo certo,
    // ou o programa dá um crash (Panic).
    io::stdin().read_line(&mut guess)
        .expect("Failed to read line.");

    println!("You guessed: {}", guess);
}
