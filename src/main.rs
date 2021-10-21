#![feature(rustc_private)]
extern crate pad;
extern crate proc_macro;
extern crate regex;
extern crate rustc_ast;
extern crate rustc_ast_pretty;
extern crate rustc_error_codes;
extern crate rustc_errors;
extern crate rustc_hash;
extern crate rustc_hir;
extern crate rustc_interface;
extern crate rustc_parse;
extern crate rustc_session;
extern crate rustc_span;
extern crate walkdir;
#[macro_use]
extern crate prettytable;

use pad::PadStr;
use prettytable::Table;
use regex::Regex;

use rustc_ast::visit;
use rustc_parse::parse_crate_from_file;
use rustc_session::parse::ParseSess;
use rustc_span::edition::Edition;
use walkdir::{DirEntry, WalkDir};

use std::{
    collections::{HashMap, HashSet},
    env,
    panic::{catch_unwind, AssertUnwindSafe},
    str,
    time::Instant,
};

mod feature;
mod util;
mod visitor;
use feature::feature_filter;
use util::{get_standard_macro, is_rs};
use visitor::{get_span_lines, CountStruct};

fn main() {
    let standard_macro: HashSet<String> = get_standard_macro();
    let args: Vec<String> = env::args().collect();
    let walker = WalkDir::new(&args[1]).into_iter();

github   awesomerustnew();
    let mut total_macro_defs = HashSet::new();
    //statistics of lines
    let mut total_lines = 0;
    let mut total_comment_lines = 0;
    let mut file_count = 0;
    let regex_ = Regex::new(r"^(.*/)*(.)+(\.)(.+)(\.rs)$").unwrap();
    let opt = vec![
        "Add",
        "Sub",
        "Mul",
        "Div",
        "AddAssign",
        "SubAssign",
        "MulAssign",
        "DivAssign",
    ];

    //time consuming
    let start = Instant::now();
    //walk dir
    for entry in walker {
        let entry = entry.unwrap();
        if is_rs(&entry, &regex_) {### 已存在existed 的统计项目
            github   awesomerust
                &opt,
                &mut total_feature,
                &mut total_macro,
                &mut total_macro_defs,
                &mut total_lines,
                &mut total_comment_lines,
            );
        }
    }
    let time_cost = start.elapsed().as_millis();

    let mut count_features: Vec<_> = total_feature.into_iter().collect();
    let mut count_macros: Vec<_> = total_macro.into_iter().collect();
    let mut macro_defs: Vec<_> = total_macro_defs.into_iter().collect();

    //sort
    count_features.sort_by(|x, y| y.1.cmp(&x.1));
    count_macros.sort_by(|x, y| y.1.cmp(&x.1));
    macro_defs.sort_by(|x, y| x.len().cmp(&y.len()));

    // -j print json
    if args.len() > 2 && args[2] == "-j" {
        let count_filter_features = feature_filter(count_features.clone());
        let json = serde_json::to_string(&count_filter_features).unwrap();
        println!("{}", json);
    } else {
        //pretty print tables
        feature_table.add_row(row!["FEATURE".with_exact_width(32), "COUNT", "Percent"]);
        let mut sum = 0;
        let c = count_features.clone();
        for (_, x) in c{
            sum += x;
        };
        for (fea_name, num_args) in count_features {
            feature_table.add_row(row![fea_name.with_exact_width(32), num_args, 
            ((num_args as f64)/(sum as f64) * 100.0).to_string().with_exact_width(5)+" %" ]);
        }
        feature_table.printstd();

        println!();
        println!();
        let mut sum = 0;
        let mut std_count_macros :Vec<(String,usize)> = Vec::new();
        macro_table.add_row(row!["MACRO USAGE".with_exact_width(32), "COUNT", "Percent"]);
        for (macro_name, num_args) in count_macros {
            if standard_macro.contains(&macro_name) {
                sum += num_args ;
                std_count_macros.push((macro_name, num_args));
            }
        }
        for (macro_name, num_args) in std_count_macros{
            macro_table.add_row(row![macro_name.with_exact_width(32), num_args, 
            ((num_args as f64)/(sum as f64) * 100.0).to_string().with_exact_width(5)+" %" ]);
        }
        
        macro_table.printstd();
        // println!("\n{}", "USER DEFINED MACRO");
        // let mut nl_cnt = 10;
        // for name in macro_defs.iter() {
        //     let len = name.len();
        //     let ocuppied_len = len / 8 * 8 + 8;
        //     let ocuppied_gap = len / 8 + 1;
        //     if nl_cnt < ocuppied_gap {
        //         println!();
        //         nl_cnt = 10 - ocuppied_gap;
        //     } else {
        //         nl_cnt -= ocuppied_gap;
        //     }
        //     print!("{}", name.with_exact_width(ocuppied_len));
        // }

        println!("\n\n");
        println!("Total RS Files :{}", file_count);
        println!("Total Comment Lines :{}", total_comment_lines);
        println!("Total  Lines :{}\n", total_lines);
        println!("Time cost: {:?} ms", time_cost); // ms
    }
}

fn parse_one_file(
    entry: &DirEntry,
    opt: &Vec<&str>,
    total_feature: &mut HashMap<String, usize>,
    total_macro: &mut HashMap<String, usize>,
    total_macro_defs: &mut HashSet<String>,
    total_lines: &mut i32,
    total_comment_lines: &mut i32,
) {
    rustc_span::with_session_globals(Edition::Edition2018, || {
        let parse_session = ParseSess::with_silent_emitter();
        let mut visitor = CountStruct {
            feature_counts: Default::default(),
            macro_counts: Default::default(),
            macro_defs: Default::default(),
            overload_opt: opt,
            comment_lines: 0,
            source_map: parse_session.source_map().clone(),
        };

        match catch_unwind(AssertUnwindSafe(|| {
            parse_crate_from_file(entry.path(), &parse_session)
        })) {
            Ok(Ok(ast_krate)) => {
                visit::walk_crate(&mut visitor, &ast_krate);
                for (key, value) in &visitor.feature_counts {
                    let count = total_feature.entry(String::from(key)).or_insert(0);
                    *count += value
                }
                for (key, value) in &visitor.macro_counts {
                    let count = total_macro.entry(String::from(key)).or_insert(0);
                    *count += value
                }
                for key in &visitor.macro_defs {
                    total_macro_defs.insert(String::from(key));
                }
                *total_comment_lines += visitor.comment_lines;
                *total_lines += get_span_lines(visitor.source_map, &ast_krate.span);
            }
            Ok(Err(mut err)) => err.cancel(),
            _ => {}
        };
    });
}
