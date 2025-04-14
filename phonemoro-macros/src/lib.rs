mod dataset_parsing;

use dataset_parsing::create_phf_map;
use proc_macro::TokenStream;
use std::path::PathBuf;
use syn::{LitStr, parse_macro_input};

/// Create a perfect hash function map from a json file.
/// The path is relative to the workspace root.
#[proc_macro]
pub fn phm_from_json(tokens: TokenStream) -> TokenStream {
    // parse input
    let passed_data_path = PathBuf::from(parse_macro_input!(tokens as LitStr).value());

    // if relative: Eval from crate root
    // WARNING: CARGO_MANIFEST_DIR must be accessed with std::env:var, not with env!.
    // If env! was used, the path to the phonemoro-macros crate would be inserted, not
    // the path to the crate where the macro was invoked.
    let passed_data_path =
        if passed_data_path.is_relative() {
            PathBuf::from(std::env::var("CARGO_MANIFEST_DIR").expect(
                "CARGO_MANIFEST_DIR has to exist during compile time, something went wrong.",
            ))
            .join(passed_data_path)
        } else {
            passed_data_path
        };

    let data_path = passed_data_path.canonicalize().expect(
        format!(
            "Canonicalization failed. Absolute path before trying to resolve it: '{:?}'",
            ::std::path::absolute(passed_data_path).expect(
                "Getting absolute path of argument path failed, something is really wrong."
            )
        )
        .as_str(),
    );

    // create map from that
    let phf_map_builder = create_phf_map(&data_path).expect("Creating phf map failed.");

    // Convert the built map to a token stream
    format!("{}", phf_map_builder.build())
        .parse()
        .expect("Parsing the built map failed.")
}
