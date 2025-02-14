use std::path::Path;
use std::path::PathBuf;

fn main() {
    // create bindings
    let include_dir = "external/shaderc/libshaderc/include";
    let target_dir = "external/shaderc_bindings";

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

    // build with cmake
    let root_dir = get_root_dir(); // root directory of this rust workspace
    let spirv_tools_dir = root_dir
        .join("external")
        .join("SPIRV-Tools");
    let spirv_headers_dir = spirv_tools_dir
        .join("external")
        .join("SPIRV-Headers");

    let spirv_tools_dir_string = spirv_tools_dir.to_str().expect("failed to get str from path");
    let spirv_headers_dir_string = spirv_headers_dir.to_str().expect("failed to get str from path");

    let dst = cmake::Config::new("external/shaderc")
        .define("DISABLE_EXCEPTIONS", "ON")
        .define("DISABLE_RTTI", "ON")
        .define("SHADERC_SKIP_COPYRIGHT-CHECK", "ON")
        .define("SHADERC_SKIP_EXAMPLES", "ON")
        .define("SHADERC_SKIP_INSTALL", "ON")
        .define("SHADERC_SKIP_TESTS", "ON")
        .define("SHADERC_SPIRV_HEADERS_DIR", spirv_headers_dir_string)
        .define("SHADERC_SPIRV_TOOLS_DIR", spirv_tools_dir_string)
        .build();

    panic!("hoi \"{:?}\"", dst);
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

fn get_root_dir() -> PathBuf {
    let output = std::process::Command::new(env!("CARGO"))
        .arg("locate-project")
        .arg("--workspace")
        .arg("--message-format=plain")
        .output()
        .expect("command had no output")
        .stdout;
    let utf8 = std::str::from_utf8(&output)
        .expect("failed to convert output to utf8")
        .trim();
    let cargo_path = Path::new(utf8);

    let root_dir = cargo_path
        .parent()
        .expect("cargo path had no parent")
        .to_path_buf();

    root_dir
}
