### Commands

```sh
rustup default nightly-2025-03-29-aarch64-apple-darwin
cargo +nightly-2025-03-29 install --git https://github.com/facebook/buck2.git buck2
buck2 run root//buck2_lab/greeter_bin:main
buck2 test root//buck2_lab/greeter_lib:test
buck2 log what-ran
```
