use std::collections::HashMap;

use bollard::service::ContainerSummary;

pub struct Container {
    pub name: String,
    labels: HashMap<String, String>,
}

impl Container {
    pub fn from_summary(container: &ContainerSummary) -> Option<Self> {
        Some(Container {
            // Remove leading '/'
            name: String::from(&container.names.as_ref()?[0][1..]),
            labels: container
                .labels
                .as_ref()
                .map_or(HashMap::new(), |labels| labels.clone()),
        })
    }
}
