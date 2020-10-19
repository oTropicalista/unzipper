mod acao;

use std::env;

fn main() {
    //Pega a flag do comando
    let args: Vec<_> = env::args().collect();
    if args.len() < 2 {
        println!("Usage: dante -e <filename>");
        return;
    }
    
    if args[1] == "-e" {
        acao::extrair();
    } else if args[1] == "-z" {
        let filename = &*args[2];
        match acao::zipar(filename) {
            Ok(_) => println!("Zipado para {}", args[2]),
            Err(e) => println!("Erro: {:?}", e),
        }
    } else {
        println!("Ate o momento apenas extraimos arquivos. Agradecemos pela compreensao.");
        return;
    }
}
