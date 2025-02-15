use id::{config::read_id_config, wad::init_multiple_files};

extern crate id;

fn main() -> Result<(), String> {
    let id_config = read_id_config()?;

    //shareware wad for testing
    let mut wad_file = id_config.data.id_data.clone();
    wad_file.push("doom1.wad");

    let files = vec![wad_file];

    init_multiple_files(&files)?;

    Ok(())
}
