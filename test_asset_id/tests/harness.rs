use fuels::{crypto::SecretKey, prelude::*, types::{Bytes32, ContractId}};
use std::{fs, str::FromStr};
use tokio::time::{sleep, Duration};

// Load abi from json
abigen!(Contract(
    name = "MyContract",
    abi = "out/release/test_asset_id-abi.json"
),
Contract(
    name = "BaseAssetContract",
    abi = "../../fuel-bridge/packages/base-asset/out/release/base-asset-contract-abi.json"
)
);

async fn get_contract_instance() -> (MyContract<WalletUnlocked>, ContractId, BaseAssetContract<WalletUnlocked>, ContractId, AssetId) {
    // Create a provider pointing to the testnet.
    let provider = Provider::connect("devnet.fuel.network").await.unwrap();

    let base_asset = provider.base_asset_id();
    assert_eq!(*base_asset, AssetId::from_str("0xf8f8b6283d7fa5b672b530cbb84fcccb4ff8dc40f8176ef4544ddb1f1952ad07").unwrap());
    
    // Setup the private key.
    let secret = SecretKey::from_str(
        "DO NOT REVEAL",
    ).unwrap();

    // Create the wallet.
    let wallet = WalletUnlocked::new_from_private_key(secret, Some(provider.clone()));
    let balance: u64 = wallet.get_asset_balance(&base_asset).await.unwrap();
    fs::write("balance.txt", balance.to_string()).expect("Unable to write file");

    assert_eq!(wallet.address().hash(), Bytes32::from_str("0xacc4e1aac25a325e250a7974ee50d9904d6bf32f475136513ff2dc3a5211fb2c").unwrap());
    assert!(balance != 0);

    // let id = Contract::load_from(
    //     "./out/release/test_asset_id.bin",
    //     LoadConfiguration::default(),
    // )
    // .unwrap()
    // .deploy(&wallet, TxPolicies::default())
    // .await
    // .unwrap();
    // fs::write("contractid.txt", id.to_string()).expect("Unable to write file");

    let id: Bech32ContractId =
            "fuel1x60w0parct2h5jjsar66k0qw770f3r8uketd6pflph2uj7klejaqdv6s4s".parse().unwrap();
    
    let instance = MyContract::new(id.clone(), wallet.clone());

    let base_contract_id: ContractId =
            "0x7e2becd64cd598da59b4d1064b711661898656c6b1f4918a787156b8965dc83c".parse().unwrap();

    let base_contract_instance = BaseAssetContract::new(base_contract_id, wallet);

    (instance, id.into(), base_contract_instance, base_contract_id, *base_asset)
}

#[tokio::test]
async fn can_get_total_assets() {
    let (instance, _id, base_instance, base_contract_id, _base_asset) = get_contract_instance().await;

    let tx_policies = TxPolicies::default()
        .with_tip(1000)
        .with_script_gas_limit(10_000_000)
        .with_maturity(0);

    // Now you have an instance of your contract you can use to test each function
    let total_asset = base_instance
        .methods()
        .total_assets()
        .with_tx_policies(tx_policies) // Chain the tx policies.
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(total_asset, 1);
}

#[tokio::test]
async fn can_get_total_supply() {
    let (instance, _id, base_instance, base_contract_id, base_asset) = get_contract_instance().await;

    sleep(Duration::from_millis(10000)).await;

    let tx_policies = TxPolicies::default()
        .with_tip(1000)
        .with_script_gas_limit(10_000_000)
        .with_maturity(0);

    // Now you have an instance of your contract you can use to test each function
    let total_supply = base_instance
        .methods()
        .total_supply(base_asset)
        .with_tx_policies(tx_policies) // Chain the tx policies.
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(total_supply, None);
}

#[tokio::test]
async fn can_get_name() {
    let (instance, _id, base_instance, base_contract_id, base_asset) = get_contract_instance().await;

    sleep(Duration::from_millis(20000)).await;


    let tx_policies = TxPolicies::default()
        .with_tip(1000)
        .with_script_gas_limit(10_000_000)
        .with_maturity(0);

    // Now you have an instance of your contract you can use to test each function
    let name = base_instance
        .methods()
        .name(base_asset)
        .with_tx_policies(tx_policies) // Chain the tx policies.
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(name, Some(String::from("Ether")));
}

