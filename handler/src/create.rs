use std::{env, fs};

use inquire::{validator::Validation, Confirm, Text};
use regex::Regex;

const DEFAULT_TEMPLATE: &str = "git@github.com:phostann/config-center.git";

pub fn create(template: Option<String>, name: Option<String>) {
    let template = match template {
        Some(t) => t,
        None => DEFAULT_TEMPLATE.to_string(),
    };

    let re = Regex::new(r"^[a-zA-Z0-9-]+$").unwrap();

    let name_validator = move |input: &str| {
        if re.is_match(input) {
            Ok(Validation::Valid)
        } else {
            Ok(Validation::Invalid("项目名称只能包含字母、数字、-".into()))
        }
    };

    let name = match name {
        Some(n) => n,
        None => Text::new("请输入项目名称:")
            .with_validator(name_validator)
            .prompt()
            .unwrap(),
    };

    let cwd = env::current_dir().unwrap();
    let dir = cwd.join(&name);
    if dir.is_dir() && dir.read_dir().unwrap().count() > 0 {
        let ans = Confirm::new("目录已存在，是否继续?")
            .with_default(false)
            .prompt();
        if let Ok(true) = ans {
            fs::remove_dir_all(&dir).unwrap();
        } else {
            println!("创建失败");
            return;
        }
    }

    exec::git::clone(&template, &name);
}
