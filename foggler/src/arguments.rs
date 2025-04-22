use clap::Parser;

#[derive(Parser)]
#[command(version, about = "Foggler keeps a constant watch on Dead by Daylight server connections")]
pub struct Arguments {
    #[arg(short, long, required = true, help = "List of servers to ping")]
    pub servers: String,

    #[arg(short, long, default_value = "3", help = "Time limit to wait before timeout")]
    pub timeout: u64,
   
    #[arg(short, long, default_value = "443", help = "Port to ping")]
    pub port: u16,
    
    #[arg(short, long, default_value = "600", help = "Waiting time in sec between check (0 will stop after first check)")]
    pub wait: u64,

    #[arg(short, long, default_value = "logs", help = "Logs folder used for monitoring")]
    pub logs: String,

    #[arg(long, default_value = "false", help = "Display debug information")]
    pub debug: bool
}
