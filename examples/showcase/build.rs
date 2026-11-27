use std::{collections::HashMap, env, path::PathBuf};

fn main() {
    let manifest_dir =
        PathBuf::from(env::var_os("CARGO_MANIFEST_DIR").expect("Cargo manifest directory"));
    let libraries = HashMap::from([("tempo-ui".into(), manifest_dir.join("../../ui/index.slint"))]);

    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(libraries)
        .embed_resources(slint_build::EmbedResourcesKind::EmbedFiles);
    slint_build::compile_with_config("ui/demo.slint", config).expect("compile Tempo UI showcase");
}
