use crate::csv::read_csv;
use crate::doc::try_read_doc;
pub use calamine::Error as CalamineErr;


pub mod csv;
pub mod doc;

pub fn try_parse_file(file_content: Vec<u8>, sheet_name: &str) -> Result<Vec<String>, CalamineErr> {
    let ft_inf = infer::get(&file_content);
    let ft_tm = tree_magic::from_u8(&file_content);
    log::debug!("{:?}", ft_inf);
    log::debug!("{:?}", ft_tm);

    if ft_tm == "text/plain" {
        let csv_keys = read_csv(file_content);
        if !csv_keys.is_empty() {
            return Ok(csv_keys);
        }
    } else {
        match ft_inf {
            None => {
                log::warn!("Unknown filetype:\n{:?}\n{:?}", ft_inf, ft_tm);
            }
            Some(v) => match v.extension() {
                "xls" | "xla" | "xlsx" | "xlsm" | "xlam" | "ods" => {
                    return try_read_doc(file_content, v.extension(), sheet_name)
                }
                &_ => {}
            },
        }
    }
    Err(CalamineErr::Msg("Unknown filetype"))
}

#[cfg(test)]
mod tests {
    use crate::try_parse_file;
    
    use std::fs::File;
    use std::io::Read;
    use std::path::Path;

    #[test]
    fn it_works() {
        pretty_env_logger::init();
        for file in std::fs::read_dir(Path::new(".item_keys_test")).unwrap() {
            let f = file.unwrap();
            let mut fx: File = File::open(f.path()).unwrap();
            let mut fxc = vec![];
            fx.read_to_end(&mut fxc).unwrap();
            log::debug!("Testing: {}", f.file_name().into_string().unwrap());
            let rows = match try_parse_file(fxc, "Tabelle1") {
                Ok(v) => v,
                Err(e) => {
                    log::error!(
                        "Failed to parse {}:\n{}",
                        f.file_name().into_string().unwrap(),
                        e
                    );
                    continue;
                }
            };

            assert_eq!(rows.len(), 250);
        }
    }
}
