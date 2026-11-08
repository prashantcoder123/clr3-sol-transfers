mod pb;

use pb::{mydata::v1 as mydata, sf::substreams::solana::v1::Transactions};

#[substreams::handlers::map]
fn map_my_data(transactions: Transactions) -> mydata::MyData {
    let mut output = mydata::MyData::default();

    for confirmed in &transactions.transactions {
        let Some(meta) = confirmed.meta.as_option() else {
            continue;
        };

        if meta.err.is_set() {
            continue;
        }

        let Some(tx) = confirmed.transaction.as_option() else {
            continue;
        };

        let Some(message) = tx.message.as_option() else {
            continue;
        };

        let accounts = &message.account_keys;

        let signature = tx.signatures.first()
            .map(|s| bs58::encode(s).into_string())
            .unwrap_or_default();

        for instruction in &message.instructions {
            let program_index = instruction.program_id_index as usize;

            if accounts.get(program_index)
                .map(|key| bs58::encode(key).into_string())
                .as_deref()
                != Some("11111111111111111111111111111111")
            {
                continue;
            }

            let data = &instruction.data;

            if data.len() != 12 {
                continue;
            }

            let discriminator = u32::from_le_bytes(data[0..4].try_into().unwrap());

            if discriminator != 2 {
                continue;
            }

            let amount = u64::from_le_bytes(data[4..12].try_into().unwrap());

            if instruction.accounts.len() < 2 {
                continue;
            }

            let from_index = instruction.accounts[0] as usize;
            let to_index = instruction.accounts[1] as usize;

            let (Some(from), Some(to)) = (
                accounts.get(from_index),
                accounts.get(to_index),
            ) else {
                continue;
            };

            output.transfers.push(mydata::Transfer {
                from: bs58::encode(from).into_string(),
                to: bs58::encode(to).into_string(),
                amount,
                signature: signature.clone(),
            });
        }
    }

    output
}
