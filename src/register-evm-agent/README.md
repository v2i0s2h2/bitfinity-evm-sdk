# Register EVM Agent

A Cli tool for generating an ETH Wallet & reserving a canister to the EVM canister

## Build

Run the commands below to install the binary.

```sh
cargo install register-evm-agent
```

## Usage

For general information how to use the cli, run below

```sh
register-evm-agent --help
```

### Generate wallet

If you need to generate a wallet first, you can run

```sh
register-evm-agent generate-wallet
```

The command output will display the generated wallet info like this:

```txt
Wallet:
  Private Key = 048f4682aa84d9c92f4452956896e459a5d8b675895ca0a7dca6028641256c12
  Public Key = 0219aa742ea1020079d2f503d754db1d0c76e240ab2c43bcf71ab1ca91a099c13b
  Address = 0x6d4662d3ab4769a4f10781325601db68874261d2
```

### Reserve an EVM address

To reserve an address the following command needs to be run

```sh
register-evm-agent reserve -k <private_key> -g <gas price> -n <network> -i <identity_path> --evm <evm_canister_principal> --canister-id <reserve_canister_principal>
```

Where:

- `private key` is the Private key for the generated wallet
- `gas_price` is the gas price for the transaction that will be used as a proof.
- `network` is the network to run against: default is `local`, the value can be both `ic` or a custom URL.
- `identity path` is the path to the identity you're going to use to reserve your canister
- `evm canister principal` is the principal for the EVM canister
- `reserve canister principal` is the principal of the canister you're going to associate to the reserved address

All the supported options can be seen with

```sh
register-evm-agent reserve --help
```

#### Additional options

- **Amount to mint**: if you're using a testnet and you need to mint native tokens to your wallet first, you can pass the amount of tokens you need to mint to your wallet before reserving the canister

    ```sh
    register-evm-agent reserve -k ... -a 1000000000 ...
    ```

- **Specify the chain id**: you can specify the cain id providing the id as an argument

    ```sh
    register-evm-agent reserve -k ... -C <custom-chain-id>
    ```


    ## ----------------------------------------------------------------

    Zaroor, main aapko "register-evm-agent" ke baare mein aur zyada detail se samjhata hoon.

"register-evm-agent" module ka main kaam EVM (Ethereum Virtual Machine) agents ko Internet Computer (IC) network par register karna hai. Ye process network identification aur integration mein madad karta hai. Main ise step-by-step explain karta hoon:

1. Agent Registration:
   - Ye module EVM addresses ko IC principals ke saath map karta hai.
   - `ReservationService` struct (reservation.rs file mein) is process ko handle karta hai.

2. Network Identification:
   - `network_url` function (cli.rs file mein) different networks ko identify karta hai:
     ```rust
     pub(crate) fn network_url(network: &str) -> &str {
         match network {
             NETWORK_LOCAL => "http://localhost:8000",
             NETWORK_IC => "https://ic0.app",
             url => url,
         }
     }
     ```
   - Ye local development, IC mainnet, ya custom networks ko support karta hai.

3. Agent Initialization:
   - `init_agent` function (mentioned in cli.rs) agent ko initialize karta hai:
     ```rust
     let agent = init_agent(&self.identity, network, None).await?;
     ```

4. Reservation Process:
   - `ReservationService::reserve` method EVM address ko IC principal ke saath reserve karta hai:
     ```rust
     pub async fn reserve(&self) -> Result<()> {
         self.reserve_ic_agent().await?;
         Ok(())
     }
     ```

5. Transaction Sending:
   - Ye process ek transaction bhejta hai jisme IC principal ka data hota hai:
     ```rust
     let tx = TransactionBuilder {
         // ... other fields ...
         input: self.reserve_canister_id.as_slice().to_vec(),
         // ... other fields ...
     }.calculate_hash_and_build()?;
     ```

6. Confirmation:
   - Transaction ke baad, ye address reservation ko confirm karta hai:
     ```rust
     self.client
         .reserve_address(self.reserve_canister_id, tx_hash)
         .await??;
     ```

Is tarah se, "register-evm-agent" module EVM addresses ko IC network par register karta hai, jisse dono networks ke beech seamless interaction possible hota hai. Ye process network identification mein help karta hai kyunki ye ensure karta hai ki har EVM address ek unique IC principal se mapped ho, jo IC network par uski identity establish karta hai.