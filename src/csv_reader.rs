use crate::pet::*;

pub fn read_csv() -> Vec<Pet> {
	let file = std::fs::File::open("pets-citizens.csv").expect("no pets-citizens.csv file found!");
	let mut reader = csv::ReaderBuilder::new().delimiter(b';').from_reader(file);
    reader.deserialize::<Pet>().filter_map(Result::ok).map(assign_id).collect()
}

fn assign_id(pet: Pet) -> Pet {
    let number: String = pet.microchip.to_string().chars().rev().take(12).collect();
    let species_letter = if let Species::Canino = pet.species { 'C' } else { 'F' };
    let sex_letter = if let Sex::Hembra = pet.sex { 'H' } else { 'M' };
    let size_letters = match pet.size {
        Size::Miniatura => "MI",
        Size::Pequeño => "P",
        Size::Mediano => "M",
        Size::Grande => "G",
		Size::MuyGrande => "MG",
    };
    let potent_letter = if pet.potent_dangerous { 'T' } else { 'F' };
    let id = format!("{number}-{species_letter}{sex_letter}{size_letters}{potent_letter}");
    Pet { id, microchip: pet.microchip, species: pet.species, sex: pet.sex, size: pet.size, potent_dangerous: pet.potent_dangerous, neighborhood: pet.neighborhood }
}