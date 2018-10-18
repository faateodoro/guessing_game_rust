extern crate rand; // importa a crate rand

// adiciona a biblioteca std::in no programa
// senão, toda entrada de dados seria std::io::stdin()
use std::io;
use std::cmp::Ordering; // função para a comparação dos números
use rand::Rng;

fn main() {
    println!("Guessing the number!" );

    // aqui um numero aleatorio é gerado
    let secret_number = rand::thread_rng().gen_range(1, 101);

    // aqui começa o loopgame, que só é quebrado quando acerta o numero
    // e ativa o break; junto com a frase "You win!"
    loop {

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

        // converte a  string guess para inteiro (u32) e retorna um erro,
        // caso o valor informado não seja um numero.
        // trim() elimina qualquer espaço em branco.
        let guess: u32 = guess.trim().parse()
            .expect("Please, type a number!");

        println!("You guessed: {}", guess);

        // faz a comparação entre números
        match guess.cmp(&secret_number) {
            Ordering::Less => println!("Too small!"),
            Ordering::Greater => println!("Too big"),
            Ordering::Equal => {
                println!("You win!");
                // para o loop
                break;
            }
        }
    }
}
