 extern crate ic_cdk;
use ic_cdk::storage;
extern crate derive_more;
extern crate ic_cdk_macros;
extern crate reqwest;
extern crate candid;
extern crate async_std;
extern crate serde;
extern crate actix_web;
use derive_more::DebugDefault;
use std::collections::HashMap;
use std::net::{TcpListener, TcpStream};
use std::io::{Read, Write};

// for transactions
use reqwest::Error;
use candid::candid;
use async_std::task;
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use serde::{Deserialize, Serialize};
use candid::CandidType;
use ic_cdk::export::candid::{CandidType};
use ic_cdk::export::Principal;
use ic_cdk_macros::*;
use actix_web::{web, App, HttpResponse, HttpServer, Responder};

// Define a struct to represent a house
#[derive(Serialize, Deserialize, CandidType)]
struct TrnsactionPay {
    from: String,
    to: String,
    amount: u64,
    // Add more details about the house as needed
}

//link the to fetch response
async fn process_transaction_endpoint(payload: web::Json<TransactionPayload>) -> impl Responder {
    match process_transaction(&payload.into_inner()) {
        Ok(message) => HttpResponse::Ok().body(message),
        Err(error) => HttpResponse::BadRequest().body(error),
    }
}
//link the endpoint
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/process_transaction", web::post().to(process_transaction_endpoint))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}



type KeyType = u32;
type ValueType = String;

const my_landlord_ids: u64 = 123;
const my_property_ids: u64 = 456;


struct Tenant{
    id: u64,
    name: String,
    // Other fields
}

impl Tenant{
    fn new(id: u64, name: String) -> Self {
        Tenant { id, name }
    }
}


#[derive(Debug)]
struct User {
    id: u64,
    name: String,
    email: String,
    role: Role,
}

#[derive(Debug)]
enum Role {
    Landlord,
    Tenant,
}

impl Default for Role {
    fn default() -> Self {
        Role::Landlord
    }
}



struct PropertyListing {
    id: u64,
    landlord_id: u64,
    property: Property,
}

struct RentalAgreement {
    tenant_id: u64,
    landlord_id: u64,
    property_id: u64,
}
pub struct RentalContract {
    tenant_id: u64,
    landlord_ids: u64,
    property_ids: u64,
}

impl RentalContract {
    pub fn new(agreement: RentalAgreement) -> Self {
        // Initialize fields here
        RentalContract {
            tenant_id: agreement.tenant_id,
            landlord_ids: agreement.landlord_id,
            property_ids: agreement.property_id,
        }
    }
}
   
    fn get_user_details(user_id: u64) -> User {
        // Fetch user details from storage or provide a default implementation
        // For now, assuming storage.get_user(user_id) returns the User
        storage::get::<User>(user_id).unwrap_or_default()
    }
    
    fn register_user(name: String, email: String, role: Role) {
        let user_id = storage::next_user_id();
    
        let user = User {
            id: user_id,
            name,
            email,
            role,
        };
    
        storage::insert(user_id, user);
    }
    
    fn create_property_listing(landlord_id: u64, property: Property) {
        let property_id = storage::next_property_id();
    
        let listing = PropertyListing {
            id: property_id,
            landlord_id,
            property,
        };
    
        storage::insert(property_id, listing);
    }
    
    fn sign_rental_agreement(tenant_id: u64, landlord_id: u64, property_id: u64) {
        let agreement = RentalAgreement {
            tenant_id,
            landlord_id,
            property_id,
        };
    
        // Create rental agreement smart contract
        let rental_contract = RentalContract::new(agreement);
    
        storage::insert(property_id, agreement);
    }

  

    fn main() {
        // Initialize the backend components, e.g., set up storage, configure server, etc.
            // Initialize storage
            let mut storage: HashMap<KeyType, ValueType> = HashMap::new();
        
            // Configure server
            let server_address = "127.0.0.1:8080";
            let server = TcpListener::bind(server_address).expect("Failed to bind to address");
        
            println!("Server listening on {}", server_address);
        
            // Main server loop
            for stream in server.incoming() {
                match stream {
                    Ok(mut stream) => {
                        // Handle client connections
                        // For example, handle incoming requests, process data, interact with storage, etc.
                  
                    }
                    Err(e) => {
                        eprintln!("Error: {}", e);
                    }
                }
            }
        
        
            fn handle_client(mut stream: TcpStream) {
                // Read data from the client stream
                let mut buffer = [0; 512];
                stream.read(&mut buffer).unwrap();
            
                // Process the incoming data (e.g., parse HTTP request)
            
                // Send a response back to the client
                let response = "HTTP/1.1 200 OK\r\n\r\nHello, world!";
                stream.write(response.as_bytes()).unwrap();
                stream.flush().unwrap();
            }
        
    
        // Start the backend server or any other backend processing,interact database and other servers
        println!("Backend is running.");
        let user = get_user_details(1);

       println!("User details: {:?}", user);

        start_backend_server();
    }
    
    fn start_backend_server() {
        // Start the backend server
        println!("Backend server started.");
        // Add your server logic here, e.g., listen for incoming requests, handle them, etc.
    }


