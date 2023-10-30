pub mod bash;
mod processing;
pub use processing::*;
pub const HELP: &str = "podboy
Pods with Borderless orchestration (yes)

CNT: short for CoNTainer
OPT: short for OPTions, podboy relies on services such as systemd and podman, command line options
for these services are functional however undocumented here.

PodboY requires an up-to-date version of systemd & podman, do not use for docker
____________________________________________________________________________________________________
SYSTEMD COMMANDS
____________________________________________________________________________________________________
start <CNT> <OPT>
Start the systemd process associated with the container

stop <CNT> <OPT>
Stop the systemd process associated with the container

enable <CNT> <OPT>
Start the systemd process associated with the container on boot

disable <CNT> <OPT>
Stop the systemd process associated with the container from starting on boot

status <CNT> <OPT>
Display the status and uptime of the systemd process associated with the container

restart <CNT> <OPT>
Restart the systemd process associated with the container
____________________________________________________________________________________________________
PODMAN COMMANDS
____________________________________________________________________________________________________
attach <CNT> <OPT>
Attach the terminal to the container EXITING PROCESS WILL STOP CONTAINER

logs <CNT> <OPT>
Display logs for the container

exec <CNT> <OPT>
Execute a command inside the container
____________________________________________________________________________________________________
PODBOY COMMANDS
____________________________________________________________________________________________________
version
Displays version number

gen <CNT>
Generate a systemd file for a given container

rm <CNT>
Remove a systemd file for a given container

regen <CNT>
Runs rm & gen together

ls
List systemd files for local user
";
