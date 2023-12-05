use std::fs::File;
use csv::ReaderBuilder;
use crate::pet::{Pet, Species};

pub fn read_csv() -> Vec<Pet> {
	let file = File::open("pets-citizens.csv").expect("no pets-citizens.csv file found!");
	let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(file);
    reader.deserialize::<Pet>()
        .filter_map(Result::ok)
        .map(assign_id)
        .collect()
}

fn assign_id(pet: Pet) -> Pet {
    let number: String = pet.microchip.to_string().chars().rev().take(7).collect();
    let species_letter = match pet.species.chars().take(1).collect::<String>().as_str() {
        "C" => "C",
        "F" => "F",
        _ => ""
    };
    let sex_letter = match pet.sex.chars().take(1).collect::<String>().as_str() {
        "H" => "H",
        "M" => "M",
        _ => ""
    };
    let size_letters = match pet.size.chars().take(2).collect::<String>().as_str() {
        "MI" => "MI",
        "PE" => "P",
        "ME" => "M",
        "GR" => "G",
		"MU" => "MG",
        _ => ""
    };
    let potent_letter = match pet.potent_dangerous {
        true => "T",
        false => "F"
    };
    let id = format!("{number}-{species_letter}{sex_letter}{size_letters}{potent_letter}");
    Pet { id, microchip: pet.microchip, species: pet.species, sex: pet.sex, size: pet.size, potent_dangerous: pet.potent_dangerous, neighborhood: pet.neighborhood }
}

pub fn find_by_microchip(pets: &Vec<Pet>, microchip: u64) -> Option<&Pet> {
	pets.into_iter().find(|pet| pet.microchip == microchip)
}

pub fn count_species(pets: &Vec<Pet>, species: Option<Species>) {
	match species {
		Some(Species::Canino) => {
			let canines = pets.iter().filter(|s| s.species.as_str() != "FELINO").count();
			println!("Canines: {canines}")
		},
		Some(Species::Felino) => {
			let felines = pets.iter().filter(|s| s.species.as_str() == "FELINO").count();
			println!("Felines: {felines}")
		},
		None => {
			let felines = pets.iter().filter(|s| s.species.as_str() == "FELINO").count();
			let canines = pets.iter().filter(|s| s.species.as_str() != "FELINO").count();
			println!("Felines: {felines} - Canines: {canines}")
		}
	}
}

