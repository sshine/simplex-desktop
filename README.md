# simplex-desktop

[![Rust CI](https://github.com/sshine/simplex-desktop/actions/workflows/main.yml/badge.svg)](https://github.com/sshine/simplex-desktop/actions)

A desktop application for [simplex-chat](https://simplex.chat). WIP, contributions are welcome.

## Architecture

For the back end we rust with [tauri](https://tauri.app) and frontend is built with [yew-rs](https://yew.rs)

## For developers

1. You will need Rust; you can most easily install this with [rustup][rustup]:
    ```shell
    curl --proto '=https' --tlsv1.2 https://sh.rustup.rs -sSf | sh
    ```
1. You will need some software libraries; installing these may vary depending on your operating
   system; a recipe for Ubuntu is given here, but Tauri's [Getting Started][tauri-getting-started]
   has steps for each popular platform.
   ```shell
   sudo apt install -y libgtk-3-dev libsoup2.4-dev libjavascriptcoregtk-4.0-dev libwebkit2gtk-4.0-dev
   ```
2. Then you will need Yew; find detailed instuctions in Yew's [Getting Started][yew-getting-started]:
    ```shell
    rustup target add wasm32-unknown-unknown
    cargo install --locked trunk
    cargo install wasm-bindgen-cli
    ```
3. And you will need the Tauri CLI:
    ```shell
    cargo install --locked tauri-cli
    ```

Now you should be able to to run the app with
```shell
cargo tauri dev
```

[tauri-getting-started]: https://tauri.app/v1/guides/getting-started/prerequisites#installing
[yew-getting-started]: https://yew.rs/docs/getting-started/introduction
[rustup]: https://rustup.rs/

## Join the discussion

You can join the simplex group [`#simplex-desktop`][simplex-desktop-group] by clicking the link, or by scanning this QR code in the app:

<div align="center">
  <img src="img/simplex-desktop-group-qr.png" alt="#simplex-desktop" width="33%">
</div>

[simplex-desktop-group]: https://simplex.chat/contact#/?v=1-2&smp=smp%3A%2F%2FSkIkI6EPd2D63F4xFKfHk7I1UGZVNn6k1QWZ5rcyr6w%3D%40smp9.simplex.im%2FB_usnNovum0Jm125FYrsWfQQcxSjuLHd%23%2F%3Fv%3D1-2%26dh%3DMCowBQYDK2VuAyEAVX4l3f9oqnGb_Ebj8bCiYKgCVb5MHc-EYsM54IMSB3I%253D%26srv%3Djssqzccmrcws6bhmn77vgmhfjmhwlyr3u7puw4erkyoosywgl67slqqd.onion&data=%7B%22type%22%3A%22group%22%2C%22groupLinkId%22%3A%22ILhMYy-DR6l5KK3dfrpx9Q%3D%3D%22%7D
