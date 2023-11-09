use super::{k8s, PodGenerator};

pub struct Generator {}

impl PodGenerator for Generator {
    fn published_port() -> u16 {
        2368 // see https://hub.docker.com/_/ghost
    }

    fn generate(name: String, domain: Option<String>, production: bool) -> k8s::Config {
        const DATABASE_PASSWD: &str = "mysqlpass";
        const DATABASE_NAME: &str = "ghost";

        let mut main_env = vec![
            k8s::Env {
                name: "database__client".to_string(),
                value: "mysql".to_string(),
            },
            k8s::Env {
                name: "database__connection__host".to_string(),
                value: "localhost".to_string(),
            },
            k8s::Env {
                name: "database__connection__database".to_string(),
                value: DATABASE_NAME.to_string(),
            },
            k8s::Env {
                name: "database__connection__user".to_string(),
                value: "root".to_string(),
            },
            k8s::Env {
                name: "database__connection__password".to_string(),
                value: DATABASE_PASSWD.to_string(),
            },
        ];
        if let Some(domain) = domain {
            main_env.push(k8s::Env {
                name: "url".to_string(),
                value: format!("https://{domain}"),
            });
        }
        if !production {
            main_env.push(k8s::Env {
                name: "NODE_ENV".to_string(),
                value: "development".to_string(),
            })
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
