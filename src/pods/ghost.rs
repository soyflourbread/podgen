use super::PodGenerator;

pub struct Generator {}

// see https://hub.docker.com/_/ghost
impl PodGenerator for Generator {
    fn published_port() -> u16 {
        2368
    }

    fn generate(name: String, domain: Option<String>, production: bool) -> k8s::Config {
        const DATABASE_PASSWD: &str = "mysqlpass";
        const DATABASE_NAME: &str = "ghost";

        let mut main_env = vec![
            k8s::Env::new("database__client", "mysql"),
            k8s::Env::new("database__connection__host", "localhost"),
            k8s::Env::new("database__connection__database", DATABASE_NAME),
            k8s::Env::new("database__connection__user", "root"),
            k8s::Env::new("database__connection__password", DATABASE_PASSWD),
        ];
        if let Some(domain) = domain {
            main_env.push(k8s::Env::new("url", format!("https://{domain}")));
        }
        if !production {
            main_env.push(k8s::Env::new("NODE_ENV", "development"));
        }

        let vol_main = format!("{name}-main");
        let vol_main_pvc = format!("{name}-main-pvc");

        k8s::Config {
            kind: k8s::Kind::Pod,
            metadata: k8s::Metadata { name: name.clone() },

            spec: k8s::Spec {
                containers: vec![k8s::Container {
                    name: format!("{name}-main"),
                    image: "docker.io/library/ghost:5-alpine".to_string(),
                    env: main_env,
                    volume_mounts: vec![k8s::VolumeMount {
                        name: vol_main_pvc.clone(),
                        mount_path: "/var/lib/ghost/content".to_string(),
                    }],
                }],
                volumes: vec![k8s::Volume {
                    name: vol_main_pvc.clone(),
                    persistent_volume_claim: k8s::PersistentVolumeClaim {
                        claim_name: vol_main.clone(),
                    },
                }],
            },

            ..k8s::Config::default()
        }
    }
}
