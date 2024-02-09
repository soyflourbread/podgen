#[derive(Default)]
pub struct PodInfoBuilder {
    name: Option<String>,
    domain: Option<String>,

    production: bool,
}

impl PodInfoBuilder {
    pub fn set_name(&mut self, name: String) {
        self.name = Some(name);
    }

    pub fn set_domain(&mut self, domain: String) {
        self.domain = Some(domain);
    }

    pub fn set_production(&mut self) {
        self.production = true;
    }

    pub fn build(self) -> PodInfo {
        let name = self.name.clone().unwrap_or(
            self.domain
                .clone()
                .map(|s| {
                    let mut vec = s.split('.').collect::<Vec<_>>();
                    vec.reverse();
                    vec.join("-")
                })
                .unwrap_or_else(|| names::Generator::default().next().unwrap()),
        );

        PodInfo {
            name,
            production: self.production,
            domain: self.domain,
        }
    }
}

pub struct PodInfo {
    pub name: String,
    pub production: bool,

    pub domain: Option<String>,
}
