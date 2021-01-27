use std::io::prelude::*;
use std::fs::File;
use rust_sbml::Model;


fn main(){
    let mut file = File::open("./sbml files/enzymekinetics.xml").expect("Unable to open the file");
    let mut contents = String::new();
    file.read_to_string(&mut contents).expect("Unable to read the file");

    let result = Model::parse(&contents);
    println!("{:#?}", result.unwrap().get_list_of_species());

   
}
