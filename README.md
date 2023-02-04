# pi-status
pi-status is a resource monitoring web application. It provides real-time information about the device's RAM, storage, CPU temperature and usage, processes' data, and network usage. It comes with a user friendly, mobile first front-end

The Dockerfile is set build an image for AArch64 architecture, which makes it compatible with Raspberry Pi Model 2/3/4

It can be compiled for other architectures and will run on any Linux device with a modern kernel, but the temperature readings for the CPU are going to be wrong if they're not found in `/sys/class/thermal/thermal_zone0/temp`, independently of CPU architecture

## Usage
```
$ ./pi-status [-p {PORT}] [-f]
```

By default, pi-status will listen on port 8080 and only be available to connections coming from [private networks](https://en.wikipedia.org/wiki/Private_network), hiding it from public ones

The accepted arguments are:

- `-p` -> Specify the port the service will run on
- `-f` -> Make the monitored data available to anyone on the internet (this option is necessary when running pi-status in a Docker container)

## Installation and running
You can choose to compile and run pi-status natively or build a Docker image and run it in a container, though the latter option requires making the monitored resources necessarily publicly exposed, and only filterable through a firewall

### Compile and run natively
- Clone the repository
- Install the appropriate Rust toolchain
- Run `cargo build --release` inside the `./back` directory to build the backend portion of pi-status
- Install `npm`
- Run `npm i` inside `./front/pi-status-front` to install the necessary tools and libraries to build the frontend
- Run `npm run build` to build the frontend
- Run `./back/target/release/pi-status`, from the project's root folder

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
### Docker
- Clone the repository
- Run `docker build --rm -t pi-status .`
- Remove intermediate images with `docker image prune -f` and, if desired, remove build images manually (rust, node, alpine)
- Edit the `docker-compose.yaml` for arguments and additional volumes mounting (this is necessary for the containerized pi-status instance to be able to gather storage information about them)
- Run with `docker compose up`

Note: the Dockerfile is set to build and run a binary with target architecture AArch64, if another one is desired, this behaviour can be changed by modifying the Dockerfile to add a different Rust toolchain, `cargo build` for a different target architecture, and then copying from the new cargo target architecture folder instead of `..target/aarch64-unknown-linux-gnu/release...` in the last stage of the build

## Endpoints
- `/` -> the web page to view the monitored resources data
- `/data` -> the monitored resources data in JSON format
