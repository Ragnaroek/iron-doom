use crate::{game::GameState, p_setup::load_lump, util::DataReader};

pub struct Column {
    /* TODO
    Field 	Type 	Size 	Offset 	Description
    topdelta 	uint8_t 	1 	0 	The y offset of this post in this patch. If 0xFF, then end-of-column (not a valid post)
    length 	uint8_t 	1 	1 	Length of data in this post
    ------unused 	uint8_t 	1 	2 	Unused padding byte; prevents error on column underflow due to loss of precision.
    data 	uint8_t[] 	length 	3 	Array of pixels in this post; each data pixel is an index into the Doom palette.
    -----unused 	uint8_t 	1 	3 + length 	Unused padding byte; prevents error on column overflow due to loss of precision.
    */
    pub top_delta: usize,
    pub length: usize,
    pub data: Vec<u8>,
}

pub struct Patch {
    pub width: usize,
    pub height: usize,
    pub left_offset: usize,
    pub top_offset: usize,
    pub column_ofs: Vec<i32>, // width many
    pub columns: Vec<Column>,
}

pub fn load_patch_lump(game_state: &mut GameState, lump: usize) -> Result<Patch, String> {
    println!("\n load patch lump");
    let lump = load_lump(game_state, lump)?;
    let mut reader = DataReader::new(&lump);

    let width = reader.read_u16() as usize;
    let height = reader.read_u16() as usize;
    let left_offset = reader.read_u16() as usize;
    let top_offset = reader.read_u16() as usize;

    let num_columns = width;
    let mut column_ofs = Vec::with_capacity(num_columns);
    for _ in 0..num_columns {
        column_ofs.push(reader.read_i32());
    }

    // read column data
    let mut columns = Vec::with_capacity(width);
    for i in 0..num_columns {
        let offset = column_ofs[i];
        reader.set_offset(offset as usize);
        // read column
        let top_delta = reader.read_u8() as usize;
        let length = reader.read_u8() as usize;
        reader.skip(1); //unused
        let mut data = Vec::with_capacity(length);
        for _ in 0..length {
            data.push(reader.read_u8());
        }
        reader.skip(1); //unused

        columns.push(Column {
            top_delta,
            length,
            data,
        })
    }

    Ok(Patch {
        width,
        height,
        left_offset,
        top_offset,
        column_ofs,
        columns,
    })
}
