use substreams::log;
use substreams_solana::pb::sf::solana::r#type::v1::Transaction;
use substreams_solana::pb::sf::solana::block_meta::v1::BlockMeta;
use substreams::store::StoreAdd;
use substreams::prelude::*;

#[substreams::handlers::store]
fn store_transfer_amounts(transactions: Vec<Transaction>, store: StoreAddInt64) {
    for tx in transactions.iter() {
        if tx.meta.as_ref().map_or(false, |m| m.err.is_none()) {
            for ix in &tx.transaction.as_ref().unwrap().message.as_ref().unwrap().instructions {
                if let Some(parsed) = &ix.parsed {
                    if parsed.r#type == "transfer" {
                        let amount = parsed.info.get("amount").unwrap_or(&"0".to_string()).parse::<i64>().unwrap_or(0);
                        store.add("total_transfer_amount", 0, amount);
                    }
                }
            }
        }
    }
}