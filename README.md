# fbaas-rust

FizzBuzz as a service, written in Rust, deployed as an AWS Lambda function using AWS SAM CLI.

## Initial Manual Configuration

1. Add your custom domain as a Route53 Hosted Zone and update the `HostedZoneId` referenced in the template. The AWS
   account used to run SAM CLI will need access to add/modify records in this hosted zone when run.
2. Add a new wildcard certificate in Certificate Manager matching your custom domain, and update the `CertificateArn`
   referenced in the template. See the article "[How do I define a custom domain name for my API Gateway API?][cdn]"
   for nuances around the use of the "EDGE" vs "REGIONAL" `EndpointConfiguration` setting.
3. Replace the `DomainName` referenced in the template with your custom subdomain. 

[cdn]: https://aws.amazon.com/premiumsupport/knowledge-center/custom-domain-name-amazon-api-gateway/

## Build

This uses the [SergioBenitez/osxct](https://github.com/SergioBenitez/homebrew-osxct) cross-compilation toolkit to
support compiling for the x86_64-unknown-linux-gnu target triple on MacOS.

For this target to work correctly in lambda, you must use the "provided.al2" runtime.

```
$ brew tap SergioBenitez/osxct
$ brew install x86_64-unknown-linux-gnu
$ echo "[target.x86_64-unknown-linux-gnu]\nlinker = \"x86_64-unknown-linux-gnu-gcc\"" >> ~/.cargo/config
$ rustup target install x86_64-unknown-linux-gnu
$ brew tap aws/tap
$ brew install aws-sam-cli
$ make build
```

## Deploy

This project uses AWS SAM cli to manage the deployments. `sam deploy` uses the aws cli default profile by default, so
if you need to use a different profile, you can pass `--profile <your_profile>` to change it.

```
$ sam deploy --guided --profile <my custom profile>
# ...later, once you've saved your deploy config, you can use make
$ make deploy
```