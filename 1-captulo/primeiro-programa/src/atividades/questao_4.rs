/*use  std::io;

pub fn questao4()->i32{
    println!("digite um numero inteiro : ");
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).expect("falha ao ler");
    let numero: i32 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            println!("Erro: Por favor, insira um número inteiro válido.");
            return -1;
        }
    };
    return numero;
}*/