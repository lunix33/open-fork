use std::process::Command;

fn main() {
    let profile = std::env::var("PROFILE").unwrap();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();

    copy_static(&manifest_dir, &out_dir);
    build_js(&manifest_dir, &out_dir, &profile);
    build_sass(&manifest_dir, &out_dir, &profile);
}

fn copy_static(manifest_dir: &str, out_dir: &str) {
    let static_source = format!("{manifest_dir}/static");
    let static_destination = format!("{out_dir}");
    println!("Copy '{static_source}' to '{static_destination}'");
    let copy_options = fs_extra::dir::CopyOptions::new().overwrite(true);
    fs_extra::dir::copy(static_source, static_destination, &copy_options)
        .expect("Failed to copy static assets.");
}

fn build_js(manifest_dir: &str, out_dir: &str, profile: &str) {
    let tsc_bin = format!("{manifest_dir}/node_modules/.bin/tsc");
    let output_dir = format!("{out_dir}/static/js");
    let args = if profile == "debug" {
        vec!["--project", "tsconfig.dev.json", "--outDir", &output_dir]
    } else {
        vec!["--outDir", &output_dir]
    };

    let out = Command::new(tsc_bin)
        .args(args)
        .output()
        .expect("Failed to run command.");

    println!(
        "tsc finished with error code: {}\nSTDOUT:\n{}\n---\nSTDERR:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}

fn build_sass(manifest_dir: &str, out_dir: &str, profile: &str) {
    let sass_bin = format!("{manifest_dir}/node_modules/.bin/sass");
    let input_dir = format!("{manifest_dir}/src/");
    let output_dir = format!("{out_dir}/static/");
    let composed_dir = format!("{input_dir}:{output_dir}");
    let args = if profile == "debug" {
        vec!["--embed-sources", "--embed-source-map", &composed_dir]
    } else {
        vec![composed_dir.as_str()]
    };

    let out = Command::new(sass_bin)
        .args(args)
        .output()
        .expect("Failed to run command.");

    println!(
        "sass finished with error code: {}\nSTDOUT:\n{}\n---\nSTDERR:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}
