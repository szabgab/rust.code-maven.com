#![allow(dead_code)]

#[cfg(test)]
use mockall::predicate;

#[cfg_attr(test, mockall::automock)]
trait BankTeller {
    fn get_balance(&self, name: &str) -> u32;
}

struct Bank;
impl BankTeller for Bank {
    fn get_balance(&self, name: &str) -> u32 {
        // Simulate a long-running operation
        std::thread::sleep(std::time::Duration::from_secs(2));
        if name == "Alice" {
            5
        } else if name == "Bob" {
            6
        } else {
            0
        }
    }
}

fn get_total_money(x: &dyn BankTeller, accounts: &[&str]) -> u32 {
    let mut balance = 0;
    for account in accounts {
        balance += x.get_balance(account);
    }
    balance
}

#[test]
fn test_call_with_alice() {
    let mut mock = MockBankTeller::new();
    mock.expect_get_balance()
        .with(predicate::eq("Alice"))
        .times(1)
        .returning(|_| 5);
    assert_eq!(5, get_total_money(&mock, ["Alice"].as_slice()));
}

#[test]
fn test_call_with_alice_bob_and_mary() {
    let mut mock = MockBankTeller::new();
    mock.expect_get_balance()
        .with(predicate::eq("Alice"))
        .times(1)
        .returning(|_| 2);
    mock.expect_get_balance()
        .with(predicate::eq("Mary"))
        .times(1)
        .returning(|_| 10);
    mock.expect_get_balance()
        .with(predicate::eq("Bob"))
        .times(1)
        .returning(|_| 3);
    assert_eq!(15, get_total_money(&mock, ["Alice", "Bob", "Mary"].as_slice()));
}


// #[test]
// fn test_call_without_mock() {
//     let real = Bank;
//     assert_eq!(11, get_total_money(&real, ["Alice", "Bob"].as_slice()));
// }
