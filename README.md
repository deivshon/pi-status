# pi-status

pi-status is a resource monitoring web application. It provides real-time information about the device's RAM, storage, CPU temperature and usage, processes' data, and network usage. It comes with a user friendly, mobile first front-end

It can be compiled for other architectures and will run on any Linux device with a modern kernel, but the temperature readings for the CPU are going to be wrong if they're not found in `/sys/class/thermal/thermal_zone0/temp`, independently of CPU architecture

## Usage

By default, pi-status will only be available to connections coming from [private networks](https://en.wikipedia.org/wiki/Private_network), hiding it from public ones

Use `-f` to make the monitored data available to anyone on the internet (this option is necessary when running pi-status in a Docker container)

## Installation and running

You can choose to compile and run pi-status natively or build a Docker image and run it in a container, though the latter option requires making the monitored resources necessarily publicly exposed, and only filterable through a firewall

### Requirements

- Rust toolchain
- NodeJS

### Compilation

To compile, simply run `make`. You can then run pi-status with the command `./back/target/release/pi-status` from the project's root folder

You probably don't want to leave a shell with pi-status constantly running, an alternative is running it as a systemd service, an example configuration file for that is shown below

```
[Unit]
Description=pi-status resource monitor
After=network.target

[Service]
Type=simple
User=<user>
WorkingDirectory=<your-pi-status-directory>
ExecStart=<your-pi-status-directory>/back/target/release/pi-status
Restart=on-failure

[Install]
WantedBy=multi-user.target
```

### Cross compilation

If you want to cross compile for your RPi 3/4 that is also possible with `make arm64`. This requires to have the aarch64 Rust and GNU toolchains installed

## Building the Docker image

### Requirements

- Docker, including compose and buildx plugins
- **If** cross compiling: `qemu-user-static` and `qemu-user-static-binfmt`

The quoted packages are found in Arch Linux repositories, you should find the corresponding ones for your distribution of choice

### Building

1. Clone the repository

The second step depends on your Raspberry Pi target

2. RPi 3/4: `make docker-arm64`

   RPi 2: `make docker-armv7`

If for some reason you want to try containerized pi-status on a amd64 machine, `make docker-amd64` is also available

After building the image and having it transferred to the target machine

3.  Remove intermediate images on your build machine with `docker image prune -f` and, if desired, remove build images manually (rust, node, alpine)
4.  Edit the `docker-compose.yaml` for arguments and additional volumes mounting (this is necessary for the containerized pi-status instance to be able to gather storage information about them)
5.  Run on target with `docker compose up`

## Endpoints

- `/` -> the web page to view the monitored resources data
- `/ws_data` -> WebSocket endpoint for monitored resources data in JSON format messages
