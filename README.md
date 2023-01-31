# aurora-workspaces

Improvements to original workspace:

- User can paste a list of addresses of contracts on aurora mainnet they would like to include in their tests
- They can then run ```node src/scripts/getABI.js``` which will get the abi and bytecode (creation code) for all contracts and paste them in the ```res/``` directory.
- The rust program will then deploy all contracts sequentially.
