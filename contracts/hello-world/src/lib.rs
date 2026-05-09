#![no_std]
use soroban_sdk::{contract, contractevent, contractimpl, Address, Env, token};

#[contract]
pub struct Contract;

#[contractevent(data_format = "single-value")]
pub struct Donate {
    #[topic]
    from: Address,
    #[topic]
    to: Address,
    amount: i128,
}

#[contractimpl]
impl Contract {

     pub fn donate(
        env: Env,
        from: Address,
        to: Address,
        token_address: Address,
        amount: i128,    
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
            amount
        }.publish(&env);
    }
}

mod test;
