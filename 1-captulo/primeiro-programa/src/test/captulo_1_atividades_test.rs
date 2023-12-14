/*use std::io; 

pub fn  questao1(){
   println!("douglas bastos merencio")
}

pub fn questao2()->i32{
    let x:i32 = 30 ;
    let y:i32 = 27;
    return x + y ;
}

pub fn questao3()->i32{
    let a:i32 = 3;
    let b:i32 = 2;
    let c:i32 = 7;

    return (a+b+c)/3;
}
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
}

#[test]
fn teste_questao2() {
    let resultado = questao2();
    assert_eq!(resultado, 57);
}

#[test]
fn teste_questao3() {
    let resultado = questao3();
    assert_eq!(resultado, 4);
}*/
