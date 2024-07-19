# Trust Me 2
An attribute macro that adds an **unsafe** block around a function body. Inspired by the iconic [Trust Me Macro](https://www.youtube.com/watch?v=TGfQu0bQTKc), the additional '2' represents the **cutting-edge** improvement from pre-existing unsafe macros that still requires a {} block.
## Usage
- Install
```Bash
cargo add trust-me-2
```
- Add attribute
```Rust
#[trust_me]
fn secretly_unsafe() {
    // Unsafe code 
}
```
