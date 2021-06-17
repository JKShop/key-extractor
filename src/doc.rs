use calamine::{Error, Ods, Reader, Xls, Xlsx};
use std::io::{BufReader, Cursor};

pub fn try_read_doc(
    file_data: Vec<u8>,
    file_ending: &str,
    sheet_name: &str,
) -> Result<Vec<String>, Error> {
    let memsheet = try_open_workbook(file_data, file_ending)?;
    let x = match memsheet {
        MemSheets::Xls(mut v) => v
            .worksheet_range(sheet_name)
            .ok_or(Error::Msg("Worksheet not found"))??,
        MemSheets::Xlsx(mut v) => v
            .worksheet_range(sheet_name)
            .ok_or(Error::Msg("Worksheet not found"))??,
        MemSheets::Ods(mut v) => v
            .worksheet_range(sheet_name)
            .ok_or(Error::Msg("Worksheet not found"))??,
    };
    let mut rows = x
        .rows()
        .filter_map(|r| match r.get(0) {
            None => None,
            Some(v) => v.get_string().map(|vx| vx.to_string()),
        })
        .collect::<Vec<String>>();

    rows.sort_unstable();
    rows.dedup();

    Ok(rows)
}

pub enum MemSheets {
    Xls(Xls<BufReader<Cursor<Vec<u8>>>>),
    Xlsx(Xlsx<BufReader<Cursor<Vec<u8>>>>),
    Ods(Ods<BufReader<Cursor<Vec<u8>>>>),
}

pub fn try_open_workbook(file_data: Vec<u8>, file_ending: &str) -> Result<MemSheets, Error> {
    Ok(match file_ending {
        "xls" | "xla" => {
            MemSheets::Xls(Xls::new(BufReader::new(Cursor::new(file_data))).map_err(Error::Xls)?)
        }
        "xlsx" | "xlsm" | "xlam" => {
            MemSheets::Xlsx(Xlsx::new(BufReader::new(Cursor::new(file_data))).map_err(Error::Xlsx)?)
        }
        "ods" => {
            MemSheets::Ods(Ods::new(BufReader::new(Cursor::new(file_data))).map_err(Error::Ods)?)
        }
        _ => {
            unreachable!()
        }
    })
}
