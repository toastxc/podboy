use crate::bash::Bash;
use result::Result;
pub mod ha;
pub mod result;
pub fn string_add(input: impl Into<Vec<String>>) -> String {
    input
        .into()
        .into_iter()
        .map(|item| format!("{item} "))
        .collect()
}
pub fn run_dual(cmd_type: bool, prefix: &str, args: Vec<String>) -> Result<Option<String>> {
    if prefix.contains("systemctl") {
        Bash::cmd("systemctl --user daemon-reload", false)?;
    };
    Bash::cmd(&format!("{prefix} {}", string_add(args)), cmd_type)
}
