use std::path::PathBuf;

fn main() {
    let include_dir = "shaderc/libshaderc/include";
    let target_dir = "shaderc_bindings";

    generate_bindings(
        include_dir,
        "shaderc/env.h",
        target_dir,
    );
    generate_bindings(
        include_dir,
        "shaderc/shaderc.h",
        target_dir,
    );
    //generate_bindings(
    //    include_dir,
    //    "shaderc/shaderc.hpp",
    //    target_dir,
    //);
    generate_bindings(
        include_dir,
        "shaderc/status.h",
        target_dir,
    );
    generate_bindings(
        include_dir,
        "shaderc/visibility.h",
        target_dir,
    );
}

fn generate_bindings(include_dir: &str, header: &str, target_dir: &str) {
    let header_path = PathBuf::from(include_dir).join(header);
    let header_path_str = header_path.to_str().expect("failed to convert header path to str");
    println!("cargo:rerun-if-changed={}", header_path_str);

    let bindings = bindgen::Builder::default()
        .header(header_path_str)
        .clang_arg(format!("-I{}", include_dir))
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .generate()
        .expect("failed to generate bindings");

    let filename = header_path
        .file_stem()
        .expect("header path had no file name")
        .to_str()
        .expect("failed to convert OsStr to str");

    let out_dir = std::env::var("OUT_DIR").expect("failed to resolve env OUT_DIR");
    let target_path = PathBuf::from(out_dir)
        .join(target_dir)
        .join(format!("{}.rs", filename));

    let target_parent = target_path.parent().expect("target_path had no parent");
    std::fs::create_dir_all(target_parent).expect("failed to create target_parent");

    bindings.write_to_file(&target_path).expect("failed to write bindings");
}
