use chrono::prelude::*;
use dwrench::{get_test_df, normalize_df_colnames};
use polars::frame::DataFrame;
use polars::prelude::*;
use polars_core::assert_df_eq;

#[test]
fn test_get_test_df() -> Result<(), Box<dyn std::error::Error>> {
    let df = get_test_df();
    let df2: DataFrame = df!(
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
    assert_df_eq!(df, df2);
    Ok(())
}

#[test]
fn test_normalize_colnames() -> Result<(), Box<dyn std::error::Error>> {
    let mut df: DataFrame = df!(
        "Weird integer" => &[1, 2, 3],
        "Weird float " => &[4.0, 5.0, 6.0],
        " Weird string" => &["a", "b", "c"],
    )
    .unwrap();

    normalize_df_colnames(&mut df);

    assert_eq!(
        df.get_column_names(),
        &["weird_integer", "weird_float", "weird_string"]
    );
    Ok(())
}
