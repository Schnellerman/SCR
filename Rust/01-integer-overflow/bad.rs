// bad.rs
pub fn process_withdrawal(balance: u64, amount: u64, fee: u64) -> Result<u64, &'static str> {
    if balance < amount + fee {
        return Err("Insufficient funds");
    }

    Ok(balance - (amount + fee))
}
