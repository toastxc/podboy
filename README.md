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
This program was designed, tested, compiled and made for GNU/Linux systems and as such there is likely no support for non POSIX systems.
The program relies on `bash`, `systemd` and `podman` so if any of these are missing.

While it may be possible to add support for NT based operating systems it is not a focus for us, and will not receive support.
However if there is a feature complete fork of PodboY for Windows made we will link to it in the description of this project.

![LGPLv3 Badge](/README_RESOURCES/LGPLv3%20Logo.svg)
