use core:: {
    block::{ BlockHeader, Block }
};
use chrono::Utc;

fn main() {
    let block_1_header = BlockHeader {
        parent_hash: "",
        difficulty: 0,
        number: 0,
        timestamp: Utc::now().timestamp(),
        nonce: 0
    };
    print!("{}", block_1_header);
}
