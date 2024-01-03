
mod atividades;

fn main() {
 let a = 12_i32;
 let b = 13_i32;
 let c = soma(a,b);
 println!("{}",c);
 let d = subtracao(a,b);
 println!("{}",d);
 let e = soma(subtracao(a,b),soma(a,b));
 println!("{}",e);
}

fn soma(a: i32 , b: i32) -> i32{
  a + b
}

fn subtracao(a: i32 , b: i32)-> i32{
 a - b
}
