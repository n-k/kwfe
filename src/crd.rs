use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

/// DAG specification
#[derive(CustomResource, Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
#[kube(
    group = "n-k.github.io",
    version = "v1",
    kind = "DAG",
    plural = "dags",
    derive = "PartialEq",
    namespaced
)]
pub struct DAGSpec {
    steps: Vec<Step>,
    dependencies: Vec<Dependency>,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub struct Step {
    name: String,
    image: String,
    command: String,
    status: Status,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub enum Status {
    PENDING,
    STARTING,
    RUNNING,
    FINISHED,
    ERROR,
}

#[derive(Serialize, Deserialize, Debug, PartialEq, Clone, JsonSchema)]
pub struct Dependency {
    from: String,
    to: String,
}
