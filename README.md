# sysmon-mqtt-rs

Tiny Rust program to send system temperature to an MQTT broker.

## Motivation

I wanted a way to monitor the temperature of my Pi Zero and my MacBook Air, both acting as AdGuard Home servers in different locations. I wanted to be able to do this remotely, preferably through Home Assistant, since I have another server that's running HA and is displaying its temperature as a sensor.

HA has an MQTT integration, so I decided to write a small program that would publish the temperature of the system to an MQTT broker at a set interval. I'm also not very familiar with Rust, so I thought this would be a good opportunity to learn more about it.

## Building

Just run `cargo build --release`, or if you have Make installed, you can run `make`.

### Cross-compiling

I wrote this on a Mac, so I'm using [cross-rs/cross](https://github.com/cross-rs/cross) to compile binaries for non-Mac targets (a Pi Zero and a Linux x86_64 machine).

To compile for the Pi Zero, run `make armhf` or `cross build --target arm-unknown-linux-gnueabihf --release`.

To compile for a Linux x86_64 machine, run `make linux` or `cross build --target x86_64-unknown-linux-gnu --release`. You don't have to do this if you're on Linux, of course - `make` will build the binary for your current platform.

## Usage

The program, when run, will start publishing the temperature of the system to the specified MQTT broker at a set interval. The temperature is read from a file, usually located in `/sys/class/thermal`.

Currently the program only supports publishing to a broker that requires authentication.

```sh
$ sysmon-mqtt-rs --help
Usage: sysmon-mqtt-rs [OPTIONS] --file <FILE> --address <ADDRESS> --username <USERNAME> --password <PASSWORD> --topic <TOPIC>

Options:
  -f, --file <FILE>          The file containing the temperature. Usually in /sys/class/thermal
  -a, --address <ADDRESS>    The address of the MQTT broker to connect to
      --port <PORT>          The port of the MQTT broker to connect to [default: 1883]
  -u, --username <USERNAME>  The username to use when connecting to the MQTT broker
  -p, --password <PASSWORD>  The password to use when connecting to the MQTT broker
  -t, --topic <TOPIC>        The topic to publish the temperature to
  -v, --verbose              Whether to log the temperature value being published
  -i, --interval <INTERVAL>  Number of seconds to wait in between publishing temperature values [default: 5]
  -r, --retain               Whether the broker should retain the last message sent
  -h, --help                 Print help
  -V, --version              Print version
```

## Running as a service

Below is an example systemd unit file for running the program as a service. This is the service file I use to run the program on my Pi Zero.

```ini
[Unit]
Description=MQTT temperature monitor
After=network.target

[Service]
ExecStart=/usr/local/etc/sysmon-mqtt-rs \
	-a IP_ADDRESS \
	-u USERNAME -p PASSWORD \
	-t TOPIC \
	-i 10 --retain \
	-f /sys/class/thermal/thermal_zone0/temp

[Install]
WantedBy=multi-user.target
```

Change the path in `ExecStart` to the location of the binary on your system. Change the other options to match your MQTT broker's settings and your system's temperature file.
