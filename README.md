# fbaas-rust

FizzBuzz as a service, written in Rust.

## Build

This uses the [SergioBenitez/osxct](https://github.com/SergioBenitez/homebrew-osxct) cross-compilation toolkit to
support compiling for the x86_64-unknown-linux-gnu target triple on MacOS.

For this target to work correctly in lambda, you must use the "provided.al2" runtime.

```
$ brew tap SergioBenitez/osxct
$ brew install x86_64-unknown-linux-gnu
$ echo "[target.x86_64-unknown-linux-gnu]\nlinker = \"x86_64-unknown-linux-gnu-gcc\"" >> ~/.cargo/config
$ rustup target install x86_64-unknown-linux-gnu
$ cargo build --target x86_64-unknown-linux-gnu --release
```

## Deploy

This project uses AWS SAM cli to manage deployment to AWS Lambda.
`sam deploy` uses the aws cli default profile by default, so if you need to use a different profile, you can pass `--profile <your_profile>` to change it.

```
$ brew tap aws/tap
$ brew install aws-sam-cli
$ sam build
$ sam deploy --guided --profile saac02-playground-iamadmin
```