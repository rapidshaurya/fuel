use std::str::FromStr;

use fuels::{prelude::*, tx::ContractId, signers::{fuel_crypto::SecretKey}};

// Load abi from json
abigen!(MyContract, "out/debug/deploy2testnet-abi.json");
async fn connect_to_wallets() -> WalletUnlocked {
    let provider = Provider::connect("node-beta-2.fuel.network").await.unwrap();
    let secret = SecretKey::from_str("YOUR SECRET KEY")
    .unwrap();
    let wallet = WalletUnlocked::new_from_private_key(secret, Some(provider));
    wallet
}

async fn deploy(wallet: WalletUnlocked) -> (MyContract, ContractId) {
    // Launch a local network and deploy the contract
   
    let id = Contract::deploy(
        "./out/debug/deploy2testnet.bin",
        &wallet,
        TxParameters::new(Option::Some(1), Option::Some(1), None),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/deploy2testnet-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = MyContract::new(id.clone(), wallet);
    (instance, id.into())
}

#[tokio::test]
async fn can_get_contract_id() {
    let wallet = connect_to_wallets().await;
    
    //let (instance, _cid) = deploy(wallet.clone()).await; //for first time use this to deploy contract
    let cid= ContractId::from_str("contract id").unwrap(); //if you already deployed contract the use this 
    let instance = MyContract::new(cid.into(), wallet.clone());// cid is contract id (if you already deployed contract the use this )
    
    let tx = TxParameters::new(Option::Some(1), None, None);
    let a=instance.methods().increment_counter(1).tx_params(tx).call().await.unwrap();
    dbg!(a.value);
    // Now you have an instance of your contract you can use to test each function
}
