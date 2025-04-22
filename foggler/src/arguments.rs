use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Foggler keeps a constant watch on Dead by Daylight server connections")]
pub struct Arguments {
    #[arg(short, long, required = true, help = "List of servers to ping")]
    pub servers: String,

    #[arg(short, long, default_value = "3", help = "Time limit to wait before timeout")]
    pub timeout: u64,
   
    #[arg(short, long = "port", default_value = "443", help = "Port to ping")]
    pub port: u16,

    #[arg(long, default_value = "false", help = "Display debug information")]
    pub debug: bool
}
