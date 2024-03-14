# Clean the contract state
Clean the contract state using [NEAR CLI JS](https://github.com/near/near-cli)

## Step
1. Compile the contract with `make all`
2. Deploy wasm to your account (e.g. `example.near`) with `near deploy example.near ./res/state_cleanup.wasm --networkId mainnet`
3. View the contact state with `near storage example.near --finality optimistic --networkId mainnet`, you can find the keys with base64 encoding
4. Call `clean` method on `example.near` with `near call example.near clean '{"keys":[<Base64 State Keys>]}' --networkId mainnet --account-id example.near --gas 300000000000000`
5. You can check whether state keys are deleted with `near storage example.near --finality optimistic` again

Sometimes you may meet the error `State of contract example.near is too large to be viewed`, this is RPC issue, so you need to change RPC which allows unlimited view (e.g. https://beta.rpc.mainnet.near.org) with `NEAR_MAINNET_RPC=https://beta.rpc.mainnet.near.org near storage example.near --finality optimistic`
