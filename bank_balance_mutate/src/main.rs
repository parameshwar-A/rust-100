fn main() {

    struct Account{
        id: u32,
        balance: u32
    }

    impl Account{

        fn balance_top_up(&mut self, value: u32){
            self.balance=self.balance+value;
        }

    }

    let mut acc1: Account = Account{
        id: 1234,
        balance: 0
    };
    println!("Initial");
    println!("Id: {} \nBalance: {}", acc1.id, acc1.balance);

    acc1.balance_top_up(20);

    println!("After update");
    println!("Id: {} \nBalance: {}", acc1.id, acc1.balance);

}
