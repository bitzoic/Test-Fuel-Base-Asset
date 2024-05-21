contract;

use standards::src20::SRC20;
use std::string::String;

abi MyContract {
    fn total_assets(contract_id: ContractId) -> u64;
    fn total_supply(contract_id: ContractId) -> Option<u64>;
    fn name(contract_id: ContractId) -> String;
    fn symbol(contract_id: ContractId) -> String;
    fn decimals(contract_id: ContractId) -> u8;
}

impl MyContract for Contract {
    fn total_assets(contract_id: ContractId) -> u64 {
        let base_asset_abi = abi(SRC20, contract_id.bits());

        base_asset_abi.total_assets()
    }

    fn total_supply(contract_id: ContractId) -> Option<u64> {
        let base_asset = AssetId::base();
        let base_asset_abi = abi(SRC20, contract_id.bits());

        base_asset_abi.total_supply(base_asset)
    }

    fn name(contract_id: ContractId) -> String {
        let base_asset = AssetId::base();
        let base_asset_abi = abi(SRC20, contract_id.bits());
  
        base_asset_abi.name(base_asset).unwrap()
    }

    fn symbol(contract_id: ContractId) -> String {
        let base_asset = AssetId::base();
        let base_asset_abi = abi(SRC20, contract_id.bits());

        base_asset_abi.symbol(base_asset).unwrap()
    }

    fn decimals(contract_id: ContractId) -> u8 {
        let base_asset = AssetId::base();
        let base_asset_abi = abi(SRC20, contract_id.bits());

        base_asset_abi.decimals(base_asset).unwrap()
    }
}
