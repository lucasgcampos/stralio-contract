#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, contracttype, Address, Env, token, String};

#[contract]
pub struct Contract;

#[contracttype]
pub struct DonationData {
    pub amount: i128,
    pub username: String,
    pub message: String,
}

#[contractevent(data_format = "single-value")]
pub struct Donate {
    #[topic]
    from: Address,
    #[topic]
    to: Address,
    data: DonationData
}

#[contractimpl]
impl Contract {

     pub fn donate(
        env: Env,
        from: Address,
        to: Address,
        token_address: Address,
        amount: i128,
        username: String,
        message: String    
    ) {
        from.require_auth();
        
        let token = token::Client::new(&env, &token_address);
        token.transfer(&from, &env.current_contract_address(), &amount);

        let split_amount = amount * 95 / 100;
        token.transfer(
            &env.current_contract_address(),
            &to,
            &split_amount,
        );

        Donate {
            from,
            to,
            data: DonationData {
                amount,
                username,
                message
            }
        }.publish(&env);
    }
}

mod test;
