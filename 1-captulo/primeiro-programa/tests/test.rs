#[path = "../src/atividades/mod.rs"] mod atividades;

#[cfg(test)]
mod tests {
    use crate::atividades::questao_2::questao2;
    use crate::atividades::questao_3::questao3;

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
}