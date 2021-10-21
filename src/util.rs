use regex::Regex;
use std::collections::HashSet;
use walkdir::DirEntry;
/// to define whether a file is a valid rust source file
pub fn is_rs(entry: &DirEntry, re: &Regex) -> bool {
    entry
        .file_name()
        .to_str()
        .map(|s| {
            if re.is_match(s) {
                return false;
            }
            s.ends_with(".rs")
        })
        .unwrap_or(false)
}
pub fn get_standard_macro() -> HashSet<String> {
    vec![
        "asm",
        "concat_idents",
        "format_args_nl",
        "global_asm",
        "is_aarch64_feature_detected",
        "is_arm_feature_detected",
        "is_mips64_feature_detected",
        "is_mips_feature_detected",
        "is_powerpc64_feature_detected",
        "is_powerpc_feature_detected",
        "llvm_asm",
        "log_syntax",
        "trace_macros",
        "assert",
        "assert_eq",
        "assert_ne",
        "cfg",
        "column",
        "compile_error",
        "concat",
        "dbg",
        "debug_assert",
        "debug_assert_eq",
        "debug_assert_ne",
        "env",
        "eprint",
        "eprintln",
        "file",
        "format",
        "format_args",
        "include",
        "include_bytes",
        "include_str",
        "is_x86_feature_detected",
        "line",
        "matches",
        "module_path",
        "option_env",
        "panic",
        "print",
        "println",
        "stringify",
        "thread_local",
        "todo",
        "try",
        "unimplemented",
        "unreachable",
        "vec",
        "write",
        "writeln",
        "assert_matches",
        "debug_assery_matches",
        "inline",
        "test",
        "warn",
        "doc",
        "allow",
        "info",
        "deny",
        "derive",
        "error",
        "no_mangle",
        "display",
    ]
    .into_iter()
    .map(|x| String::from(x))
    .collect::<HashSet<String>>()
}
