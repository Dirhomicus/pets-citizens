use crate::pet::*;

pub struct Manager<'a> {
	pets: &'a Vec<Pet>
}

pub enum Position {
	Top,
	Last
}

impl<'a> Manager<'a> {
	pub fn new(pets: &'a Vec<Pet>) -> Self {
		Self { pets }
	}

	pub fn write_csv(&self) {
		let mut file = csv::Writer::from_path("new-pets-citizens.csv").expect("bad writer");
		for pet in self.pets {
			if let Err(e) = file.serialize(pet) { eprintln!("{e}") }
		}
	}

	pub fn find_by_microchip(&self, microchip: u64) -> Option<&Pet> {
		self.pets.iter().find(|pet| pet.microchip == microchip)
	}

	pub fn count_species(&self, species: Option<Species>) {
		match species {
			Some(Species::Canino) => {
				let canines = self.pets.iter().filter(|s| s.species == Species::Canino).count();
				println!("Canines: {canines}")
			},
			Some(Species::Felino) => {
				let felines = self.pets.iter().filter(|s| s.species == Species::Felino).count();
				println!("Felines: {felines}")
			},
			None => {
				let canines = self.pets.iter().filter(|s| s.species == Species::Canino).count();
				let felines = self.pets.iter().filter(|s| s.species == Species::Felino).count();
				println!("Felines: {felines} - Canines: {canines}")
			}
		}
	}

