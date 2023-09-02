//Day 01 Module 01 - Rust Fundamentals 1 - Home Assignment
//Includes Bonus
//Output Sample
// dabin@DabinZ16G1:~/learning/Solana/exercise1$ cargo run
//    Compiling exercise1 v0.1.0 (/home/dabin/learning/Solana/exercise1)
//     Finished dev [unoptimized + debuginfo] target(s) in 0.26s
//      Running `target/debug/exercise1`
// Name: John, Balance: (110.0, "SGD")
// Name: John, Balance: (121.0, "SGD")
// Name: John, Balance: (133.1, "SGD")
// Name: John, Balance: (146.41, "SGD")
// Name: John, Balance: (161.05101, "SGD")

pub struct User {
    pub name: String,
    pub balance: (f32, String)
   }

impl User {
    pub fn print_user_detail(&self) {
    println!("Name: {}, Balance: {:?}",self.name, self.balance)
    }
}

fn main() {
    let mut user = User {
        name: "John".to_owned(),
        balance: (100.0, "SGD".to_owned()),
    };
    
    accrue_interest(&mut user, 10.0, 5);

    pub fn update_compound(user: &mut User, interest: f32){
        user.balance.0 = user.balance.0 + (user.balance.0 * interest / 100.0);
        user.print_user_detail();  
    }
    
    pub fn accrue_interest(user: &mut User, interest: f32, compound_factor: i8) {
        for _ in 1..=compound_factor{
            update_compound(user, interest);
        }     
    }
}