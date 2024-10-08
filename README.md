<picture>
    <img src="https://raw.githubusercontent.com/vavakado/maerbooru/main/public/logo-cropped.png" alt="Maerbooru Logo">
</picture>

# Maerbooru

[![Rust CI/CD](https://github.com/vavakado/maerbooru/actions/workflows/rust.yml/badge.svg)](https://github.com/vavakado/maerbooru/actions/workflows/rust.yml)
[![Docker Image CI](https://github.com/vavakado/maerbooru/actions/workflows/docker-image.yml/badge.svg)](https://github.com/vavakado/maerbooru/actions/workflows/docker-image.yml)

This is a work-in-progress booru style imageboard.

## TODO

- [ ] Add posting
  - [ ] Image upload page
  - [ ] post grid page
  - [ ] individual post page
- [ ] impove tag naming regex ( allow tags like lain\_(serial_experements_lain) or see-through)
- [ ] add proper documentation comments.
- [ ] auth

## Running

To run you have to have nightly rust installed, and wasm target added.

```bash
npm install // only once to install tailwind
cargo leptos watch
```

## Compiling for Release

```bash
cargo leptos build --release
```

This will generate the server binary into target/server/release and site package in target/site

## Executing the Server on a Remote Machine Without the Toolchain

### Docker

I still have no image uploaded to a container repo so you will have to build the image yourself

```bash
docker build .
```

### Bare-metal

After running a `cargo leptos build --release` the minimum files needed are:

1. The server binary located in `target/server/release`
2. The `site` directory and all files within located in `target/site`

Copy these files to your remote server. The directory structure should be:

```text
maerbooru
site/
```

Set the following environment variables (updating for your project as needed):

```text
LEPTOS_OUTPUT_NAME="maerbooru"
LEPTOS_SITE_ROOT="site"
LEPTOS_SITE_PKG_DIR="pkg"
LEPTOS_SITE_ADDR="127.0.0.1:3000"
LEPTOS_RELOAD_PORT="3001"
```

Finally, run the server binary.