	pub fn count_by_neighborhood(&self, neighborhood: Option<&str>) {
		match neighborhood {
			Some("USAQUEN") => { 
				let usaquen = self.pets.iter().filter(|s| s.neighborhood.as_str() == "USAQUEN").count();
				println!("Usaquen: {usaquen}");
			},
			Some("CHAPINERO") => { 
				let chapinero = self.pets.iter().filter(|s| s.neighborhood.as_str() == "CHAPINERO").count();
				println!("Chapinero: {chapinero}");
			},
			Some("SANTA FE") => { 
				let santa_fe = self.pets.iter().filter(|s| s.neighborhood.as_str() == "SANTA FE").count();
				println!("Santa Fe: {santa_fe}");
			},
			Some("SAN CRISTOBAL") => { 
				let san_cristobal = self.pets.iter().filter(|s| s.neighborhood.as_str() == "SAN CRISTOBAL").count();
				println!("San Cristobal: {san_cristobal}");
			},
			Some("TUNJUELITO") => { 
				let tujuelito = self.pets.iter().filter(|s| s.neighborhood.as_str() == "TUNJUELITO").count();
				println!("Tujuelito: {tujuelito}");
			},
			Some("USME") => { 
				let usme = self.pets.iter().filter(|s| s.neighborhood.as_str() == "USME").count();
				println!("Usme: {usme}");
			},
			Some("BOSA") => { 
				let bosa = self.pets.iter().filter(|s| s.neighborhood.as_str() == "BOSA").count();
				println!("Bosa: {bosa}");
			},
			Some("KENNEDY") => { 
				let kennedy = self.pets.iter().filter(|s| s.neighborhood.as_str() == "KENNEDY").count();
				println!("Kennedy: {kennedy}");
			},
			Some("FONTIBON") => { 
				let fontibon = self.pets.iter().filter(|s| s.neighborhood.as_str() == "FONTIBON").count();
				println!("Fontibon: {fontibon}");
			},
			Some("ENGATIVA") => { 
				let engativa = self.pets.iter().filter(|s| s.neighborhood.as_str() == "ENGATIVA").count();
				println!("Engativa: {engativa}");
			},
			Some("SUBA") => { 
				let suba = self.pets.iter().filter(|s| s.neighborhood.as_str() == "SUBA").count();
				println!("Suba: {suba}");
			},
			Some("B. UNIDOS") => { 
				let unidos = self.pets.iter().filter(|s| s.neighborhood.as_str() == "B. UNIDOS").count();
				println!("Barrios Unidos: {unidos}");
			},
			Some("A. NARINO") => { 
				let narino = self.pets.iter().filter(|s| s.neighborhood.as_str() == "A. NARINO").count();
				println!("Antonio Narino: {narino}");
			},
			Some("P. ARANDA") => { 
				let aranda = self.pets.iter().filter(|s| s.neighborhood.as_str() == "P. ARANDA").count();
				println!("P. Aranda: {aranda}");
			},
			Some("LA CANDELARIA") => { 
				let candelaria = self.pets.iter().filter(|s| s.neighborhood.as_str() == "LA CANDELARIA").count();
				println!("La Calendaria: {candelaria}");
			},
			Some("R. URIBE") => { 
				let uribe = self.pets.iter().filter(|s| s.neighborhood.as_str() == "R. URIBE").count();
				println!("R. Uribe: {uribe}");
			},
			Some("C. BOLIVAR") => { 
				let bolivar = self.pets.iter().filter(|s| s.neighborhood.as_str() == "C. BOLIVAR").count();
				println!("Ciduad Bolivar: {bolivar}");
			},
			Some("MUNICIPIOS ALEDAÑOS BOGOTA D.C.") => { 
				let aledanos = self.pets.iter().filter(|s| s.neighborhood.as_str() == "MUNICIPIOS ALEDATORIOS BOGOTA D.C.").count();
				println!("Municipios Aledaños: {aledanos}");
			},
			Some(_) => println!("Invalid neighborhood"),
			None => {
				let usaquen = self.pets.iter().filter(|s| s.neighborhood.as_str() == "USAQUEN").count();
				println!("Usaquen: {usaquen}");
				let chapinero = self.pets.iter().filter(|s| s.neighborhood.as_str() == "CHAPINERO").count();
				println!("Chapinero: {chapinero}");
				let santa_fe = self.pets.iter().filter(|s| s.neighborhood.as_str() == "SANTA FE").count();
				println!("Santa Fe: {santa_fe}");
				let san_cristobal = self.pets.iter().filter(|s| s.neighborhood.as_str() == "SAN CRISTOBAL").count();
				println!("San Cristobal: {san_cristobal}");
				let tujuelito = self.pets.iter().filter(|s| s.neighborhood.as_str() == "TUNJUELITO").count();
				println!("Tujuelito: {tujuelito}");
				let usme = self.pets.iter().filter(|s| s.neighborhood.as_str() == "USME").count();
				println!("Usme: {usme}");
				let bosa = self.pets.iter().filter(|s| s.neighborhood.as_str() == "BOSA").count();
				println!("Bosa: {bosa}");
				let kennedy = self.pets.iter().filter(|s| s.neighborhood.as_str() == "KENNEDY").count();
				println!("Kennedy: {kennedy}");
				let fontibon = self.pets.iter().filter(|s| s.neighborhood.as_str() == "FONTIBON").count();
				println!("Fontibon: {fontibon}");
				let engativa = self.pets.iter().filter(|s| s.neighborhood.as_str() == "ENGATIVA").count();
				println!("Engativa: {engativa}");
				let suba = self.pets.iter().filter(|s| s.neighborhood.as_str() == "SUBA").count();
				println!("Suba: {suba}");
				let unidos = self.pets.iter().filter(|s| s.neighborhood.as_str() == "B. UNIDOS").count();
				println!("Barrios Unidos: {unidos}");
				let narino = self.pets.iter().filter(|s| s.neighborhood.as_str() == "A. NARINO").count();
				println!("Antoino Narino: {narino}");
				let aranda = self.pets.iter().filter(|s| s.neighborhood.as_str() == "P. ARANDA").count();
				println!("P. Aranda: {aranda}");
				let candelaria = self.pets.iter().filter(|s| s.neighborhood.as_str() == "LA CANDELARIA").count();
				println!("La Candelaria: {candelaria}");
				let uribe = self.pets.iter().filter(|s| s.neighborhood.as_str() == "R. URIBE").count();
				println!("R. Uribe: {uribe}");
				let bolivar = self.pets.iter().filter(|s| s.neighborhood.as_str() == "C. BOLIVAR").count();
				println!("Ciudad Bolivar: {bolivar}");
				let aledanos = self.pets.iter().filter(|s| s.neighborhood.as_str() == "MUNICIPIOS ALEDAÑOS BOGOTA D.C.").count();
				println!("Municipios Aledaños: {aledanos}");
			}
		}
	}

