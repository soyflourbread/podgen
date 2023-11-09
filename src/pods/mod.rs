pub mod ghost;
mod k8s;

pub trait PodGenerator {
    fn published_port() -> u16;

    fn generate(name: String, domain: Option<String>, production: bool) -> k8s::Config;
}

#[derive(Default)]
pub struct PodBuilder {
    name: Option<String>,
    domain: Option<String>,

    production: bool,
}

impl PodBuilder {
    pub fn set_name(&mut self, name: String) -> &mut Self {
        self.name = Some(name);
        self
    }

    pub fn set_domain(&mut self, domain: String) -> &mut Self {
        self.domain = Some(domain);
        self
    }

    pub fn set_production(&mut self) -> &mut Self {
        self.production = true;
        self
    }

    pub fn build<G: PodGenerator>(self) -> k8s::Config {
        let name = self.name.unwrap_or(
            self.domain
                .clone()
                .map(|s| {
                    let mut vec = s.split('.').collect::<Vec<_>>();
                    vec.reverse();
                    vec.join("-")
                })
                .unwrap_or_else(|| names::Generator::default().next().unwrap()),
        );
        let domain = self.domain;
        let production = self.production;

        G::generate(name, domain, production)
    }
}
