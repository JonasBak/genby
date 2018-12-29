# genby
Using rust to simulate an environment, visualized in js by compiling rust to wasm.

## Setup
Requires `rustup`, `rustc`, `cargo`, `wasm-pack` and `yarn`

```
git clone git@github.com:JonasBak/genby.git
cd genby
```

To compile the rust sources into wasm and generate js api, run:
* `wasm-pack build`

To install all frontend dependencies:
* Run `yarn` in genby/www

To use the newly generated package in js:
* Run `yarn link` in genby/pkg
* Run `yarn link genby` in genby/www

## Run
`yarn start` in genby/www

## Rebuild
`wasm-pack build`
