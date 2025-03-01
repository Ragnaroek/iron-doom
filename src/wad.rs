use std::{
    fs::File,
    os::unix::fs::FileExt,
    path::{Path, PathBuf},
};

use crate::util::DataReader;

#[derive(Debug)]
pub struct LumpInfo {
    pub name: String,
    pub handle: usize, // index into the files array for the WAD files
    pub position: usize,
    pub size: usize,
}

pub fn init_multiple_files(file_paths: &[PathBuf]) -> Result<(Vec<File>, Vec<LumpInfo>), String> {
    let mut lumps = Vec::new();
    let mut files = Vec::new();

    for i in 0..file_paths.len() {
        add_file(&mut files, &mut lumps, i, &file_paths[i])?;
    }

    Ok((files, lumps))
}

pub fn add_file(
    files: &mut Vec<File>,
    lumps: &mut Vec<LumpInfo>,
    handle: usize,
    path: &Path,
) -> Result<(), String> {
    let file = File::open(path).map_err(|e| e.to_string())?;

    let mut header_data = vec![0; 12];
    file.read_exact_at(&mut header_data, 0)
        .map_err(|e| e.to_string())?;

    let mut header_reader = DataReader::new(&header_data);
    let _identification = clean_string(&header_reader.read_utf8_string(4));
    let num_lumps = header_reader.read_i32();
    let info_table_ofs = header_reader.read_i32();

    let mut file_info_data = vec![0; (16 * num_lumps) as usize];
    file.read_exact_at(&mut file_info_data, info_table_ofs as u64)
        .map_err(|e| e.to_string())?;

    let mut info_reader = DataReader::new(&file_info_data);
    for _ in 0..num_lumps {
        let file_pos = info_reader.read_i32();
        let size = info_reader.read_i32();
        let name_raw = info_reader.read_utf8_string(8);
        let name = clean_string(&name_raw);

        lumps.push(LumpInfo {
            name,
            handle,
            position: file_pos as usize,
            size: size as usize,
        })
    }

    files.push(file);

    Ok(())
}

fn clean_string(str: &str) -> String {
    str.replace('\0', "")
}

pub fn check_num_for_name(lump_info: &[LumpInfo], name: &str) -> Option<usize> {
    let name_cmp = name.to_uppercase();
    // scan backwards so patch lump files take precedence
    for i in (0..lump_info.len()).rev() {
        if lump_info[i].name.to_uppercase() == name_cmp {
            return Some(i);
        }
    }
    None
}
