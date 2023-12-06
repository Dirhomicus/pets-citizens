use pets_citizens::{manager::*, pet::{Species, Sex, Size}};

fn main() {
    let pets = read_csv();
    write_csv(&pets);
    pets.iter().for_each(|pet| println!("{:?}", pet));
    println!("------------ FIND MICROCHIP ------------");
    println!("{:?}", find_by_microchip(&pets, 900113000295973));
    println!("------------ COUNT SPECIES ------------");
    count_species(&pets, None);
    println!("------------ COUNT NEIGHBORHOODS ------------");
    count_by_neighborhood(&pets, None);
    println!("------------ FIND ID ------------");
    println!("{:?}", find_by_id(&pets, "3795920-CMGF"));
    println!("------------ SPECIES ------------");
    find_by_species(&pets, Some(5),None, Species::Canino).iter().for_each(|pet| println!("{:?}", pet));
    println!("------------ SEX ------------");
    find_by_sex(&pets, Some(5),Some(Position::Last), Sex::Hembra).iter().for_each(|pet| println!("{:?}", pet));
    println!("------------ SIZE ------------");
    find_by_size(&pets, Some(5),Some(Position::Top), Size::Miniatura).iter().for_each(|pet| println!("{:?}", pet));
    println!("------------ POTENT DANGEROUS ------------");
    find_by_potent_dangerous(&pets, Some(5),Some(Position::Top), true).iter().for_each(|pet| println!("{:?}", pet));
    println!("------------ NEIGHBORHOOD ------------");
    find_by_neighborhood(&pets, Some(5),Some(Position::Last), "USAQUEN").iter().for_each(|pet| println!("{:?}", pet));
}