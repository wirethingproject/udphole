# udphole

## Usage

    udphole action ip:port

### Actions

    listen  Start a server that listens for requests and returns the public
            ip and port of the source.
    punch   Punch a UDP NAT hole by making a request to a server and prints
            the private and public ip and port in this order.

## Build

Install Rust https://www.rust-lang.org/tools/install and run:

    cargo build --release

## Install

    cargo install --path .

## Run

### Start a server

    cargo run listen 0.0.0.0:6094

Any port can be used, `6094` was chosen for example purposes.

### Punch a hole

    cargo run punch udphole.fly.dev:6094
    192.168.0.100:44266 # private ip and port
    208.60.21.109:14554 # public ip and port

## Docker

### Build

    docker build -t udphole:local -f Dockerfile .

### Listen

    docker run --rm \
        -p 6094:6094/udp --name udphole
        udphole:local listen 0.0.0.0:6094

As of 2023-10, it does not work on `macOS + colima` because currently `colima`
does not expose UDP ports.

#### Stop listening

    docker kill udphole

It is necessary to use `docker kill` to stop because `contro+c` is
not terminating the process.

### Punch

    docker run --rm \
        udphole:local punch udphole.fly.dev:6094

Use this command only for testing purpose, it has no practical use because
the container is terminated after execution.
