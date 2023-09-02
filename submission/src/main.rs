mod HomeAssignment1;
mod HomeAssignment2;
mod HomeAssignment3;


fn main() {
    println!("Start of Home Assignment 1");
    let mut user = HomeAssignment1::User {
        name: "John".to_owned(),
        balance: (100.0, "SGD".to_owned()),
    };
    
    accrue_interest(&mut user, 10.0, 5);

    fn update_compound(user: &mut HomeAssignment1::User, interest: f32){
        user.balance.0 = user.balance.0 + (user.balance.0 * interest / 100.0);
        user.print_user_detail();  
    }
    
    fn accrue_interest(user: &mut HomeAssignment1::User, interest: f32, compound_factor: i8) {
        for _ in 1..=compound_factor{
            update_compound(user, interest);
        }     
    }
    println!("End of Home Assignment 1");
    println!("Start of Home Assignment 2");
        let mut buyer_john = HomeAssignment2::Buyer {
        name: "John".to_owned(),
        payment_type: HomeAssignment2::PaymentType::DigitalToken,
        balance: 100.00,
    };

    let mut buyer_sally = HomeAssignment2::Buyer {
        name: "Sally".to_owned(),
        payment_type: HomeAssignment2::PaymentType::Cash,
        balance: 100.00,
    };

    // let mut buyer_group_struct: Vec<Buyer> = Vec::new();

    let mut buyer_group = HomeAssignment2::BuyerGroup {
        members: Vec::new(),
    };

    buyer_group.add_member(buyer_john);
    println!("Buyer john has been added");
    buyer_group.add_member(buyer_sally);
    println!("Buyer Sally has been added");

    let mut seller_dabin = HomeAssignment2::Seller {
        payment_type: HomeAssignment2::PaymentType::Cash,
        price: 10.0,
        balance: 0.0,
    };
    println!("Seller Dabin has been instantiated");

    let found_index: i32 = buyer_group.find_buyers(&seller_dabin.payment_type);
    // check if index was found?
    println!("Find Buyers function returned the index: {}",found_index);

    if found_index > 0 {
        buyer_group.buy(found_index, &mut seller_dabin);
        println!("Since the value of found index is greater than 0, buy funcion is executed");
    }
    println!("End of Home Assignment 2");
    println!("Start of Home Assignment 3");

    // create user_account, and set his age as Option::None
    let mut user_account = HomeAssignment3::UserAccount{
        name: "Dabin".to_owned(),
        age: None
    };

    // You want to increase the user_account's balance by 11
    // use a match, if the result of increase_balance is
    match HomeAssignment3::increase_balance(&mut user_account, 11) {

        // - Ok: print "UserAccount balance increased to {}" where {} is the new balance value
        Ok(x) => println!("UserAccount balance increased to {}", x),

        // - Err: print the error message returned
        Err(e) => println!("{}",e)
    }

    if let Some(user_account_age) = user_account.age {
        println!("User account age is {}", user_account_age);
    } else{
        println!("User account age did not return a value of a type Option::Some. The user account's age is None");
    }
    // use an if...let...else statement to print the UserAccount age if it is a Option::Some
    println!("End of Home Assignment 3");
}
