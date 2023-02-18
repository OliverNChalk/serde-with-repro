use serde::Deserialize;
use serde_with::{serde_as, DisplayFromStr, PickFirst};

fn main() {
    #[serde_as]
    #[derive(Debug, Clone, PartialEq, Deserialize)]
    struct MaybeFloatMaybeString(#[serde_as(as = "PickFirst<(DisplayFromStr, _)>")] f64);

    println!("{}", serde_json::from_str::<f64>(r#"8.0e-5"#).unwrap());
    println!(
        "{}",
        serde_json::from_str::<MaybeFloatMaybeString>(r#"8.0e-5"#)
            .unwrap()
            .0
    );
    println!(
        "{}",
        serde_json::from_str::<MaybeFloatMaybeString>(r#""8.0e-5""#)
            .unwrap()
            .0
    );
}