pub fn count_by_neighborhood(pets: &Vec<Pet>, neighborhood: Option<&str>) {
	match neighborhood {
		Some("USAQUEN") => { 
			let usaquen = pets.iter().filter(|s| s.neighborhood.as_str() == "USAQUEN").count();
			println!("Usaquen: {usaquen}");
		},
		Some("CHAPINERO") => { 
			let chapinero = pets.iter().filter(|s| s.neighborhood.as_str() == "CHAPINERO").count();
			println!("Chapinero: {chapinero}");
		},
		Some("SANTA FE") => { 
			let santa_fe = pets.iter().filter(|s| s.neighborhood.as_str() == "SANTA FE").count();
			println!("Santa Fe: {santa_fe}");
		},
		Some("SAN CRISTOBAL") => { 
			let san_cristobal = pets.iter().filter(|s| s.neighborhood.as_str() == "SAN CRISTOBAL").count();
			println!("San Cristobal: {san_cristobal}");
		},
		Some("TUNJUELITO") => { 
			let tujuelito = pets.iter().filter(|s| s.neighborhood.as_str() == "TUNJUELITO").count();
			println!("Tujuelito: {tujuelito}");
		},
		Some("USME") => { 
			let usme = pets.iter().filter(|s| s.neighborhood.as_str() == "USME").count();
			println!("Usme: {usme}");
		},
		Some("BOSA") => { 
			let bosa = pets.iter().filter(|s| s.neighborhood.as_str() == "BOSA").count();
			println!("Bosa: {bosa}");
		},
		Some("KENNEDY") => { 
			let kennedy = pets.iter().filter(|s| s.neighborhood.as_str() == "KENNEDY").count();
			println!("Kennedy: {kennedy}");
		},
		Some("FONTIBON") => { 
			let fontibon = pets.iter().filter(|s| s.neighborhood.as_str() == "FONTIBON").count();
			println!("Fontibon: {fontibon}");
		},
		Some("ENGATIVA") => { 
			let engativa = pets.iter().filter(|s| s.neighborhood.as_str() == "ENGATIVA").count();
			println!("Engativa: {engativa}");
		},
		Some("SUBA") => { 
			let suba = pets.iter().filter(|s| s.neighborhood.as_str() == "SUBA").count();
			println!("Suba: {suba}");
		},
		Some("B. UNIDOS") => { 
			let unidos = pets.iter().filter(|s| s.neighborhood.as_str() == "B. UNIDOS").count();
			println!("Barrios Unidos: {unidos}");
		},
		Some("A. NARINO") => { 
			let narino = pets.iter().filter(|s| s.neighborhood.as_str() == "A. NARINO").count();
			println!("Antonio Narino: {narino}");
		},
		Some("P. ARANDA") => { 
			let aranda = pets.iter().filter(|s| s.neighborhood.as_str() == "P. ARANDA").count();
			println!("P. Aranda: {aranda}");
		},
		Some("LA CANDELARIA") => { 
			let candelaria = pets.iter().filter(|s| s.neighborhood.as_str() == "LA CANDELARIA").count();
			println!("La Calendaria: {candelaria}");
		},
		Some("R. URIBE") => { 
			let uribe = pets.iter().filter(|s| s.neighborhood.as_str() == "R. URIBE").count();
			println!("R. Uribe: {uribe}");
		},
		Some("C. BOLIVAR") => { 
			let bolivar = pets.iter().filter(|s| s.neighborhood.as_str() == "C. BOLIVAR").count();
			println!("Ciduad Bolivar: {bolivar}");
		},
		Some("MUNICIPIOS ALEDAÑOS BOGOTA D.C.") => { 
			let aledanos = pets.iter().filter(|s| s.neighborhood.as_str() == "MUNICIPIOS ALEDATORIOS BOGOTA D.C.").count();
			println!("Municipios Aledaños: {aledanos}");
		},
		Some(_) => println!("Invalid neighborhood"),
		None => {
			let usaquen = pets.iter().filter(|s| s.neighborhood.as_str() == "USAQUEN").count();
			println!("Usaquen: {usaquen}");
			let chapinero = pets.iter().filter(|s| s.neighborhood.as_str() == "CHAPINERO").count();
			println!("Chapinero: {chapinero}");
			let santa_fe = pets.iter().filter(|s| s.neighborhood.as_str() == "SANTA FE").count();
			println!("Santa Fe: {santa_fe}");
			let san_cristobal = pets.iter().filter(|s| s.neighborhood.as_str() == "SAN CRISTOBAL").count();
			println!("San Cristobal: {san_cristobal}");
			let tujuelito = pets.iter().filter(|s| s.neighborhood.as_str() == "TUNJUELITO").count();
			println!("Tujuelito: {tujuelito}");
			let usme = pets.iter().filter(|s| s.neighborhood.as_str() == "USME").count();
			println!("Usme: {usme}");
			let bosa = pets.iter().filter(|s| s.neighborhood.as_str() == "BOSA").count();
			println!("Bosa: {bosa}");
			let kennedy = pets.iter().filter(|s| s.neighborhood.as_str() == "KENNEDY").count();
			println!("Kennedy: {kennedy}");
			let fontibon = pets.iter().filter(|s| s.neighborhood.as_str() == "FONTIBON").count();
			println!("Fontibon: {fontibon}");
			let engativa = pets.iter().filter(|s| s.neighborhood.as_str() == "ENGATIVA").count();
			println!("Engativa: {engativa}");
			let suba = pets.iter().filter(|s| s.neighborhood.as_str() == "SUBA").count();
			println!("Suba: {suba}");
			let unidos = pets.iter().filter(|s| s.neighborhood.as_str() == "B. UNIDOS").count();
			println!("Barrios Unidos: {unidos}");
			let narino = pets.iter().filter(|s| s.neighborhood.as_str() == "A. NARINO").count();
			println!("Antoino Narino: {narino}");
			let aranda = pets.iter().filter(|s| s.neighborhood.as_str() == "P. ARANDA").count();
			println!("P. Aranda: {aranda}");
			let candelaria = pets.iter().filter(|s| s.neighborhood.as_str() == "LA CANDELARIA").count();
			println!("La Candelaria: {candelaria}");
			let uribe = pets.iter().filter(|s| s.neighborhood.as_str() == "R. URIBE").count();
			println!("R. Uribe: {uribe}");
			let bolivar = pets.iter().filter(|s| s.neighborhood.as_str() == "C. BOLIVAR").count();
			println!("Ciudad Bolivar: {bolivar}");
			let aledanos = pets.iter().filter(|s| s.neighborhood.as_str() == "MUNICIPIOS ALEDAÑOS BOGOTA D.C.").count();
			println!("Municipios Aledaños: {aledanos}");
		}
	}
}


