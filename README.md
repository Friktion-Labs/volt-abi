# Friktion (volt) ABI

## Overview

Provides contexts and instruction functions to interact with Friktion Program via CPI calls

Friktion program ID: ```VoLT1mJz1sbnxwq5Fv2SXjdVDgPXrb9tJyC8WpMDkSp```

Keep in mind, the CPI feature must be enabled on "volt-abi" as the following demonstrates:

```rust
volt-abi = { version="0.2.0", features = ["cpi"]}
```

## Examples

To see an example of this ABI in action, see our [CPI Examples](https://github.com/Friktion-Labs/lightning)

## Practical Notes:

- vault_token_destination && underlying_token_source accounts must be initialized prior to calling the Deposit instruction. The authorities on those token accounts must be the dao_authority, which practically is a PDA of the invoking program.