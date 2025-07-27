use subxt::{OnlineClient, PolkadotConfig};

#[subxt::subxt(runtime_metadata_path = "polkadot_metadata.scale")]
pub mod polkadot {}

#[tokio::main]
pub async fn print_transfer_extrinsics() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://rpc.polkadot.io:443").await?;
    let mut blocks_sub = api.blocks().subscribe_finalized().await?;

    while let Some(block) = blocks_sub.next().await {
        let block = block?;
        let block_number = block.number();
        println!("New block #{block_number} created! Cheking transfer events..");

        let extrinsics = block.extrinsics().await?;

        for extrinsic_details in extrinsics.iter() {
            let events = extrinsic_details.events().await?;
            for event in events.iter() {
                // println!("Extrinsics: {extrinsic_name} #{extrinsic_index}");
                match event {
                    Ok(event) => {
                        let parsed_transfer =
                            event.as_event::<polkadot::balances::events::Transfer>()?;
                        if let Some(transfer) = parsed_transfer {
                            println!(
                                "-------- {:?} transfered from {:?} to {:?}",
                                transfer.amount,
                                transfer.from.to_string(),
                                transfer.to.to_string()
                            )
                        };
                    }
                    Err(e) => {
                        println!("Error: {e}");
                    }
                }
            }
        }
    }

    Ok(())
}
