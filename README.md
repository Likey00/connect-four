# connect-four
Rust web app using Yew to play connect four against an AI opponent

# How to run locally
First, download rust from [here](https://www.rust-lang.org/tools/install)

Next, use cargo to install trunk by running:
```cargo install trunk```

Finally, add the WASM build target by running:
```rustup target add wasm32-unknown-unknown```

Now you can run the app locally from the project folder with:
```trunk serve --open --release```
