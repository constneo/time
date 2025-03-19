use clap::Parser;
use colored::Colorize;
use std::process::Command;
use std::time::Instant;

/// 主程序
#[derive(Parser, Debug)]
#[command(name = "time", version)]
#[command(about = "Simple count CLI run time.", long_about = None)]
struct Args {
    /// 必填的参数
    input: Vec<String>,
}

fn main() {
    let args = Args::parse();

    if args.input.len() == 0 {
        panic!("{}", "Error:Do you input Command?".to_string().red());
    }

    println!("------------- Begin -------------");

    for item in &args.input {
        println!("{item}")
    }

    println!("------------- End -------------");

    let input: Vec<_> = args.input.iter().skip(1).collect();
    let start = Instant::now();
    let mut binding = Command::new(&args.input[0]);
    let command = binding.args(input).spawn();

    if let Ok(mut child) = command {
        child.wait().expect("Command wasn't running");
        let duration = start.elapsed();
        println!(
            "Command running time is: {}s {}ms {}us {}ns",
            duration.as_secs().to_string().green(),
            duration.subsec_millis().to_string().green(),
            (duration.subsec_micros() % 1_000).to_string().green(),
            (duration.subsec_nanos() % 1_000).to_string().green()
        );
    } else {
        println!("Command didn't start");
    }
}
