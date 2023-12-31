pub fn calculo_saldo(carteira:f64, pagamento:f64) -> Result<f64, &'static str> {
    if carteira< pagamento {
        return  Err("Saldo insuficiente !");
    }
    let saldo = carteira - pagamento;
    return Ok(saldo);
}