// Define a struct for representing a validator node
#[derive(Debug)]
struct Validator {
    name: String,
    stake: u64,
    is_validator: bool,
}

impl Validator {
    // Method to create a new validator with a given name and stake
    fn new(name: String, stake: u64) -> Self {
        Validator {
            name,
            stake,
            is_validator: false,
        }
    }

    // Method to deposit coins and become a validator
    fn deposit(&mut self, amount: u64) {
        self.stake += amount;
        self.is_validator = true;
    }

    // Method to validate a block and receive transaction fees as reward
    fn validate_block(&mut self, transactions: &[Transaction]) {
        // Perform block validation here
        // If all transactions are valid, add block to blockchain and receive fees
        // For simplicity, we'll just print a message indicating validation
        println!("Validator {} validated a block and received transaction fees", self.name);
    }

    // Method to penalize a validator for approving fraudulent transactions
    fn penalize(&mut self, amount: u64) {
        if self.stake >= amount {
            self.stake -= amount;
        } else {
            // Handle case where validator's stake is insufficient
            println!("Validator {} does not have enough stake to penalize", self.name);
        }
    }

    // Method to release stake and transaction fees when no longer a validator
    fn release_stake(&mut self) -> u64 {
        if !self.is_validator {
            // Handle case where validator is not currently active
            return 0;
        }
        self.is_validator = false;
        let released_stake = self.stake;
        self.stake = 0;
        released_stake
    }
}

// Define a struct for representing a transaction
#[derive(Debug)]
struct Transaction {
    // Define transaction fields here
}

// Create some validator nodes
fn create_validators() -> HashMap<String, Validator> {
    let mut validators: HashMap<String, Validator> = HashMap::new();
    validators.insert("Validator1".to_string(), Validator::new("Validator1".to_string(), 100));
    validators.insert("Validator2".to_string(), Validator::new("Validator2".to_string(), 150));
    validators.insert("Validator3".to_string(), Validator::new("Validator3".to_string(), 200));
    validators
}

// Deposit coins to become validators
fn deposit_coins(validators: &mut HashMap<String, Validator>) {
    if let Some(validator) = validators.get_mut("Validator1") {
        validator.deposit(50);
    }
    if let Some(validator) = validators.get_mut("Validator2") {
        validator.deposit(100);
    }
}

// Simulate block validation process
fn validate_blocks(validators: &mut HashMap<String, Validator>, transactions: &[Transaction]) {
    for (_, validator) in validators.iter_mut() {
        if validator.is_validator {
            validator.validate_block(transactions);
        }
    }
}

// Simulate penalizing a validator
fn penalize_validator(validators: &mut HashMap<String, Validator>) {
    if let Some(validator) = validators.get_mut("Validator3") {
        validator.penalize(50);
    }
}

// Simulate releasing stake and transaction fees for a validator
fn release_stake(validators: &mut HashMap<String, Validator>) {
    if let Some(validator) = validators.get_mut("Validator1") {
        let released_amount = validator.release_stake();
        println!("Validator {} released {} coins", validator.name, released_amount);
    }
}


