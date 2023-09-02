//Day 01 Module 02 - Rust Fundamentals 2 - Home Assignment
//Includes Bonus
//Output Sample

#[derive(PartialEq)]
pub enum PaymentType {
    DigitalToken,
    Cash
}

pub struct Seller {
    pub payment_type: PaymentType,
    pub price: f32,
    pub balance: f32,
}

#[derive(PartialEq)]
pub struct Buyer {
    pub name: String,
    pub payment_type: PaymentType,
    pub balance: f32
}

pub struct BuyerGroup {
    pub members: Vec<Buyer> 
}

impl BuyerGroup {
    pub fn add_member(&mut self, buyer: Buyer){
        self.members.push(buyer);
        // println!("{:?}",self.members.get(0));
    }

    pub fn find_buyers(&self, payment_type: &PaymentType) -> i32{
        let mut return_index = 0;
        // while 
        for i in &self.members {
            // match self.members.get(i).payment_type {
            //     payment_type => return i
            // };
            if i.payment_type == *payment_type {
                return return_index;
                // break ? - after returning, iteration stops. This only returns the 1st buyer that mathces if there are multiple of them
            }
            return_index += 1;
        };
        return -1; //non-found
    }

    // pub fn buy(&mut self, index_buy: i32, seller_struct: &mut Seller){
    //     let mut buyer = self.members.get(index_buy as usize);
    //     for i in &mut self.members {
    //         if buyer.balance >= seller_struct.price {
    //             seller_struct.balance += seller_struct.price;
    //             buyer.balance -= seller_struct.price;
    //         } else {
    //             break;
    //         }
    //     }
    // }

    pub fn buy(&mut self, index_buy: i32, seller_struct: &mut Seller){
        let mut buyer = &mut self.members[index_buy as usize];
        loop {
            if buyer.balance>= seller_struct.price {
                buyer.balance -= seller_struct.price;
                seller_struct.balance += seller_struct.price;
            } else {
                break;
            }
        }
    }   
}

// pub fn main() {
//     let mut buyer_john = Buyer {
//         name: "John".to_owned(),
//         payment_type: PaymentType::DigitalToken,
//         balance: 100.00,
//     };

//     let mut buyer_sally = Buyer {
//         name: "Sally".to_owned(),
//         payment_type: PaymentType::Cash,
//         balance: 100.00,
//     };

//     // let mut buyer_group_struct: Vec<Buyer> = Vec::new();

//     let mut buyer_group = BuyerGroup {
//         members: Vec::new(),
//     };

//     buyer_group.add_member(buyer_john);
//     buyer_group.add_member(buyer_sally);

//     let mut seller_dabin = Seller {
//         payment_type: PaymentType::Cash,
//         price: 10.0,
//         balance: 0.0,
//     };

//     let found_index = buyer_group.find_buyers(&seller_dabin.payment_type);
//     // check if index was found?
//     // println!("{}",found_index);

//     if found_index > 0 {
//         buyer_group.buy(found_index, &mut seller_dabin);
//     }

// }