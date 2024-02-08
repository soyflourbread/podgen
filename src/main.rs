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
        Commands::Add {
            pods,
            name,
            dev,
            domain,
            port,
        } => {
            let mut builder = pods::PodBuilder::default();
            if !dev {
                builder.set_production();
            }
            if let Some(name) = name {
                builder.set_name(name);
            }
            if let Some(domain) = domain {
                builder.set_domain(domain);
            }

            use crate::cmdline::Pods;
            match pods {
                Pods::Ghost => {
                    let k8s_config = builder.build::<pods::ghost::Generator>();
                    println!("{}", serde_yaml::to_string(&k8s_config).unwrap());
                }
            }
        }
        Commands::Del { name, delete_data } => {}
        _ => {}
    }
}
