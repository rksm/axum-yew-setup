_This is a template for [cargo-generate](https://cargo-generate.github.io/cargo-generate/)._
_Use with `cargo generate rksm/axum-yew-template`._

# {{project-name}}

This is a full stack Rust web app using [axum](https://github.com/tokio-rs/axum) and [yew](https://yew.rs/).

## Usage

Run the dev version (auto-reloads server & client on file change) with `./dev.sh`.

Run the pre-compiled version with `./prod.sh`.

The app will start at http://localhost:8080 by default. You can modify that by changing the flags passed to the server binary:

```
Usage: server [OPTIONS]

Options:
  -l, --log <LOG_LEVEL>          set the log level [default: debug]
  -a, --addr <ADDR>              set the listen addr [default: ::1]
  -p, --port <PORT>              set the listen port [default: 8080]
      --static-dir <STATIC_DIR>  set the directory where static files are to be found [default: ../dist]
  -h, --help                     Print help
```