pub enum Position {
	Top,
	Last
}

pub fn find_by_id<'a>(pets: &'a Vec<Pet>, id: &str) -> Option<&'a Pet> {
	pets.into_iter().find(|pet| pet.id == id)
}

pub fn find_by_species<'a>(pets: &'a Vec<Pet>, pos: Option<Position>, species: &str) -> Vec<&'a Pet> {
	let mut pets: Vec<&Pet> = pets.iter().collect();
	match pos {
		Some(Position::Last) => pets.sort_by(|a, b| a.id.cmp(&b.id)),
		Some(Position::Top) => pets.sort_by(|a, b| b.id.cmp(&a.id)),
		None => {}
	}
	pets.into_iter().filter(|pet| pet.species == species).collect()
}

pub fn find_by_sex<'a>(pets: &'a Vec<Pet>, pos: Option<Position>, sex: &str) -> Vec<&'a Pet> {
	let mut pets: Vec<&Pet> = pets.iter().collect();
	match pos {
		Some(Position::Last) => pets.sort_by(|a, b| a.id.cmp(&b.id)),
		Some(Position::Top) => pets.sort_by(|a, b| b.id.cmp(&a.id)),
		None => {}
	}
	pets.into_iter().filter(|pet| pet.sex == sex).collect()
}

pub fn find_by_size<'a>(pets: &'a Vec<Pet>, pos: Option<Position>, size: &str) -> Vec<&'a Pet> {
	let mut pets: Vec<&Pet> = pets.iter().collect();
	match pos {
		Some(Position::Last) => pets.sort_by(|a, b| a.id.cmp(&b.id)),
		Some(Position::Top) => pets.sort_by(|a, b| b.id.cmp(&a.id)),
		None => {}
	}
	pets.into_iter().filter(|pet| pet.size == size).collect()
}

pub fn find_by_potent_dangerous<'a>(pets: &'a Vec<Pet>, pos: Option<Position>, potent: bool) -> Vec<&'a Pet> {
	let mut pets: Vec<&Pet> = pets.iter().collect();
	match pos {
		Some(Position::Last) => pets.sort_by(|a, b| a.id.cmp(&b.id)),
		Some(Position::Top) => pets.sort_by(|a, b| b.id.cmp(&a.id)),
		None => {}
	}
	pets.into_iter().filter(|pet| pet.potent_dangerous == potent).collect()
}

pub fn find_by_neighborhood<'a>(pets: &'a Vec<Pet>, pos: Option<Position>, neighborhood: &str) -> Vec<&'a Pet> {
	let mut pets: Vec<&Pet> = pets.iter().collect();
	match pos {
		Some(Position::Last) => pets.sort_by(|a, b| a.id.cmp(&b.id)),
		Some(Position::Top) => pets.sort_by(|a, b| b.id.cmp(&a.id)),
		None => {}
	}
	pets.into_iter().filter(|pet| pet.neighborhood == neighborhood).collect()
}