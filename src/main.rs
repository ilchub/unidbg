use sysinfo::{ProcessExt, SystemExt};
use textplots::{Chart, Plot, Shape};
use structopt::StructOpt;
use std::process::Command;

#[derive(StructOpt, Debug)]
#[structopt(name = "Unity Debugger", about = "Tool for debugging and profiling Unity desktop environment \nMade by Illia `drag0nhaze` Chub <ilchub5@protonmail.com>")]
struct Cli {
    #[structopt(short = "s", long = "single", help = "Setting program to single data fetch(true/false)")]
    single: String,
}
fn main() {
    let args = Cli::from_args();
    if args.single == "true".to_string() {
        debug();
    }
    else {
        Command::new("watch").arg("./unidbg").arg("--single true").spawn().expect("Failed to execute command");
    }

}
fn debug() {
    let mut system = sysinfo::System::new_all();
    let mut memusage: Vec<(f32,f32)> = Vec::with_capacity(10);
    let mut graph_index: f32 = 0.0;

// Getting system info
    system.refresh_all();

// Getting processes data
    println!("ID\t NAME\t PATH\t MEMORY");
    for (pid, proc) in system.get_processes() {
        if proc.name().contains("unity") {
            println!("{}\t {}\t {}\t {}", pid, proc.name(),proc.exe().display(), proc.memory());
            memusage.push((graph_index, proc.memory() as f32));
            graph_index+=1 as f32;
        }
    }
    for _i in 0..3 {
        println!("");
    }
    println!("Memory usage graph");
    Chart::new(200,100,0.0,memusage.len() as f32).lineplot(Shape::Lines(&memusage)).display();
}