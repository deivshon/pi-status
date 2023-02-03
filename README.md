# pi-status
Pi-status is a resource monitoring web application. It provides real-time information about the device's RAM, storage, CPU temperature and usage, processes' data, and network usage. It comes with a user friendly, mobile first front-end

The releases and relative Docker containers are all for AArch64 devices, which makes it compatible with Raspberry Pi Models 2/3/4

It can be compiled for other architectures and will run on any Linux device with a modern kernel, but the temperature readings for the CPU are going to be wrong if they're not found in `/sys/class/thermal/thermal_zone0/temp`, independently of CPU architecture

## Usage
```
$ ./pi-status [-p {PORT}] [-f]
```

By default, pi-status will run on port 8080 and only be available to connections coming from [private networks](https://en.wikipedia.org/wiki/Private_network), hiding it from public ones

The accepted arguments are:

- `-p` -> Specify the port the service will run on
- `-f` -> Make the monitored data available to anyone on the internet (this option is necessary when running pi-status in a Docker container)

## Installation and running
It's recommended to not run pi-status in a container if possible, but a Docker image is available, albeit requiring more configuration and making the monitored resources necessarily publicly exposed, and only filterable through a firewall

### Native
- Download and extract the release of choice
- Run `./pi-status`, from the project's root folder
### Docker
- Pull the image with `docker pull deivshon/pi-status`
- Create a directory for pi-status and `cd` into it
- Download and save into the new directory the `.env` and `docker-compose.yaml` files
- Edit the `docker-compose.yaml` for arguments and additional volumes mounting (this is necessary for the containerized pi-status instance to be able to gather storage information about them)
- Run with `docker compose up`

## Endpoints
- `/`
- `/data`
