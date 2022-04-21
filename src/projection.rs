use crate::errors::SHError;
use std::{collections::HashMap, path::PathBuf, str::FromStr};

use serde::Deserialize;
#[allow(dead_code)]
#[derive(Debug, Deserialize, PartialEq)]
#[serde(untagged)]
enum Alternate {
    One(String),
    Many(Vec<String>),
}

#[derive(Debug, Deserialize)]
pub struct Projection {
    #[serde(rename = "type")]
    #[serde(default)]
    type_name: Option<String>,
    #[serde(default)]
    alternate: Option<Alternate>,
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

pub struct ProjectionCollection {
    collection: HashMap<String, Projection>,
}

impl ProjectionCollection {
    pub fn new(s: &str) -> Result<Self, SHError> {
        let mut collection: HashMap<String, Projection> = serde_json::from_str::<_>(s)?;

        for (key, value) in &mut collection {
            value.path = Some(key.clone());
        }

        Ok(Self { collection })
    }

    fn get(&self, key: &str) -> Option<&Projection> {
        self.collection.get(key)
    }
}

impl Projection {
    fn get_alternates(&self, substitution: &str) -> Result<Vec<PathBuf>, SHError> {
        fn convert_to_path(pattern: &str, substitution: &str) -> Result<PathBuf, SHError> {
            let computed_path = pattern.replace("{}", substitution);
            PathBuf::from_str(&computed_path).map_err(|_| SHError::PathNotFound(computed_path))
        }

        match &self.alternate {
            Some(Alternate::One(pat)) => Ok(vec![convert_to_path(pat, substitution)?]),
            Some(Alternate::Many(ref pats)) => {
                return pats
                    .iter()
                    .map(|pat| convert_to_path(pat, substitution))
                    .collect()
            }
            None => Err(SHError::NoAlternate(
                self.path.clone().unwrap_or_else(|| "unknown path".into()),
            )),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_can_deserialize_projections() {
        let input = r##"
        {
            "README.md": { "type": "readme" },
            "lib/*.rb": {
                "type": "lib",
                "alternate": "spec/{}_spec.rb",
                "template": ["#!/usr/bin/env bash"]
            },
            "config/*.yml": {
                "alternate": [
                    "config/{}.example.yml",
                    "config/{}.yml.example",
                    "config/{}.yml.sample"
                ]
            }
        }"##;
        let projections = ProjectionCollection::new(input).unwrap();
        assert_eq!(
            projections
                .get("README.md")
                .unwrap()
                .type_name
                .as_ref()
                .unwrap(),
            "readme"
        );
        assert!(projections.get("README.md").unwrap().alternate.is_none(),);
        assert_eq!(
            projections.get("lib/*.rb").unwrap().alternate,
            Some(Alternate::One("spec/{}_spec.rb".into()))
        );
        assert_eq!(
            projections.get("config/*.yml").unwrap().alternate,
            Some(Alternate::Many(
                [
                    "config/{}.example.yml",
                    "config/{}.yml.example",
                    "config/{}.yml.sample"
                ]
                .iter()
                .map(|&a| a.into())
                .collect()
            ))
        );
    }

    #[test]
    fn it_can_get_alternates() {
        let input = r##"
        {
            "config/*.yml": {
                "alternate": [
                    "config/{}.example.yml",
                    "config/{}.yml.example",
                    "config/{}.yml.sample"
                ]
            }
        }"##;
        let projections = ProjectionCollection::new(input).unwrap();

        assert_eq!(
            projections
                .get("config/*.yml")
                .unwrap()
                .get_alternates("test"),
            Ok(vec![
                "config/test.example.yml",
                "config/test.yml.example",
                "config/test.yml.sample"
            ]
            .into_iter()
            .map(PathBuf::from)
            .collect())
        );
    }
}
