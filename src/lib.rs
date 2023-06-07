use wasm_bindgen::prelude::*;
use read_fonts::{FontRead, FontRef, TableProvider, TopLevelTable};
use write_fonts::{dump_table, tables::os2::Os2, FontBuilder};

#[wasm_bindgen]
extern {
    pub fn alert(s: &str);
}

#[wasm_bindgen]
pub fn greet(name: &str) {
    alert(&format!("Hello, {}!", name));
}

#[wasm_bindgen]
pub fn mod_os2_us_weight_class(input_path: &str, output_path: &str, value_str: &str,) {
    let value: u16 = value_str.parse().unwrap();

    let font_data = std::fs::read(input_path).unwrap();
    let font = FontRef::new(&font_data).unwrap();

    let os2 = font.os2().expect("missing OS/2 table");
    alert(&format!("os2.us_weight_class(): {}!", os2.us_weight_class()));

    /*
    let mut builder: FontBuilder = FontBuilder::default();

    
    for table in font.table_directory.table_records() {
        let tag = table.tag();

        println!("    Adding table {tag} ...");

        let font_data = font
            .table_data(tag)
            .expect(&format!("Table {tag} not found!"));

        let mut raw_data = font_data.as_ref().to_owned();

        // Modify the OS2 tag
        if tag == Os2::TAG {
            let mut os2 = Os2::read(font_data).unwrap();
            os2.us_weight_class = value;
            raw_data = dump_table(&os2).unwrap();
        }

        builder.add_table(tag, raw_data);
    }

    // Build the font
    let data = builder.build();
    std::fs::write(output_path, data).unwrap();
     */


    alert(&format!("Modified and saved: {}!", output_path));

}