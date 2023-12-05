use serde::{Serialize, Deserialize, de};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pet {
    #[serde(default = "default_id")]
    pub id: String,
	pub microchip: u64,
    #[serde(deserialize_with = "match_unidentified")]
	pub species: String,
	pub sex: String,
	pub size: String,
    #[serde(deserialize_with = "bind_bool")]
	pub potent_dangerous: bool,
    #[serde(deserialize_with = "match_unidentified")]
    pub neighborhood: String
}

pub enum Species {
	Canino,
	Felino
}

fn default_id() -> String {
    "NO ASIGNADO".to_string()
}

fn bind_bool<'de, D>(deserializer: D) -> Result<bool, D::Error> where D: de::Deserializer<'de> {
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    match s {
        "SI" => Ok(true),
        "NO" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["SI", "NO"])),
    }
}

fn match_unidentified<'de, D>(deserializer: D) -> Result<String, D::Error> where D: de::Deserializer<'de> {
    let s = String::deserialize(deserializer).unwrap();
    match s.as_str() {
        "NO IDENTIFICADO" => Err(de::Error::unknown_variant(s.as_str(), &["CANINO", "FELINO"])),
        "SIN IDENTIFICAR"=> Err(de::Error::unknown_variant(s.as_str(), &["Real Neighborhoods"])),
        "" => Err(de::Error::unknown_variant(s.as_str(), &["Some Neighborhood"])),
        _ => Ok(s)
    }
}