use  std::io;

pub fn questao4(){
    println!("Digite um numero inteiro : ");
    let mut input = String::new(); 
    io::stdin().read_line(&mut input).expect("Falha ao ler");
    let valor = verifica_inteiro(input);
    println!("{}",valor);

}


pub fn verifica_inteiro(input :String)-> String{
    if input.trim().chars().all(char::is_numeric) {
        let numero = input.trim().parse::<i32>().unwrap();
        numero.to_string()
    } else {
        "Valor não é um número inteiro".to_string()
    }
}




