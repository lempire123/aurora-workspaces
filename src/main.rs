use aurora_workspace::types::output::TransactionStatus;
use aurora_workspace_demo::common;
use ethabi::Constructor;
use ethereum_tx_sign::{LegacyTransaction, Transaction};
use std::fs::File;
use std::fs::read_dir;
use std::clone::Clone;

// const ETH_RANDOM_HEX_PATH: &str = "./res/BYTECODE.hex";
// const ETH_RANDOM_ABI_PATH: &str = "./res/ABI.abi";
const PRIVATE_KEY: [u8; 32] = [88u8; 32];

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Create a sandbox environment.
    let worker = workspaces::sandbox().await?;
    // Init and deploy the Aurora EVM in sandbox.
    let evm = common::init_and_deploy_contract(&worker).await?;

    let paths = read_dir("./res/ABI/").unwrap();
    let num = paths.count();

    let mut addresses = Vec::new();
    let mut count: u32 = 0;

    for i in 0..num {

        // Set the contract.
        let contract = {
            let abi = File::open("./res/ABI/".to_owned() + &count.to_string() + ".abi")?; 
            let code = hex::decode(std::fs::read("./res/BYTECODE/".to_owned() + &count.to_string() + ".hex")?)?;
            EthContract::new(abi, code)
        };
        
        // Create a deploy transaction and sign it.
        let signed_deploy_tx = {
            let deploy_tx = contract.deploy_transaction(0, &[]);
            let ecdsa = deploy_tx.ecdsa(&PRIVATE_KEY).unwrap();
            deploy_tx.sign(&ecdsa)
        };

        // Submit the transaction and get the ETH address.
        let address = match evm
            .as_account()
            .submit(signed_deploy_tx)
            .max_gas()
            .transact()
            .await?
            .into_value()
            .into_result()?
        {
            TransactionStatus::Succeed(bytes) => {
                let mut address_bytes = [0u8; 20];
                address_bytes.copy_from_slice(&bytes);
                address_bytes
            }
            _ => panic!("Ahhhhhh"),
        };
        addresses.push(address);
        count += 1;
    }

    let contract = {
        let abi = File::open("./res/ABI/".to_owned() + &0.to_string() + ".abi")?; 
        let code = hex::decode(std::fs::read("./res/BYTECODE/".to_owned() + &0.to_string() + ".hex")?)?;
        EthContract::new(abi, code)
    };

    // let random_logic_contract = RandomLogic::new(contract, address);
    let wrapped_ether_contract = Wrappedether::new(contract, addresses[0]);

    // Fast forward a few blocks...
    worker.fast_forward(10).await?;

    let supply_tx = wrapped_ether_contract.get_supply_transaction(1);
    let ecdsa = supply_tx.ecdsa(&PRIVATE_KEY).unwrap();
    let signed_random_tx = supply_tx.sign(&ecdsa);
    if let TransactionStatus::Succeed(bytes) = evm  
        .as_account()
        .submit(signed_random_tx)
        .max_gas()
        .transact()
        .await?
        .into_value()
        .into_result()?
    {
        println!("Total Supply: {}", hex::encode(bytes));
    }

    Ok(())
}
struct Wrappedether {
    contract: EthContract,
    address: [u8; 20],
}

impl Wrappedether {
    pub fn new(contract: EthContract, address: [u8; 20]) -> Self {
        Self { contract, address }
    }

    pub fn get_supply_transaction(&self, nonce: u128) -> LegacyTransaction {
        let data = self 
            .contract
            .abi
            .function("totalSupply")
            .unwrap()
            .encode_input(&[])
            .unwrap();

            LegacyTransaction {
                chain: 1313161556,
                nonce,
                gas_price: Default::default(),
                to: Some(self.address),
                value: Default::default(),
                data,
                gas: u64::MAX as u128,
            }
    }
}
// struct RandomLogic {
//     contract: EthContract,
//     address: [u8; 20],
// }

// impl RandomLogic {
//     pub fn new(contract: EthContract, address: [u8; 20]) -> Self {
//         Self { contract, address }
//     }

//     pub fn compute_winner_transaction(&self, nonce: u128) -> LegacyTransaction {
//         let data = self
//             .contract
//             .abi
//             .function("computeWinner")
//             .unwrap()
//             .encode_input(&[])
//             .unwrap();

//             LegacyTransaction {
//                 chain: 1313161556,
//                 nonce,
//                 gas_price: Default::default(),
//                 to: Some(self.address),
//                 value: Default::default(),
//                 data,
//                 gas: u64::MAX as u128,
//             }
//     }
// }

struct EthContract {
    abi: ethabi::Contract,
    code: Vec<u8>,
}

impl EthContract {
    pub fn new(abi_file: File, code: Vec<u8>) -> Self {
        Self {
            abi: ethabi::Contract::load(abi_file).unwrap(),
            code,
        }
    }

    pub fn deploy_transaction(&self, nonce: u128, args: &[ethabi::Token]) -> LegacyTransaction {
        let data = self
            .abi
            .constructor()
            .unwrap_or(&Constructor { inputs: vec![] })
            .encode_input(self.code.clone(), args)
            .unwrap();

        LegacyTransaction {
            chain: 1313161556,
            nonce,
            gas_price: Default::default(),
            to: None,
            value: Default::default(),
            data,
            gas: u64::MAX as u128,
        }
    }
}
