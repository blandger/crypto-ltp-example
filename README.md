# crypto-ltp-example
Simple example accessing crypto stock from rest api server

# Prerequisite installation

### Install Rust compiler
Use any suitable approach to [Rust compiler](https://www.rust-lang.org/tools/install). Then check it using command below

>cargo --version

The output should be something like :

```
cargo 1.77.2 (e52e36006 2024-03-26)
```

## Docker (with docker compose)

### Install Docker + docker compose tools into your system
See how to install it [from Docker guide/doc](https://docs.docker.com/engine/install/)

## Build docker image

> docker build -t crypto-ltp-example:v1 .

## Run docker image

> docker run -p 8080:8080 crypto-ltp-example:v1

## Testing REST API
Deployed application API is accessible by URL: http://127.0.0.1:8080/api/v1/ltp

