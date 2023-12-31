#[path = "../src/modules/services/math/transacao.rs"] mod transacao;
use transacao::calculo_saldo;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_calculo_saldo() {
        let valor = calculo_saldo(100.0, 90.0);
        assert_eq!(valor,Ok(10.0) );
    }

    #[test]
    fn test_calculo_saldo_saldo_insuficiente() {
        let valor = calculo_saldo(10.0, 100.0);
        assert_eq!(valor,Err("Saldo insuficiente !"));
    }
}