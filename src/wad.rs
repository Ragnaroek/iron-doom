use std::{
    fs::File,
    os::unix::fs::FileExt,
    path::{Path, PathBuf},
};

use crate::util::DataReader;

#[derive(Debug)]
struct WadInfo {
    identification: String,
    num_lumps: i32,
    info_table_ofs: i32,
}

pub struct LumpInfo {
    name: String,
    handle: usize,
    position: usize,
    size: usize,
}

pub fn init_multiple_files(files: &[PathBuf]) -> Result<Vec<LumpInfo>, String> {
    let mut lumps = Vec::new();

    for i in 0..files.len() {
        add_file(&mut lumps, i, &files[i])?;
    }

    Ok(lumps)
}

pub fn add_file(lumps: &mut Vec<LumpInfo>, handle: usize, path: &Path) -> Result<(), String> {
    let file = File::open(path).map_err(|e| e.to_string())?;

    let mut header_data = vec![0; 12];
    file.read_exact_at(&mut header_data, 0)
        .map_err(|e| e.to_string())?;

    let mut header_reader = DataReader::new(&header_data);
    let header = WadInfo {
        identification: header_reader.read_utf8_string(4),
        num_lumps: header_reader.read_i32(),
        info_table_ofs: header_reader.read_i32(),
    };

    let mut file_info_data = vec![0; (16 * header.num_lumps) as usize];
    file.read_exact_at(&mut file_info_data, header.info_table_ofs as u64)
        .map_err(|e| e.to_string())?;

    let mut info_reader = DataReader::new(&file_info_data);
    for _ in 0..header.num_lumps {
        let file_pos = info_reader.read_i32();
        let size = info_reader.read_i32();
        let name = info_reader.read_utf8_string(8);

        println!("name = {}", name);

        lumps.push(LumpInfo {
            name,
            handle,
            position: file_pos as usize,
            size: size as usize,
        })
    }

    Ok(())
}
