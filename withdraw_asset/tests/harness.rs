use fuels::{prelude::*, tx::ContractId};

// Load abi from json
abigen!(MyContract, "out/debug/withdraw_asset-abi.json");

// this function will help you to create multipe wallets
async fn create_wallets() -> Vec<WalletUnlocked> {
    let base_asset_id: AssetId =
        "0x9ae5b658754e096e4d681c548daf46354495a437cc61492599e33fc64dcdc30c" //liquidity asset id
            .parse()
            .unwrap();

    let asset_ids = [AssetId::default(), base_asset_id];
    let asset_configs = asset_ids
        .map(|id| AssetConfig {
            id,
            num_coins: 1,
            coin_amount: 1_000_000,
        })
        .into();

    let wallet_config = WalletsConfig::new_multiple_assets(1, asset_configs); // you can set total no. of wallets here
    let wallets = launch_custom_provider_and_get_wallets(wallet_config, None, None).await; //connecting to local node
    wallets
}

//this function will help you to deploy contract and return MyContract instance and contract id
async fn deploy_contract(wallet: &WalletUnlocked) -> (MyContract, ContractId) {
    let id = Contract::deploy(
        "./out/debug/withdraw_asset.bin",
        &wallet,
        TxParameters::default(),
        StorageConfiguration::with_storage_path(Some(
            "./out/debug/withdraw_asset-storage_slots.json".to_string(),
        )),
    )
    .await
    .unwrap();

    let instance = MyContract::new(id.clone(), wallet.to_owned());

    (instance, id.into())
}

#[tokio::test]
async fn deposit() {
    let wallet = create_wallets().await;
    let (instance, contract_id) = deploy_contract(&wallet[0]).await;
    let deposit_amount = 1_000_000;
    let base_asset_id: AssetId =
        "0x9ae5b658754e096e4d681c548daf46354495a437cc61492599e33fc64dcdc30c"
            .parse()
            .unwrap();
    let call_params = CallParameters::new(Some(deposit_amount), Some(base_asset_id), None);
    let contract_methods = instance.methods();
    let lp_token_balance = wallet[0].get_asset_balance(&base_asset_id).await.unwrap();
    dbg!(lp_token_balance);
    //depositing asset to contract
    contract_methods
        .deposit(wallet[0].address().into())
        .call_params(call_params)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    let lp_asset_id = AssetId::from(*contract_id);
    let lp_token_balance = wallet[0].get_asset_balance(&lp_asset_id).await.unwrap();
    dbg!(lp_token_balance);
}

#[tokio::test]
async fn deposit_and_withdraw() {
    let wallet = create_wallets().await;
    let (instance, contract_id) = deploy_contract(&wallet[0]).await;
    let deposit_amount = 1_000_000;
    let base_asset_id: AssetId =
        "0x9ae5b658754e096e4d681c548daf46354495a437cc61492599e33fc64dcdc30c"
            .parse()
            .unwrap();
    let call_params = CallParameters::new(Some(deposit_amount), Some(base_asset_id), None);
    let contract_methods = instance.methods();
    let lp_token_balance = wallet[0].get_asset_balance(&base_asset_id).await.unwrap();
    dbg!(lp_token_balance);
    //depositing asset to contract
    contract_methods
        .deposit(wallet[0].address().into())
        .call_params(call_params)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    let lp_asset_id = AssetId::from(*contract_id);
    let lp_token_balance = wallet[0].get_asset_balance(&lp_asset_id).await.unwrap();
    dbg!(lp_token_balance);
    
    //withdrawing asset
    let call_params = CallParameters::new(Some(lp_token_balance), Some(lp_asset_id), None);
    contract_methods
        .withdraw(wallet[0].address().into())
        .call_params(call_params)
        .append_variable_outputs(1)
        .call()
        .await
        .unwrap();

    let base_balance = wallet[0].get_asset_balance(&base_asset_id).await.unwrap();
    assert_eq!(base_balance, deposit_amount);
}
