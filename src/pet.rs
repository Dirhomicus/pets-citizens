use serde::{Serialize, Deserialize, de};

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(rename_all = "camelCase")]
pub struct Pet {
   #[serde(default = "default_id")]
   pub id: String,
   pub microchip: u64,
   #[serde(deserialize_with = "match_species")]
   pub species: Species,
   #[serde(deserialize_with = "match_sex")]
   pub sex: Sex,
   #[serde(deserialize_with = "match_size")]
   pub size: Size,
   #[serde(deserialize_with = "match_potent")]
   pub potent_dangerous: bool,
   #[serde(deserialize_with = "match_unidentified")]
   pub neighborhood: String
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Species {
	Canino,
	Felino
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Sex {
    Macho,
    Hembra
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(rename_all = "UPPERCASE")]
pub enum Size {
    Miniatura,
    Pequeño,
    Mediano,
    Grande,
    MuyGrande
}

fn default_id() -> String {
    "NO ASIGNADO".to_string()
}

fn match_potent<'de, D>(deserializer: D) -> Result<bool, D::Error> where D: de::Deserializer<'de> {
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    match s {
        "SI" => Ok(true),
        "NO" => Ok(false),
        _ => Err(de::Error::unknown_variant(s, &["SI", "NO"])),
    }
}

fn match_species<'de, D>(deserializer: D) -> Result<Species, D::Error> where D: de::Deserializer<'de> {
   let s: &str = de::Deserialize::deserialize(deserializer)?;
   match s {
       "CANINO" => Ok(Species::Canino),
       "FELINO" => Ok(Species::Felino),
       _ => Err(de::Error::unknown_variant(s, &["Canino", "Felino"])),
   }
}

fn match_sex<'de, D>(deserializer: D) -> Result<Sex, D::Error> where D: de::Deserializer<'de> {
   let s: &str = de::Deserialize::deserialize(deserializer)?;
   match s {
       "MACHO" => Ok(Sex::Macho),
       "HEMBRA" => Ok(Sex::Hembra),
       _ => Err(de::Error::unknown_variant(s, &["Macho", "Hembra"])),
   }
}

fn match_size<'de, D>(deserializer: D) -> Result<Size, D::Error> where D: de::Deserializer<'de> {
   let s: &str = de::Deserialize::deserialize(deserializer)?;
   match s{
       "MINIATURA" => Ok(Size::Miniatura),
       "PEQUEÑO" => Ok(Size::Pequeño),
       "MEDIANO" => Ok(Size::Mediano),
       "GRANDE" => Ok(Size::Grande),
       "MUY GRANDE" => Ok(Size::MuyGrande),
       _ => Err(de::Error::unknown_variant(s, &["Miniatura", "Pequeño", "Mediano", "Grande", "MuyGrande"])),
   }
}

fn match_unidentified<'de, D>(deserializer: D) -> Result<String, D::Error> where D: de::Deserializer<'de> {
    let s: &str = de::Deserialize::deserialize(deserializer)?;
    match s {
        "SIN IDENTIFICAR" => Err(de::Error::unknown_variant(s, &["Real Neighborhoods"])),
        "" => Err(de::Error::unknown_variant(s, &["Some Neighborhood"])),
        _ => Ok(s.to_string())
    }
}