use serde::Deserialize;

#[derive(Debug, Deserialize, Clone)]
pub struct Radio {
    pub name: String,
    pub description: String,
    pub url: String,
}

impl Radio {
    pub fn parse(radios: &str) -> Result<Vec<Radio>, serde_yaml::Error> {
        let radios: Vec<Radio> = serde_yaml::from_str(&radios)?;
        Ok(radios)
    }
}

impl From<IndexedRadio> for Radio {
    fn from(ir: IndexedRadio) -> Self {
        Radio {
            name: ir.name,
            description: ir.description,
            url: ir.url,
        }
    }
}

#[derive(Clone)]
pub struct IndexedRadio {
    pub name: String,
    pub description: String,
    pub url: String,
    pub a_name: String,
    pub a_description: String,
    pub a_url: String,
}

impl IndexedRadio {
    pub fn retain(mut text: String) -> String {
        text.make_ascii_lowercase();
        text.retain(|c| {
            c.is_alphanumeric()
        });

        text
    }

    pub fn is_part(&self, search: &str) -> bool {
        self.a_name.contains(search) || self.a_description.contains(search) || self.a_url.contains(search)
    }
}

impl From<Radio> for IndexedRadio {
    fn from(radio: Radio) -> Self {
        IndexedRadio {
            name: radio.name.clone(),
            description: radio.description.clone(),
            url: radio.url.clone(),
            a_name: Self::retain(radio.name),
            a_description: Self::retain(radio.description),
            a_url: Self::retain(radio.url),
        }
    }
}