# Stunts for Web

```sh
cargo install trunk leptosfmt
```

You can add the `wasm` compilation target to rust using
```sh
rustup target add wasm32-unknown-unknown
```


## Developing with Leptos

```sh
trunk serve --port 3001
```

will open your app in your default browser at `http://localhost:3000`.


## Deploying with Leptos

To build a Leptos CSR app for release, use the command

```sh
trunk build --release
```

This will output the files necessary to run your app into the `dist` folder; you can then use any static site host to serve these files.

For further information about hosting Leptos CSR apps, please refer to [the Leptos Book chapter on deployment available here][deploy-csr].


[Leptos]: https://github.com/leptos-rs/leptos

[Trunk]: https://github.com/trunk-rs/trunk
[Trunk-instructions]: https://trunkrs.dev/assets/

[deploy-csr]: https://book.leptos.dev/deployment/csr.html