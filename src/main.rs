use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Debug, Subcommand)]
enum Commands {
    /// 创建项目
    Create {
        /// 项目模板的 git 地址
        #[arg(long)]
        template: Option<String>,

        /// 项目名称
        #[arg(long)]
        name: Option<String>,
    },
}

fn main() {
    let cli = Cli::parse();
    match &cli.command {
        Some(Commands::Create { template, name }) => {
            handler::create(template.clone(), name.clone());
        }
        None => {}
    }
}
