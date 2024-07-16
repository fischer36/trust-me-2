# Trust Me 2
"Trust-me-2" is an attribute macro that allows functions unsafe access without requiring any redundant unsafe blocks, even for calling the function. The "two" represents the cutting-edge improvement from existing trust-me macros that still requires redundant `{}` blocks. 

## Usage
- Add `trust-me-2` to your `Cargo.toml`.
- Then simply mark any function with `#[trust_me]` Refer to the example for further clarification.
