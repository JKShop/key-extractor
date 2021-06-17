pub fn read_csv(data: Vec<u8>) -> Vec<String> {
    let delimiters = [b',', b';', b'\t', b' ', b'|'];
    let mut rdr_r = vec![];
    for delimiter in delimiters {
        let mut rdr = csv::ReaderBuilder::new()
            .delimiter(delimiter)
            .has_headers(false)
            .from_reader(data.as_slice());
        rdr_r.append(
            rdr.records()
                .filter_map(|r| match r {
                    Ok(v) => v.get(0).map(|v| v.to_string()),
                    Err(_) => None,
                })
                .collect::<Vec<String>>()
                .as_mut(),
        );
    }

    rdr_r.sort_unstable();
    rdr_r.dedup();

    rdr_r
}
