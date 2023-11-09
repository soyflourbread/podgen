use crate::cmdline::Commands;

mod cmdline;

mod pods;
mod systemctl;

fn main() {
    let cli = {
        use clap::Parser;
        cmdline::Cmdline::parse()
    };

    match cli.command {
        Commands::Dev {} => {
            let mut builder = pods::PodBuilder::default();

            use crate::cmdline::Pods;
            match cli.pods {
                Pods::Ghost => {
                    let k8s_config = builder.build::<pods::ghost::Generator>();
                    println!("{}", serde_yaml::to_string(&k8s_config).unwrap());
                }
            }
        }
        _ => {}
    }
}
