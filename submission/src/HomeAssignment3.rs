// define struct of UserAccount with field: name (String), and age (Option<u32>)
pub struct UserAccount {
    pub name: String,
    pub age: Option<u32>
}

// define a trait called Balance, and within, function get_balance returning integer of 10
pub trait Balance {
    fn get_balance(&self)->u32;
}

// implement trait Balance to UserAccount struct
impl Balance for UserAccount{
    fn get_balance(&self)->u32{
        return 10
    }
}

// create function increase_balance which takes as arguments
// - a type that implements Balance trait
// - an u32 amount parameter containing the increase amount
// within this function,
pub fn increase_balance<T:Balance>(user_account: &mut T, increase_amount: u32)->Result<u32, String>{
// - if increase amount is <= 10, return an OK containing the get_balance + amount
    if increase_amount <= 10 {
        // user_account.age += increase_amount;
        return Ok(user_account.get_balance() + increase_amount);

    // - if increase amount is > 10, return an Err with error message "Increase must be less than 10!"
    } else {
        return Err("Increase must be less than 10!".to_owned());
    }
}
// Tip: this function should return a Result<u32, String>

// fn main() {
//     // create user_account, and set his age as Option::None
//     let mut user_account = UserAccount{
//         name: "Dabin".to_owned(),
//         age: None
//     };

//     // You want to increase the user_account's balance by 11
//     // use a match, if the result of increase_balance is
//     match increase_balance(&mut user_account, 11) {

//         // - Ok: print "UserAccount balance increased to {}" where {} is the new balance value
//         Ok(x) => println!("UserAccount balance increased to {}", x),

//         // - Err: print the error message returned
//         Err(e) => println!("{}",e)
//     }

//     if let Some(user_account_age) = user_account.age {
//         println!("User Account age is {}", user_account_age);
//     }
//     // use an if...let...else statement to print the UserAccount age if it is a Option::Some
//    }