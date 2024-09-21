<picture>
    <source srcset="https://raw.githubusercontent.com/vavakado/maerbooru/main/public/logo-cropped_inverted.png" media="(prefers-color-scheme: dark)">
    <img src="https://raw.githubusercontent.com/vavakado/maerbooru/main/public/logo-cropped.png" alt="Maerbooru Logo">
</picture>

# Maerbooru

This is a work-in-progress booru style imageboard.

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

## Testing Your Project

TODO: figure out how to set ssr leptos

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