#[tokio::test]
async fn can_get_symbol() {
    let (instance, _id, base_instance, base_contract_id, base_asset) = get_contract_instance().await;

    sleep(Duration::from_millis(30000)).await;


    let tx_policies = TxPolicies::default()
        .with_tip(1000)
        .with_script_gas_limit(10_000_000)
        .with_maturity(0);

    // Now you have an instance of your contract you can use to test each function
    let symbol = base_instance
        .methods()
        .symbol(base_asset)
        .with_tx_policies(tx_policies) // Chain the tx policies.
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(symbol, Some(String::from("ETH")));
}

#[tokio::test]
async fn can_get_decimals() {
    let (instance, _id, base_instance, base_contract_id, base_asset) = get_contract_instance().await;

    sleep(Duration::from_millis(40000)).await;

    let tx_policies: TxPolicies = TxPolicies::default()
        .with_tip(1000)
        .with_script_gas_limit(10_000_000)
        .with_maturity(0);

    // Now you have an instance of your contract you can use to test each function
    let decimals = base_instance
        .methods()
        .decimals(base_asset)
        .with_tx_policies(tx_policies) // Chain the tx policies.
        .call()
        .await
        .unwrap()
        .value;

    assert_eq!(decimals, Some(9u8));
}

#[tokio::test]
async fn can_get_name_from_contract() {
    let (instance, _id, base_instance, base_contract_id, base_asset) = get_contract_instance().await;

    sleep(Duration::from_millis(50000)).await;

    let tx_policies = TxPolicies::default()
        .with_tip(10000)
        .with_script_gas_limit(10_000_000)
        .with_maturity(0);

    // Now you have an instance of your contract you can use to test each function
    let name = instance
        .methods()
        .name(base_contract_id)
        .with_contracts(&[&base_instance])
        .with_tx_policies(tx_policies) // Chain the tx policies.
        .call()
        .await
        .unwrap();

    assert_eq!(name.value, String::from("Ether"));
}

#[tokio::test]
async fn can_get_total_supply_from_contract() {
    let (instance, _id, base_instance, base_contract_id, base_asset) = get_contract_instance().await;

    sleep(Duration::from_millis(60000)).await;

    let tx_policies = TxPolicies::default()
        .with_tip(10000)
        .with_script_gas_limit(10_000_000)
        .with_maturity(0);

    // Now you have an instance of your contract you can use to test each function
    let total_supply = instance
        .methods()
        .total_supply(base_contract_id)
        .with_contracts(&[&base_instance])
        .with_tx_policies(tx_policies) // Chain the tx policies.
        .call()
        .await
        .unwrap();

    assert_eq!(total_supply.value, None);
}

#[tokio::test]
async fn can_get_total_assets_from_contract() {
    let (instance, _id, base_instance, base_contract_id, base_asset) = get_contract_instance().await;

    sleep(Duration::from_millis(70000)).await;

    let tx_policies = TxPolicies::default()
        .with_tip(10000)
        .with_script_gas_limit(10_000_000)
        .with_maturity(0);

    // Now you have an instance of your contract you can use to test each function
    let total_assets = instance
        .methods()
        .total_assets(base_contract_id)
        .with_contracts(&[&base_instance])
        .with_tx_policies(tx_policies) // Chain the tx policies.
        .call()
        .await
        .unwrap();

    assert_eq!(total_assets.value, 1);
}

#[tokio::test]
async fn can_get_symbol_from_contract() {
    let (instance, _id, base_instance, base_contract_id, base_asset) = get_contract_instance().await;

    sleep(Duration::from_millis(80000)).await;

    let tx_policies = TxPolicies::default()
        .with_tip(10000)
        .with_script_gas_limit(10_000_000)
        .with_maturity(0);

    // Now you have an instance of your contract you can use to test each function
    let symbol = instance
        .methods()
        .symbol(base_contract_id)
        .with_contracts(&[&base_instance])
        .with_tx_policies(tx_policies) // Chain the tx policies.
        .call()
        .await
        .unwrap();

    assert_eq!(symbol.value, String::from("ETH"));
}

#[tokio::test]
async fn can_get_decimals_from_contract() {
    let (instance, _id, base_instance, base_contract_id, base_asset) = get_contract_instance().await;

    sleep(Duration::from_millis(90000)).await;

    let tx_policies = TxPolicies::default()
        .with_tip(10000)
        .with_script_gas_limit(10_000_000)
        .with_maturity(0);

    // Now you have an instance of your contract you can use to test each function
    let decimals = instance
        .methods()
        .decimals(base_contract_id)
        .with_contracts(&[&base_instance])
        .with_tx_policies(tx_policies) // Chain the tx policies.
        .call()
        .await
        .unwrap();

    assert_eq!(decimals.value, 9u8);
}
