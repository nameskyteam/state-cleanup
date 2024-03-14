# Clean contract state
Clean contract state with [NEAR CLI JS](https://github.com/near/near-cli)

## Step
1. Compile state-cleanup contract
    ```shell
    make all
    ```
2. Deploy state-cleanup contract to account (e.g. example.near)
    ```shell
    near deploy example.near ./res/state_cleanup.wasm --networkId mainnet
    ```
3. View contact state and you can find state keys with base64 encoding
    ```shell
    near storage example.near --finality optimistic --networkId mainnet
    ```
4. Clean state keys
    ```shell
    near call example.near clean '{"keys":[<base64 state keys>]}' --networkId mainnet --account-id example.near --gas 300000000000000
    ```

Sometimes you may meet the error `State of contract example.near is too large to be viewed`. This is RPC issue, so you need to use the RPC that allows unlimited view
```shell
NEAR_MAINNET_RPC=https://beta.rpc.mainnet.near.org near storage example.near --finality optimistic
```
