use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::{env, log, near_bindgen, Balance, AccountId};

const URL: &str = "https//www.dmart.com";
const DEFAULT_TEXT: &str = "I am a data string";

#[derive(BorshDeserialize, BorshSerialize)]
enum Data {
    Text(String),
    Dataset(df),
    Image([u32; 256]),
}

// Define the contract structure
#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    data: Data,
    title: String,
    description: String,
    tags: [String; 25],
    price: u128,
    full_data_avaliable_at: String,
}

impl Default for Contract {
    fn default() -> Self {
        Self {
            data: Data::text(DEFAULT_TEXT).to_string(),
        }
    }
}

// Implement the contract structure
#[near_bindgen]
impl Contract {
    // Overrides default implementation
    #[init]
    pub fn new(data: Data, title: String, description: String, tags: Box<[String]>, price: u128) -> Self {
        Self { 
            data, 
            title, 
            description,
            tags,
            price,
            full_data_avaliable_at: URL,
        }
    }

    pub fn get_data(&self) -> Data {
        self.data.clone()
    }

    // This will be handled by website and front-end
    pub fn display_sample(&self) {
        println!("Title: {}", self.title);
        println!("Description: {}", self.description);
        match self.data {
            Data::text => println!("Sample Viewing: {}", Data::text),
            None => println!("Data not avalaible for sample viewing"),
        };
        println!("Tags: {}", self.tags);
        println!("Avalaible at {}", self.full_data_avaliable_at);
    }

    // Overwrites current data with new data
    pub fn set_new_data(&mut self, new_data: Data) {
        log!("Overwriting data from {}", new_data);
        self.data = new_data;
    }

    pub fn recieve_payment(&self) -> u128 {
        let consumer: AccountId = env::predecessor_account_id();
        let amount: Balance = env::attached_deposit();

        if amount == self.price {
            // Extensive error checking must be done here to confirm the contract has been fulfilled
            // May be an easy way to do this
            // You have to make an HTTPS GET 
            // Adding the the consumer address to the front-end
            // The front end will tell the user you have one avaliable on the contract
        }
    }
    // can you upload data to the smart contract?
    // make the data invisible or non-viewable
    // I want to look at .clone() and log! macro
}

/*
 * The rest of this file holds the inline tests for the code above
 * Learn more about Rust tests: https://doc.rust-lang.org/book/ch11-01-writing-tests.html
 */
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn view_default_data_in_contract() {
        let mut contract = Contract::default();
        let data = contract.get_data();
        assert_eq!(
            "I am a data string",
            data,
        )
    }

    #[test]
    fn set_new_data_text_then_get_data() {
        let mut contract = Contract::default();
        let text_data = Data::text(String::from("Updated data"));
        contract.set_new_data(text_data);

        assert_eq!(
            contract.get_data(),
            "Updated data".to_string()
        );
    }
}