	pub fn find_by_id(&self, id: &str) -> Option<&Pet> {
		self.pets.iter().find(|pet| pet.id == id)
	}

	pub fn find_by_species(&self, get: Option<usize>, pos: Option<Position>, species: Species) -> Vec<&Pet> {
		let mut pets: Vec<&Pet> = self.pets.iter().collect();
		match pos {
			Some(Position::Last) => pets.sort_by(|a, b| a.id.cmp(&b.id)),
			Some(Position::Top) => pets.sort_by(|a, b| b.id.cmp(&a.id)),
			None => {}
		}
		match get {
			Some(n) => pets.iter().filter(|pet| pet.species == species).take(n).map(|&pet| pet).collect(),
			None => pets.iter().filter(|pet| pet.species == species).map(|&pet| pet).collect()
		}
	}

	pub fn find_by_sex(&self, get: Option<usize>, pos: Option<Position>, sex: Sex) -> Vec<&Pet> {
		let mut pets: Vec<&Pet> = self.pets.iter().collect();
		match pos {
			Some(Position::Last) => pets.sort_by(|a, b| a.id.cmp(&b.id)),
			Some(Position::Top) => pets.sort_by(|a, b| b.id.cmp(&a.id)),
			None => {}
		}
		match get {
			Some(n) => pets.iter().filter(|pet| pet.sex == sex).take(n).map(|&pet| pet).collect(),
			None => pets.iter().filter(|pet| pet.sex == sex).map(|&pet| pet).collect()
		}
	}

	pub fn find_by_size(&self, get: Option<usize>, pos: Option<Position>, size: Size) -> Vec<&Pet> {
		let mut pets: Vec<&Pet> = self.pets.iter().collect();
		match pos {
			Some(Position::Last) => pets.sort_by(|a, b| a.id.cmp(&b.id)),
			Some(Position::Top) => pets.sort_by(|a, b| b.id.cmp(&a.id)),
			None => {}
		}
		match get {
			Some(n) => pets.iter().filter(|pet| pet.size == size).take(n).map(|&pet| pet).collect(),
			None => pets.iter().filter(|pet| pet.size == size).map(|&pet| pet).collect()
		}
	}

	pub fn find_by_potent_dangerous(&self, get: Option<usize>, pos: Option<Position>, potent: bool) -> Vec<&Pet> {
		let mut pets: Vec<&Pet> = self.pets.iter().collect();
		match pos {
			Some(Position::Last) => pets.sort_by(|a, b| a.id.cmp(&b.id)),
			Some(Position::Top) => pets.sort_by(|a, b| b.id.cmp(&a.id)),
			None => {}
		}
		match get {
			Some(n) => pets.iter().filter(|pet| pet.potent_dangerous == potent).take(n).map(|&pet| pet).collect(),
			None => pets.iter().filter(|pet| pet.potent_dangerous == potent).map(|&pet| pet).collect()
		}
	}

	pub fn find_by_neighborhood(&self, get: Option<usize>, pos: Option<Position>, neighborhood: &str) -> Vec<&Pet> {
		let mut pets: Vec<&Pet> = self.pets.iter().collect();
		match pos {
			Some(Position::Last) => pets.sort_by(|a, b| a.id.cmp(&b.id)),
			Some(Position::Top) => pets.sort_by(|a, b| b.id.cmp(&a.id)),
			None => {}
		}
		match get {
			Some(n) => pets.iter().filter(|pet| pet.neighborhood == neighborhood).take(n).map(|&pet| pet).collect(),
			None => pets.iter().filter(|pet| pet.neighborhood == neighborhood).map(|&pet| pet).collect()
		}
	}	
}
