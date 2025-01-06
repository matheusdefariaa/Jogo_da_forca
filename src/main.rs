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

    println!("\nBem vindo ao jogo {jogador}");

    println!("Deseja começar o jogo (S/N)");

    io::stdin()
    .read_line(&mut opc)
    .expect("Erro ao ler entrada !");

    let opc: String = opc.trim().to_string().to_ascii_lowercase();

    match opc.as_str () {
        "s" => jogo_forca(&jogador),
        _ => {
            println!("--- Fim de Jogo! ---\nAdeus !");
            process::exit(1);
        }
    };

}

fn jogo_forca(nome: &str) {
    let palavra_secreta = String::from("casa");
    let mut nova_palavra: [char;4] = ['_';4];
    let mut vidas: i32 = 6;
    let mut letras_digitadas: Vec<String> = Vec::new();

    loop {
        // Pede ao jogador para digitar uma letra
        let mut letra = String::new();
        
        println!("Você ainda tem {vidas} vidas");
        println!("Por favor digite uma letra: ");
        io::stdin()
        .read_line(&mut letra)
        .expect("Erro ao ler letra");
        
        // Tira os espaços e quera de linha da letra
        let letra = letra.trim().to_string();

        // Verifica se foi digitado mais de um caractere
        if letra.len() > 1 {
            println!("Entrada Inválida !");
            continue;
        };
 
        // Verifica se a letra já foi digitada
        if letras_digitadas.contains(&letra) {
            println!("\n ⚠️ Letra já digitada ⚠️ \n");
            continue;
        }
        
        // Salva a letra digitada em um vetor de string
        letras_digitadas.push(letra.clone());

        // Verifica se a letra digitada existe na palavra secreta
        // Se existir coloca a letra no indice correto
        for (index,letras) in palavra_secreta.chars().enumerate() {
            if &letras.to_string() == &letra  {
                nova_palavra[index] = letras;
            }
        }
        
        // Imprimi na tela como a palavra está até o momento
        for elem in nova_palavra {
            print!("{elem}");
        }
        println!("\n");

        // Transforma o vetor de char em um string
        let nova_palavra_string: String = nova_palavra.iter().collect();
        
        // Verifica igualdade
        if nova_palavra_string == palavra_secreta {
            println!("Parabéns {nome}você venceu o jogo !");
            break;
        }

        vidas -= 1;

        if vidas == 0 {
            println!("Jogador: {nome}você perdeu o jogo !");
            break;
        } 
    }
}