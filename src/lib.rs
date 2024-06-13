use polars::io::parquet::write::ParquetWriter;
use polars::prelude::SerReader;
use polars::{frame::DataFrame, io::csv::read::CsvReadOptions};
use std::fs::File;
use std::path::PathBuf;

use chrono::prelude::*;
use polars::prelude::*;

pub fn get_test_df() -> DataFrame {
    let df: DataFrame = df!(
        "integer" => &[1, 2, 3],
        "date" => &[
                NaiveDate::from_ymd_opt(2025, 1, 1).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                NaiveDate::from_ymd_opt(2025, 1, 2).unwrap().and_hms_opt(0, 0, 0).unwrap(),
                NaiveDate::from_ymd_opt(2025, 1, 3).unwrap().and_hms_opt(0, 0, 0).unwrap(),
        ],
        "float" => &[4.0, 5.0, 6.0],
        "string" => &["a", "b", "c"],
    )
    .unwrap();
    return df;
}

pub fn normalize_df_colnames(df: &mut DataFrame) {
    for colname in df.get_column_names_owned().into_iter() {
        df.rename(&colname, &colname.trim().to_lowercase().replace(" ", "_"))
            .unwrap();
    }
}

pub fn create_test_csv_file(csv_file_path: &str) {
    let mut df = get_test_df();

    let mut csv_file_out = File::create(csv_file_path).expect("could not create file");
    CsvWriter::new(&mut csv_file_out)
        .include_header(true)
        .with_separator(b',')
        .finish(&mut df)
        .unwrap();

    let df_in = CsvReadOptions::default()
        .try_into_reader_with_file_path(Some(csv_file_path.into()))
        .unwrap()
        .finish()
        .unwrap();
    println!("{}", df_in)
}

pub fn df_from_csv(path: &PathBuf) -> DataFrame {
    CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some(path.into()))
        .unwrap()
        .finish()
        .unwrap()
}

pub fn df_to_parquet(mut df: &mut DataFrame, path: &PathBuf) {
    let mut file = std::fs::File::create(&path).unwrap();
    ParquetWriter::new(&mut file).finish(&mut df).unwrap();
}
