use polars::prelude::*;
// use polars::df;

fn unnest_all_structs(df: &DataFrame) -> Result<DataFrame, PolarsError> {
    let df = df.clone();
    let mut cols = Vec::new();

    let schema = df.schema();
    for (k, dtype) in schema.iter() {
        if dtype.is_struct() {
            cols.push(k.clone());
            // df = df.unnest(["address"])?;
        }
    }
    let df = df.unnest(cols)?;
    Ok(df)


    
}

fn unnest_debug() -> Result<DataFrame, PolarsError> {
    let df = df! (
        "nrs" => &[Some(1), Some(2), Some(3), None, Some(5)],
        "names" => &["foo", "ham", "spam", "egg", "spam"],
        "random" => &[0.37454, 0.950714, 0.731994, 0.598658, 0.156019],
        "groups" => &["A", "A", "B", "A", "B"],
        "street" => &["123 Main St", "456 Maple Ave", "789 Elm Rd", "101 Oak St", "202 Pine St"],
        "city" => &["New York", "Los Angeles", "Chicago", "Houston", "Phoenix"],
        "zip" => &["10001", "90001", "60601", "77001", "85001"],
    )?;

    let df2 = df
        .clone()
        .lazy()
        // .with_columns([
        .select([
            col("groups"),
            // col("nrs"),
            as_struct(vec![
                col("street"),
                col("city"),
                col("zip"),
            ]).alias("address"),
            as_struct(vec![
                col("nrs"),
                col("names"),
                col("random"),
            ]).alias("info"),
        ])
        .collect()?;
    println!("df2: {}", df2);
    Ok(df2)

}

fn main() -> Result<(), PolarsError> {

    let df = unnest_debug()?;
    // let df = df! (
    //     "nrs" => &[Some(1), Some(2), Some(3), None, Some(5)],
    //     "names" => &["foo", "ham", "spam", "egg", "spam"],
    //     "random" => &[0.37454, 0.950714, 0.731994, 0.598658, 0.156019],
    //     "groups" => &["A", "A", "B", "A", "B"],
    //     "street" => &["123 Main St", "456 Maple Ave", "789 Elm Rd", "101 Oak St", "202 Pine St"],
    //     "city" => &["New York", "Los Angeles", "Chicago", "Houston", "Phoenix"],
    //     "zip" => &["10001", "90001", "60601", "77001", "85001"],
    // )?;

    println!("df: {}", df);

    let result = unnest_all_structs(&df)?;
    println!("result: {}", result);

    // https://docs.pola.rs/user-guide/expressions/basic-operations/#basic-arithmetic
    // let result = df
    // .clone()
    // .lazy()
    // .select([
    //     (col("nrs") + lit(5)).alias("nrs + 5"),
    //     (col("nrs") - lit(5)).alias("nrs - 5"),
    //     (col("nrs") * col("random")).alias("nrs * random"),
    //     (col("nrs") / col("random")).alias("nrs / random"),
    //     (col("nrs").pow(lit(2))).alias("nrs ** 2"),
    //     (col("nrs") % lit(3)).alias("nrs % 3"),
    // ])
    // .collect()?;
    // println!("{}", result);
    Ok(())
}
