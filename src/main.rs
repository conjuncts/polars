use polars::prelude::*;
use polars::df;

fn main() {


    // use macro
    let df = df! [
        "names" => ["a", "b", "c"],
        "values" => [1, 2, 3],
        "values_nulls" => [Some(1), None, Some(3)]
    ];

    

    println!("{:?}", df);
}
