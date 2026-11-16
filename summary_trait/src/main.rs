fn main() {

    struct UPI{
        id: String,
        account: i32,
        balance: i32
    }

    struct DebitCard{
        card_number: String,
        account: i32,
        balance: i32
    }

    trait Transaction{
        // dynamically add things
        fn transaction(&mut self, other: &mut Self, amount:i32) -> bool;
    }

    impl Transaction for UPI{
        fn transaction(&mut self, other: &mut Self, amount:i32) -> bool {

            if amount > self.balance{
                return false;
            }
            println!("The balance from {} is debited through {} UPI id", self.account, self.id);
            self.balance = self.balance-amount;
            other.balance = other.balance + amount;
            true
        }
    }

    impl Transaction for DebitCard{
        fn transaction(&mut self, other: &mut Self, amount: i32) ->bool {
            if amount > self.balance{
                return false;
            }
            println!("The balance from {} is debited through {} card", self.account, self.card_number);
            self.balance = self.balance-amount;
            other.balance = other.balance + amount;
            true
        }
    }

    let mut cd1: DebitCard = DebitCard{
        card_number: String::from("9999 9999 8888 8888"),
        account : 3999,
        balance: 20000
    };

    let mut cd2: DebitCard = DebitCard{
        card_number: String::from("7777 7777 8888 8888"),
        account : 4099,
        balance: 1000
    };

    let mut upid: UPI= UPI{
        id: String::from("wwwwwwwww@swdw"),
        account : 1000,
        balance: 20000
    };

    let mut upid2: UPI= UPI{
        id: String::from("rrrrrrrrrr@wwww"),
        account : 1001,
        balance: 2000
    };

    cd1.transaction(&mut cd2, 200);
    upid.transaction(&mut upid2, 3000);

}


