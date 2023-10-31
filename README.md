# udphole

## Usage

    udphole action ip:port

### Actions

    listen  Start a server that listens for requests and returns the public
            ip and port of the source.
    punch   Punch a UDP NAT hole by making a request to a server and prints
            the private and public ip and port in this order.

## Examples

    # Start a server
    udphole listen 0.0.0.0:53000

    # Punch a hole
    udphole punch udphole.fly.dev:53000
    192.168.0.100:44266 # private
    208.60.21.109:14554 # public

## Docker

    # Build
    docker build -t udphole:local -f Dockerfile .

    # Listen
    docker run --rm \
        -p 53000:53000/udp --name udphole
        udphole:local listen 0.0.0.0:53000

    # Stop listening
    docker kill udphole

    # Punch
    docker run --rm \
        udphole:local punch udphole.fly.dev:53000
