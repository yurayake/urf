mod frun;
mod new;
mod rrun;

use clap::Parser;

#[derive(Parser)]
#[command(name = "urf")]
#[command(author = "yurayake <yuyayoshimura2003@gmail.com>")]
#[command(version = "1.0.0")]

struct Cli {
    #[arg(long, value_name = "project_name")]
    new: Option<String>,
    #[arg(long)]
    frun: bool,
    #[arg(long)]
    rrun: bool,
}

fn main() {
    let cli = Cli::parse();

    match cli.new {
        None => {
            if cli.frun == true {
                frun::frun();
            } else if cli.rrun == true {
                rrun::rrun();
            }
        }
        Some(project_name) => {
            new::new(project_name);
        }
    };
}
