use crate::cmdline::Commands;
use crate::pods::PodGenerator;

mod cmdline;

mod info;
mod pods;

fn main() {
    let cli = {
        use clap::Parser;
        cmdline::Cmdline::parse()
    };

    match cli.command {
        Commands::Add {
            pods,
            name,
            dev,
            domain,
            port,
        } => {
            let mut builder = info::PodInfoBuilder::default();
            if !dev {
                builder.set_production();
            }
            if let Some(name) = name {
                builder.set_name(name);
            }
            if let Some(domain) = domain {
                builder.set_domain(domain);
            }
            let info = builder.build();

            use crate::cmdline::Pods;
            match pods {
                Pods::Ghost => {
                    let k8s_config =
                        pods::ghost::Generator::generate(info.name, info.domain, info.production);
                    println!("{}", serde_yaml::to_string(&k8s_config).unwrap());
                }
            }
        }
        Commands::Del { name, delete_data } => {}
        _ => {}
    }
}
