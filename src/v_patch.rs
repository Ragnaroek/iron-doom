use crate::{game::GameState, p_setup::load_lump, util::DataReader};

pub struct Post {
    pub top_delta: usize,
    pub length: usize,
    pub data: Vec<u8>,
}

pub struct Column {
    pub posts: Vec<Post>,
}

impl Column {
    fn new() -> Column {
        Column { posts: Vec::new() }
    }

    fn add_post(&mut self, post: Post) {
        self.posts.push(post);
    }
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
        let mut column = Column::new();

        loop {
            // read posts here and append them to the column
            let top_delta = reader.read_u8() as usize;
            if top_delta == 0xFF {
                break;
            }

            let length = reader.read_u8() as usize;
            reader.skip(1); //unused
            let mut data = Vec::with_capacity(length);
            for _ in 0..length {
                data.push(reader.read_u8());
            }
            reader.skip(1); //unused

            column.add_post(Post {
                top_delta,
                length,
                data,
            });
        }

        columns.push(column);
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
