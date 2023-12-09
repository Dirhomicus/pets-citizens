use pets_citizens::{manager::*, pet::{Species, Sex, Size}};

fn main() {
    let pets = read_csv();
    // pets.iter().for_each(|pet| println!("{:?}", pet));
    let manager = Manager::new(&pets);
    // manager.write_csv();
    println!("------------ FIND MICROCHIP ------------");
    println!("{:?}", manager.find_by_microchip(900113000295973));
    println!("------------ COUNT SPECIES ------------");
    manager.count_species(None);
    println!("------------ COUNT NEIGHBORHOODS ------------");
    manager.count_by_neighborhood(None);
    println!("------------ FIND ID ------------");
    println!("{:?}", manager.find_by_id("621988000100-CHGF"));
    println!("------------ SPECIES ------------");
    manager.find_by_species(Some(5),None, Species::Canino).iter().for_each(|pet| println!("{:?}", pet));
    println!("------------ SEX ------------");
    manager.find_by_sex(Some(5),Some(Position::Last), Sex::Hembra).iter().for_each(|pet| println!("{:?}", pet));
    println!("------------ SIZE ------------");
    manager.find_by_size(Some(5),Some(Position::Top), Size::Miniatura).iter().for_each(|pet| println!("{:?}", pet));
    println!("------------ POTENT DANGEROUS ------------");
    manager.find_by_potent_dangerous(Some(5),Some(Position::Top), true).iter().for_each(|pet| println!("{:?}", pet));
    println!("------------ NEIGHBORHOOD ------------");
    manager.find_by_neighborhood(Some(5),Some(Position::Last), "USAQUEN").iter().for_each(|pet| println!("{:?}", pet));
}