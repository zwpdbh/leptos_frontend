# Leptos for Client Side Rendering(CRS)

## Setup frontend project in workspace

```sh 
# In workspace 
cargo new frontend 
cd frontend 

cargo install trunk
cargo add leptos --features=csr,nightly

rustup target add wasm32-unknown-unknown
```

## Create Hello World 

- Create a simple `index.html` in the **root of the frontend** directory.

- Modify `src/main.rs` to test leptos.

- Run `trunk serve --open`
  
  - The first time will be slow for complation.

- See [Hello World! Getting Set up for Leptos CSR Development](https://book.leptos.dev/getting_started/index.html#hello-world-getting-set-up-for-leptos-csr-development)

## How to customize leptos configuration

Edit the `Cargo.toml` file:

```toml
[package.metadata.leptos]
# On which port to serve the client side rendered site (when using --csr option)
csr_port = 8000
# The port to use for automatic reload monitoring
reload_port = 3001
```

- This metadata is not interpreted by Cargo itself but can be used by other tools or libraries that integrate with Cargo.
- The `[package.metadata.leptos]` section is an example of such metadata, likely used by a specific tool or library called leptos.

## Add Bulma CSS

- Add dependencies
  
  ```toml
  [dependencies]
  leptos-bulma = { version = "0.4.0", features = ["build-script"] }
  ```

- Follow [How to Install in Leptos CSR (Trunk)](https://leptos-bulma.fermyon.app/guides#how-to-install-csr)
  - In `frontend` project folder, run `cargo run --bin  build-leptos-bulma`.
  - Run `trunk serve`.
  - Then, edit `Trunk.toml`:

  ```toml
  [serve]
  port = 3002

  [watch]
  ignore = ["./style", "./dist", "./target"]

  [[hooks]]
  stage = "pre_build"
  command = "cargo"
  command_arguments = ["run", "--bin", "build-leptos-bulma"]
  ```

  At last, visit `http://localhost:3002/`.
  
## References

- [Letpos CSR](https://book.leptos.dev/getting_started/index.html)
  - [Leptos is becoming best rust web framwork and How to set up(may outdated)](https://github.com/leptos-rs/leptos/discussions/125)
- [leptos-bulma](https://crates.io/crates/leptos-bulma)
  - [How to Install in Leptos CSR (Trunk)](https://leptos-bulma.fermyon.app/guides#how-to-install-csr)
