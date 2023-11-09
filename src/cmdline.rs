use clap::{Parser, Subcommand, ValueEnum};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
pub struct Cmdline {
    #[clap(value_enum, value_name = "POD_TYPE")]
    pub pods: Pods,

    #[command(subcommand)]
    pub command: Commands,
}

#[derive(ValueEnum, Clone, Debug)]
pub enum Pods {
    Ghost,
}

#[derive(Subcommand, Debug)]
pub enum Commands {
    #[command(about = "Quickly spawn a pod for testing/development purposes")]
    Dev {},
    #[command(
        about = "Create a pod for production",
        long_about = r#"
Spawn a production-ready pod.

You can then visit the pod from http://<domain>:<port>.
However, using a reverse proxy is recommended."#
    )]
    Add {
        #[arg(short, long, help = "Domain of the pod, e.g. example.org")]
        domain: String,
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
