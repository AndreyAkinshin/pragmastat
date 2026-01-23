use anyhow::Result;
use serde::Serialize;
use std::collections::HashMap;

/// A parsed bibliography reference
#[derive(Debug, Clone, Serialize)]
pub struct Reference {
    pub key: String,
    pub authors: Vec<String>,
    pub title: String,
    pub year: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub venue: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub doi: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub url: Option<String>,
}

pub type References = HashMap<String, Reference>;

/// Parse a Hayagriva YAML file into a map of references
/// Also extracts DOIs manually from the raw YAML since hayagriva may not expose them directly
pub fn parse_hayagriva(content: &str) -> Result<References> {
    let library = hayagriva::io::from_yaml_str(content)
        .map_err(|e| anyhow::anyhow!("Failed to parse Hayagriva YAML: {e}"))?;

    // Also parse raw YAML to extract DOIs manually
    let raw_yaml: serde_yaml::Value = serde_yaml::from_str(content)?;

    let mut references = HashMap::new();

    for entry in library.iter() {
        let key = entry.key().to_string();

        // Extract authors
        let authors: Vec<String> = entry
            .authors()
            .map(|persons| {
                persons
                    .iter()
                    .map(|p| {
                        let name = &p.name;
                        let given = p.given_name.as_deref().unwrap_or("");
                        if given.is_empty() {
                            name.clone()
                        } else {
                            format!("{name}, {given}")
                        }
                    })
                    .collect()
            })
            .unwrap_or_default();

        // Extract title
        let title = entry
            .title()
            .map(|t| t.value.to_string())
            .unwrap_or_default();

        // Extract year from date
        let year = entry.date().map(|d| d.year.to_string()).unwrap_or_default();

        // Extract venue (journal or book title) from parents
        let venue = entry
            .parents()
            .first()
            .and_then(|parent| parent.title())
            .map(|t| t.value.to_string());

        // Extract DOI - try hayagriva first, then fall back to raw YAML parsing
        let doi = entry.doi().map(std::string::ToString::to_string).or_else(|| {
            raw_yaml
                .get(&key)
                .and_then(|v| v.get("doi"))
                .and_then(serde_yaml::Value::as_str)
                .map(String::from)
        });

        // Extract URL
        let url = entry.url().map(|u| u.value.to_string());

        let reference = Reference {
            key: key.clone(),
            authors,
            title,
            year,
            venue,
            doi,
            url,
        };

        references.insert(key, reference);
    }

    Ok(references)
}

/// Extract last name from author string
fn extract_last_name(author: &str) -> String {
    // Handle "Last, First" format
    if let Some(comma_pos) = author.find(',') {
        author[..comma_pos].trim().to_string()
    } else {
        // Handle "First Last" format
        author
            .split_whitespace()
            .last()
            .unwrap_or(author)
            .to_string()
    }
}

/// Format for short citation display: "Hodges & Lehmann" or "Hodges et al."
pub fn short_citation(reference: &Reference) -> String {
    let year = &reference.year;

    match reference.authors.len() {
        0 => format!("[{}]", reference.key),
        1 => {
            let name = extract_last_name(&reference.authors[0]);
            format!("{name} {year}")
        }
        2 => {
            let name1 = extract_last_name(&reference.authors[0]);
            let name2 = extract_last_name(&reference.authors[1]);
            format!("{name1} & {name2} {year}")
        }
        _ => {
            let name = extract_last_name(&reference.authors[0]);
            format!("{name} et al. {year}")
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_simple_entry() {
        // Test basic parsing without DOI since hayagriva DOI field
        // requires specific YAML format that may differ from Typst's
        let yaml = r"
hodges1963:
  type: article
  title: Estimates of Location Based on Rank Tests
  author:
    - name: Hodges
      given-name: J. L.
    - name: Lehmann
      given-name: E. L.
  date: 1963
  parent:
    - type: periodical
      title: The Annals of Mathematical Statistics
";
        let refs = parse_hayagriva(yaml).unwrap();
        let r = refs.get("hodges1963").unwrap();
        assert_eq!(r.authors.len(), 2);
        assert_eq!(r.year, "1963");
    }

    #[test]
    fn short_citation_two_authors() {
        let r = Reference {
            key: "test".into(),
            authors: vec!["Hodges, J. L.".into(), "Lehmann, E. L.".into()],
            title: "Test".into(),
            year: "1963".into(),
            venue: None,
            doi: None,
            url: None,
        };
        assert_eq!(short_citation(&r), "Hodges & Lehmann 1963");
    }

    #[test]
    fn short_citation_many_authors() {
        let r = Reference {
            key: "test".into(),
            authors: vec!["A, X".into(), "B, Y".into(), "C, Z".into()],
            title: "Test".into(),
            year: "2020".into(),
            venue: None,
            doi: None,
            url: None,
        };
        assert_eq!(short_citation(&r), "A et al. 2020");
    }

    #[test]
    fn extract_last_name_comma_format() {
        assert_eq!(extract_last_name("Hodges, J. L."), "Hodges");
    }

    #[test]
    fn extract_last_name_space_format() {
        assert_eq!(extract_last_name("John Smith"), "Smith");
    }
}
