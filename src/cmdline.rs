use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cmdline {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(
        about = "Create a pod for production",
        long_about = r#"
Spawn a pod.

You can then visit the pod from http://<domain>:<port>.
However, using a reverse proxy is recommended."#
    )]
    Add {
        #[command(subcommand)]
        pods: Pods,

        #[arg(short, long, help = "Name of the pod, e.g. wordpress")]
        name: Option<String>,
        #[arg(long, help = "Quickly spawn a development pod")]
        dev: bool,

        #[arg(short, long, help = "Domain of the pod, e.g. example.org")]
        domain: Option<String>,
        #[arg(short, long, help = "Host port, e.g. 8001")]
        port: u16,
    },
    #[command(about = "Delete a pod")]
    Del {
        #[arg(help = "Name of the pod to delete")]
        name: String,

        #[arg(long, help = "Remove volumes associated with the pod")]
        delete_data: bool,
    },
}

#[derive(Subcommand, Debug)]
pub enum Pods {
    Ghost,
}
