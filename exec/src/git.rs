use crate::utils::exec_command;

pub fn clone(template: &str, name: &str) {
    exec_command("git", &vec!["clone", template, name]);
}
