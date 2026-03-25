#![no_std]
use soroban_sdk::{contract, contractimpl, symbol_short, Address, Env, Symbol};

// Định nghĩa các khóa để lưu trữ dữ liệu trên blockchain
const CLIENT: Symbol = symbol_short!("CLIENT");
const FREELANCER: Symbol = symbol_short!("FREELAN");
const STATUS: Symbol = symbol_short!("STATUS");

#[contract]
pub struct EscrowContract;

#[contractimpl]
impl EscrowContract {
    // 1. Khởi tạo hợp đồng với địa chỉ của Khách hàng và Freelancer
    pub fn init(env: Env, client: Address, freelancer: Address) {
        env.storage().instance().set(&CLIENT, &client);
        env.storage().instance().set(&FREELANCER, &freelancer);
        env.storage().instance().set(&STATUS, &symbol_short!("PENDING")); // Trạng thái ban đầu
    }

    // 2. Khách hàng nạp tiền vào Escrow (Ở bản đơn giản này, ta chỉ cập nhật trạng thái)
    pub fn fund(env: Env, caller: Address) {
        caller.require_auth(); // Bắt buộc người gọi phải ký giao dịch
        
        let client: Address = env.storage().instance().get(&CLIENT).unwrap();
        if caller != client {
            panic!("Chi co khach hang (Client) moi co the nap tien!");
        }
        
        env.storage().instance().set(&STATUS, &symbol_short!("FUNDED"));
    }

    // 3. Khách hàng nghiệm thu và giải ngân cho Freelancer
    pub fn release(env: Env, caller: Address) {
        caller.require_auth();
        
        let client: Address = env.storage().instance().get(&CLIENT).unwrap();
        if caller != client {
            panic!("Chi co khach hang (Client) moi co the giai ngan!");
        }
        
        let status: Symbol = env.storage().instance().get(&STATUS).unwrap();
        if status != symbol_short!("FUNDED") {
            panic!("Chua nap tien hoac da giai ngan roi!");
        }
        
        env.storage().instance().set(&STATUS, &symbol_short!("RELEASED"));
        
        // LƯU Ý: Trong ứng dụng thực tế, code chuyển USDC sẽ được gọi ở đây.
        // Ví dụ: token::Client::new(&env, &token_address).transfer(...);
    }


    pub fn get_status(env: Env) -> Symbol {
        env.storage().instance().get(&STATUS).unwrap_or(symbol_short!("NONE"))
    }
}