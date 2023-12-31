pub fn questao2()->i32{
    let x:i32 = 30 ;
    let y:i32 = 27;
    return x + y ;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn teste_questao2() {
        let resultado = questao2();
        assert_eq!(resultado, 57);
    }
}
