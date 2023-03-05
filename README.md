<h1 align="center">
  PodboY

<h1 align="center">
  <img src="https://github.com/toastxc/podboy/blob/main/README_RESOURCES/podboy.svg" alt="Crab claw holding a spanner" width="30%" height="30%">

</h1>

Podboy (styled podboy or PodboY) is a simple daemon manager for Podman, similar to docker's daemon.


### Testing
```bash
git clone https://github.com/toastxc/podboy.git
cd podboy
cargo b
```

### Installation 
```bash
git clone https://github.com/toastxc/podboy.git
cd podboy
cargo b -r
sudo cp ./target/release/podboy /usr/bin/
```

### Removal
```bash
sudo rm /usr/bin/podboy
```

### Compatibility
This program was designed, tested, compiled and made for GNU/Linux systems and as such there is likely no support for non POSIX systems.
The program relies on `bash`, `systemd` and `podman` so if any of these are missing.

While it may be possible to add support for NT based operating systems it is not a focus for us, and will not receive support.
However if there is a feature complete fork of PodboY for Windows made we will link to it in the description of this project.

![LGPLv3 Badge](/README_RESOURCES/LGPLv3%20Logo.svg)
