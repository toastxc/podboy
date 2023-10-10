use crate::bash::Bash;
use result::Result;
pub mod ha;
pub mod result;
pub fn string_add(input: impl Into<Vec<String>>) -> String {
    let mut buffer = String::new();
    input
        .into()
        .into_iter()
        .for_each(|item| buffer += &format!("{item} "));
    buffer
}
pub fn run_dual(cmd_type: bool, prefix: &str, mut args: Vec<String>) -> Result<Option<String>> {
    args.remove(0);
    Bash::cmd(&format!("{prefix} {}", string_add(args)), cmd_type)
}
