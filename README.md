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

Create a lambda function with a runtime of "provided.al2", the use its name in the command below in place of "
RuntimeTest". If you are using the default aws cli profile, you can remove the profile flag. Otherwise, set it to your
profile name.

```
$ cp ./target/x86_64-unknown-linux-gnu/release/fbaas-rust ./bootstrap && \
     zip lambda.zip bootstrap && \
     rm bootstrap
$ aws --profile=saac02-playground-iamadmin \
      lambda update-function-code \
      --function-name RuntimeTest \
      --zip-file fileb://lambda.zip
```