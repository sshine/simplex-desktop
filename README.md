# Simplex-desktop

A desktop application for [simplex-chat](https://simplex.chat). WIP, contributions are welcome.

## Architecture

For the back end we rust with [tauri](https://tauri.app) and frontend is built with [yew-rs](https://yew.rs)

## For developers

To get started with development you need a bunch of things, what is missing here can be found [here](https://tauri.app/v1/guides/getting-started/prerequisites#installing). 

1. You will need rust, for linux use rustup:

    ```shell
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    ```
1. Then you'll need tools for yew. Find detailed instuctions [here](https://yew.rs/docs/getting-started/introduction)
    ```shell
    rustup target add wasm32-unknown-unknown
    cargo install trunk
    cargo install wasm-bindgen-cli
    ```
1. And at least the tauri cli
    ```shell
    cargo install tauri-cli
    ```

Now you should be able to to run the app with
```shell
cargo tauri dev
```

