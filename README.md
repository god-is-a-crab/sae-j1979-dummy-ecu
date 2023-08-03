# SAE J1979 Dummy ECU
Respond to OBD-II parameter queries.

[OBD-II CAN format (wikipedia)](https://en.wikipedia.org/wiki/OBD-II_PIDs#CAN_(11-bit)_bus_format)

## Build for aarch64
Install cross (Rust tool)
```
cargo install cross
```
Install docker or podman
```
sudo snap install docker
sudo apt-get install -y podman
```
Build for aarch64-unknown-linux-gnu. You may need to resolve some [permission issues with docker](https://stackoverflow.com/questions/48957195/how-to-fix-docker-got-permission-denied-issue).
```
cross build --target aarch64-unknown-linux-gnu --release
```

## Usage
```
./sae-j1979-dummy-ecu [interface (e.g. can0)]
```
