use std::env::current_dir;
use std::path::Path;

use xbrl_parser::parser::{read_file, read_linkfile};

fn main() -> anyhow::Result<()> {
    let base_path = Path::new(&current_dir()?).join("PublicDoc");
    let schema_path = base_path.join("jpcrp030000-asr-001_E04708-000_2022-03-20_01_2022-06-10_pre.xml");
    let text = read_file(schema_path)?;
    let pre_link = read_linkfile(&text)?;

    println!("{:#?}", pre_link);
    Ok(())
}
