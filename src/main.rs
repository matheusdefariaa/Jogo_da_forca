// Jogo da forca
// 05/01/2024
// Autor: Matheus de Faria

use std::{io,process};

fn main() {
    println!("--- Jogo da Forca ---");
    let mut jogador = String::new();
    let mut opc = String::new();


    println!("Digite seu nome: ");
    io::stdin()
    .read_line(&mut jogador)
    .expect("Erro ao ler entrada !");

    println!("Bem vindo ao jogo {jogador} !");

    println!("Deseja começar o jogo (S/N)");

    io::stdin()
    .read_line(&mut opc)
    .expect("Erro ao ler entrada !");

    let opc: String = opc.trim().to_string().to_ascii_lowercase();

    match opc.as_str () {
        "s" => jogo_forca(),
        _ => {
            println!("--- Fim de Jogo! ---\nAdeus !");
            process::exit(1);
        }
    };

}

fn jogo_forca() {
    let palavra_secreta = String::from("casa");
    let mut vidas: i32 = 6;
    let mut letra = String::new();

    println!("Por favor digite uma letra: ");
    io::stdin()
    .read_line(&mut letra)
    .expect("Erro ao ler letra");
}