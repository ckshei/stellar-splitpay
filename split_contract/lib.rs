#![cfg_attr(not(feature = "std"), no_std)]

#[ink::contract]
mod splitter {
    #[ink(storage)]
    pub struct Splitter {
        recipient1: 14gJrvGkKpAANr3R3cqC8nNSH4B7aQoYduJhD3W5H2Qdn1Re,
        recipient2: 15uRzL5XJxZL9EN1uFZJZw7h9CFggwoS6RDzz1R7Xy5a1XtR,
        recipient3: 12v5Xeoyv3nmq5NECNPhgK7vLuw7kNPAyQ95Pv17tVJJKZJ9,
    }

    impl Splitter {
        #[ink(constructor)]
        pub fn new(r1: AccountId, r2: AccountId, r3: AccountId) -> Self {
            Self {
                recipient1: r1,
                recipient2: r2,
                recipient3: r3,
            }
        }

        #[ink(message, payable)]
        pub fn split(&mut self) {
            let value = self.env().transferred_value();
            assert!(value > 0, "No funds sent");

            let share1 = value / 4; // 25%
            let share2 = value / 4; // 25%
            let share3 = value - share1 - share2; // 50%

            self.env()
                .transfer(self.recipient1, share1)
                .expect("Transfer to recipient1 failed");
            self.env()
                .transfer(self.recipient2, share2)
                .expect("Transfer to recipient2 failed");
            self.env()
                .transfer(self.recipient3, share3)
                .expect("Transfer to recipient3 failed");
        }
    }
}
