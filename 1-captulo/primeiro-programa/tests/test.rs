#[path = "../src/atividades/mod.rs"] mod atividades;

#[cfg(test)]
mod tests {
    use crate::atividades::questao_2::questao2;
    use crate::atividades::questao_3::questao3;
    use crate::atividades::questao_4::verifica_inteiro;

    #[test]
    fn teste_questao2() {
        let resultado = questao2();
        assert_eq!(resultado, 57);
    }
    #[test]
    fn teste_questao3() {
        let resultado = questao3();
        assert_eq!(resultado, 4);
    }

    #[test]
    fn test_questao4(){
        let test_1 = verifica_inteiro("1".to_string());
        assert_eq!(test_1,"1");
        let test_2 = verifica_inteiro("1a".to_string());
        assert_eq!(test_2,"Valor não é um número inteiro".to_string())
    }
}