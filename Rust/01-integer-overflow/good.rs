// good.rs
pub fn process_withdrawal_secure(balance: u64, amount: u64, fee: u64) -> Result<u64, &'static str> {
    let total_cost = amount.checked_add(fee).ok_or("Integer overflow during addition")?;

    if balance < total_cost {
        return Err("Insufficient funds");
    }

    balance.checked_sub(total_cost).ok_or("Integer overflow during subtraction")
}
