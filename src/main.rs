use std::cmp::Ordering;
use std::io;

use rand::Rng;

fn main() {
    println!("Jogo de adivinhar");

    let secret_num = rand::thread_rng().gen_range(1..=100);

    loop {
        println!("Tente adivinhar o numero de 0 a 100");

        let mut tentativa: String = String::new();

        io::stdin()
            .read_line(&mut tentativa)
            .expect("Erro ao ler linha");

        let tentativa: u32 = match tentativa.trim().parse() {
            Ok(num) => num,
            Err(_) => continue,
        };

        println!("Voce tentou {}", tentativa);

        match tentativa.cmp(&secret_num) {
            Ordering::Less => println!("Mais alto"),
            Ordering::Greater => println!("Mais baixo"),
            Ordering::Equal => {
                println!("Parabens, voce acertou o numero!!!");
                break;
            }
        }
    }
}
