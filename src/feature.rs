//! this file only provide
//! [feature struct] and [feature_filter] function
//!
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, str};

#[derive(Clone, Serialize, Deserialize)]
pub struct Feature {
    pub name: String,
    pub count: usize,
    pub f_type: String,
    pub percent: f64,
}

#[derive(Serialize, Deserialize)]
pub struct Features {
    pub features: Vec<Feature>,
}

/// arg0: &mut map, arg1: category_name, rest of all: features that belong to the category
macro_rules! insert_into_map{
    ($map:expr, $cate_name:expr ,$($elem:expr),*) => {
        let names = vec!{ $( $elem),*};

        for feature in names.iter(){
            $map.insert(String::from(feature.clone()), String::from($cate_name.clone()));
        }
    }
}

/// note that the count_features is already sorted
pub fn feature_filter(count_features: Vec<(String, usize)>) -> Features {
    let mut category_map: HashMap<String, String> = HashMap::new();
    // second arg is the catgory name
    insert_into_map!(
        &mut category_map,
        "polymorphism",
        "bare fn",
        "generics",
        "impl trait",
        "trait object",
        "trait reference",
        "const generic arg",
        "associate type binding",
        "closure",
        "trait single inherited",
        "trait multi inherited",
        "where constraint"
    );

    insert_into_map!(
        &mut category_map,
        "safety",
        "static",
        "lifetime",
        "unsafe",
        "const fn",
        "unsafe module"
    );

    insert_into_map!(
        &mut category_map,
        "handy",
        "const fn",
        "assign op",
        "repeated element array construct",
        "operator overload",
        "type alias",
        "type never",
        "trait alias",
        "if let",
        "while let"
    );

    insert_into_map!(&mut category_map, "macro", "macro usage", "macro def");

    insert_into_map!(
        &mut category_map,
        "pattern",
        "tuple struct pattern",
        "wildcard pattern",
        "tuple pattern",
        "match",
        "struct pattern",
        "reference pattern",
        "slice pattern",
        "range pattern"
    );

    insert_into_map!(&mut category_map, "exception", "try", "Result");

    insert_into_map!(&mut category_map, "async", "await", "async", "yield");

    insert_into_map!(
        &mut category_map,
        "pointer",
        "box usage",
        "Rc",
        "RefCell",
        "raw pointer",
        "expr box",
        "Arc",
        "Cell"
    );

    let mut sum: usize = 0;
    for (name, count) in &count_features {
        if category_map.contains_key(name) {
            sum += count;
        }
    }
    let sum_f64 = sum as f64;
    let mut features = Vec::new();
    for (name, count) in count_features {
        if category_map.contains_key(&name) {
            let foo = category_map.get(&name);
            let feature = Feature {
                name: name,
                count: count,
                f_type: String::from(foo.unwrap()),
                percent: (count as f64) / sum_f64 * 100.0,
            };
            features.push(feature);
        }
    }
    Features { features: features }
}
