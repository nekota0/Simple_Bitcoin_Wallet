struct Wallet {
    balance: f64,
    transactions: Vec<Transaction>,
}

struct Transaction {
    from: String,
    to: String,
    amount: f64,
}

impl Wallet {
    fn new() -> Self {
        Wallet {
            balance: 0.0,
            transactions: Vec::new(),
        }
    }

    fn receive(&mut self, amount: f64, from: String) {
        self.balance += amount;
        self.transactions.push(Transaction {
            from,
            to: "My Wallet".to_string(),
            amount,
        });
    }

    fn send(&mut self, amount: f64, to: String) -> Result<(), String> {
        if self.balance >= amount {
            self.balance -= amount;
            self.transactions.push(Transaction {
                from: "My Wallet".to_string(),
                to,
                amount,
            });
            Ok(())
        } else {
            Err("Insufficient funds".to_string())
        }
    }

    fn get_balance(&self) -> f64 {
        self.balance
    }

    fn get_transaction_history(&self) -> &Vec<Transaction> {
        &self.transactions
    }
}

fn main() {
    let mut my_wallet = Wallet::new();

    my_wallet.receive(1.5, "Alice".to_string());
    my_wallet.receive(2.0, "Bob".to_string());

    match my_wallet.send(1.0, "Charlie".to_string()) {
        Ok(_) => println!("Transaction successful"),
        Err(e) => println!("Transaction failed: {}", e),
    }

    println!("Current balance: {}", my_wallet.get_balance());

    println!("Transaction history:");
    for transaction in my_wallet.get_transaction_history() {
        println!("From: {}, To: {}, Amount: {}", transaction.from, transaction.to, transaction.amount);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_wallet() {
        let wallet = Wallet::new();
        assert_eq!(wallet.get_balance(), 0.0);
        assert!(wallet.get_transaction_history().is_empty());
    }

    #[test]
    fn test_receive() {
        let mut wallet = Wallet::new();
        wallet.receive(1.5, "Alice".to_string());
        assert_eq!(wallet.get_balance(), 1.5);
        assert_eq!(wallet.get_transaction_history().len(), 1);
    }

    #[test]
    fn test_send_success() {
        let mut wallet = Wallet::new();
        wallet.receive(2.0, "Bob".to_string());
        let result = wallet.send(1.0, "Charlie".to_string());
        assert!(result.is_ok());
        assert_eq!(wallet.get_balance(), 1.0);
        assert_eq!(wallet.get_transaction_history().len(), 2);
    }

    #[test]
    fn test_send_insufficient_funds() {
        let mut wallet = Wallet::new();
        wallet.receive(1.0, "Dave".to_string());
        let result = wallet.send(2.0, "Eve".to_string());
        assert!(result.is_err());
        assert_eq!(wallet.get_balance(), 1.0);
        assert_eq!(wallet.get_transaction_history().len(), 1);
    }
}