use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Config {
    pub api_version: ApiVersion,

    pub kind: Kind,
    pub metadata: Metadata,

    pub spec: Spec,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub enum ApiVersion {
    #[default]
    V1,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
pub enum Kind {
    #[default]
    Pod,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Metadata {
    pub name: String,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Spec {
    pub containers: Vec<Container>,
    pub volumes: Vec<Volume>,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Container {
    pub name: String,
    pub image: String,

    pub env: Vec<Env>,

    pub volume_mounts: Vec<VolumeMount>,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Env {
    pub name: String,
    pub value: String,
}

impl Env {
    pub fn new<T0: Into<String>, T1: Into<String>>(name: T0, value: T1) -> Self {
        Self {
            name: name.into(),
            value: value.into(),
        }
    }
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VolumeMount {
    pub name: String,
    pub mount_path: String,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Volume {
    pub name: String,
    pub persistent_volume_claim: PersistentVolumeClaim,
}

#[derive(Serialize, Deserialize, Default, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct PersistentVolumeClaim {
    pub claim_name: String,
}
