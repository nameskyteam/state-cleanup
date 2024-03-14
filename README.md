# Clean contract state
Clean contract state with [NEAR CLI JS](https://github.com/near/near-cli)

## Step
1. Compile the state-cleanup contract
    ```shell
    make all
    ```
2. Deploy the state-cleanup contract to the account (e.g. example.near)
    ```shell
    near deploy example.near ./res/state_cleanup.wasm --networkId mainnet
    ```
3. View the contact state and you can find the state keys with base64 encoding
    ```shell
    near storage example.near --finality optimistic --networkId mainnet
    ```
4. Clean the state keys
    ```shell
    near call example.near clean '{"keys":[<base64 state keys>]}' --networkId mainnet --account-id example.near --gas 300000000000000
    ```

Sometimes you may meet the error `State of contract example.near is too large to be viewed`. This is RPC issue, so you need to use the RPC that allows unlimited view
```shell
NEAR_MAINNET_RPC=https://beta.rpc.mainnet.near.org near storage example.near --finality optimistic
```
