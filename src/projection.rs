use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    #[serde(rename = "type")]
    #[serde(default)]
    type_name: Option<String>,
    #[serde(default)]
    alternate: Option<String>,
    #[serde(default)]
    template: Option<Vec<String>>,
    #[serde(default)]
    dispatch: Option<String>,
    #[serde(default)]
    make: Option<String>,
    #[serde(default)]
    path: Option<String>,
    #[serde(default)]
    related: Option<String>,
    #[serde(default)]
    start: Option<String>,
}

pub type Projections = std::collections::HashMap<String, Config>;

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_serialize_projections() {
        let input = r##"
        {
            "README.md": { "type": "readme" },
            "lib/*.rb": {
                "type": "lib",
                "alternate": "spec/{}_spec.rb",
                "template": ["#!/usr/bin/env bash"]
            }
        }"##;
        let projections: Projections = serde_json::from_str::<Projections>(input).unwrap();
        assert_eq!(
            projections
                .get("README.md")
                .unwrap()
                .type_name
                .as_ref()
                .unwrap(),
            "readme"
        );
        assert_eq!(
            projections.get("README.md").unwrap().alternate.is_none(),
            true
        );
        assert_eq!(
            projections
                .get("lib/*.rb")
                .unwrap()
                .alternate
                .as_ref()
                .unwrap(),
            "spec/{}_spec.rb"
        );
    }
}
