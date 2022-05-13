use polars::df;
//use polars::error::PolarsError;
use polars::prelude::{DataFrame, NamedFrom, Result, Series};

/*
fn test1() {
    let dfd = df!("Fruit" => &["Apple", "Apple", "Pear"],
              "Color" => &["Red", "Yellow", "Green"]);

    assert_eq!(dfd[0], Series::new("Fruit", &["Apple", "Apple", "Pear"]));
    assert_eq!(dfd[1], Series::new("Color", &["Red", "Yellow", "Green"]));
}
*/

fn main() {
    let dfa = DataFrame::default();
    assert!(dfa.is_empty());
    println!("{:?}", dfa);

    let s1 = Series::new("Fruit", &["Apple", "Apple", "Pear"]);
    let s2 = Series::new("Color", &["Red", "Yellow", "Green"]);

    let dfb: Result<DataFrame> = DataFrame::new(vec![s1, s2]);
    println!("{:?}", dfb.unwrap());

    let dfc: Result<DataFrame> = df!("Fruit" => &["Apple", "Apple", "Pear"],
                                    "Color" => &["Red", "Yellow", "Green"]);
    println!("{:?}", dfc.unwrap());
}
