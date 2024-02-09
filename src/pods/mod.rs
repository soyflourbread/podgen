pub mod ghost;

pub trait PodGenerator {
    fn published_port() -> u16;

    fn generate(name: String, domain: Option<String>, production: bool) -> k8s::Config;
}
