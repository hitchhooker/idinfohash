use subxt::{OnlineClient, PolkadotConfig};
use subxt::utils::AccountId32;
use sp_core::blake2_256;
use std::str::FromStr;
use sp_core::Encode;

#[subxt::subxt(runtime_metadata_path = "people_rococo_metadata.scale")]
pub mod people_chain {}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let api = OnlineClient::<PolkadotConfig>::from_url("wss://dev.rotko.net/people-rococo").await?;

    // master_of_success account
    let account_id = "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y";
    let account = AccountId32::from_str(account_id)?;

    let identity_info = api
        .storage()
        .at_latest()
        .await?
        .fetch(&people_chain::storage().identity().identity_of(&account))
        .await?;

    if let Some((registration, _username)) = identity_info {
        println!("Identity found for account: {:?}", account_id);
        println!("Registration: {:?}", registration);

        let identity_info = &registration.info;
        let encoded_info = identity_info.encode();
        println!("Encoded IdentityInfo: 0x{}", hex::encode(&encoded_info));
        let hash = blake2_256(&encoded_info);

        println!("Hashed IdentityInfo (Blake2-256): {:?}", hash);
        println!("Hashed IdentityInfo (Hex): 0x{}", hex::encode(hash));
        println!("pjs master of success: 0xe59561336092568da2f4167fe25946e7145962d2de6bb29806530b3a6fc625b6");
    } else {
        println!("No identity information found for account: {:?}", account_id);
    }

    Ok(())
}
