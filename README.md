# Clean the contract state
Clean the contract state using [NEAR CLI JS](https://github.com/near/near-cli)

## Step
1. Compile the state-cleanup contract
    ```shell
    make all
    ```
2. Deploy wasm to your account (e.g. `example.near`)
    ```shell
    near deploy example.near ./res/state_cleanup.wasm --networkId mainnet
    ```
3. View the contact state, you can find the state keys with base64 encoding
    ```shell
    near storage example.near --finality optimistic --networkId mainnet
    ```
4. Call `clean` method on `example.near`
    ```shell
    near call example.near clean '{"keys":[<base64 state keys>]}' --networkId mainnet --account-id example.near --gas 300000000000000
    ```

Sometimes you may meet the error `State of contract example.near is too large to be viewed`, this is RPC issue, so you need to change RPC which allows unlimited view
```shell
NEAR_MAINNET_RPC=https://beta.rpc.mainnet.near.org near storage example.near --finality optimistic
```
