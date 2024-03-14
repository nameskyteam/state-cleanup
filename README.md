# Clean the contract state
Clean the contract state using [near-cli-js](https://github.com/near/near-cli-js)

## Step
1. Compile the contract using `make all`
2. Deploy wasm to your account (e.g. `example.near`) using `near deploy example.near ./res/state_cleanup.wasm`
3. View the contact state using `near storage example.near --finality final`, you can find the keys with base64 encoding
4. Call `clean` method on `example.near` using `near call example.near clean '{"keys":[<Base64 State Keys>]}' --account-id example.near --gas 300000000000000`
