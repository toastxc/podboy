<h1 align="center">
  PodboY

<h1 align="center">
  <img src="https://github.com/toastxc/podboy/blob/main/README_RESOURCES/podboy.svg" alt="Geometric Logo" width="30%" height="30%">

</h1>

Podboy (styled podboy or PodboY) is a simple daemon manager for Podman, similar to docker's daemon.

### Install (Cargo)
```bash
cargo install podboy
```
### Install (Linux)
```bash
git clone https://github.com/toastxc/podboy.git
cd podboy
cargo b -r
sudo cp ./target/release/podboy /usr/bin/
```

### Remove (cargo)
```bash
cargo uninstall podboy
```

### Remove (Linux)
```bash
sudo rm /usr/bin/podboy
```

### Compatibility
Software compatiblity is subject to https://toastxc.xyz/policies#windows

![LGPLv3 Badge](/README_RESOURCES/LGPLv3%20Logo.svg)
