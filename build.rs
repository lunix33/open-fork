use std::process::Command;

fn main() {
    let profile = std::env::var("PROFILE").unwrap();
    let manifest_dir = std::env::var("CARGO_MANIFEST_DIR").unwrap();
    let out_dir = std::env::var("OUT_DIR").unwrap();

    copy_static(&manifest_dir, &out_dir);
    build_js_scss(&manifest_dir, &out_dir, &profile);
}

fn copy_static(manifest_dir: &str, out_dir: &str) {
    let static_source = format!("{manifest_dir}/static");
    let static_destination = out_dir.to_string();
    println!("Copy '{static_source}' to '{static_destination}'");
    let copy_options = fs_extra::dir::CopyOptions::new().overwrite(true);
    fs_extra::dir::copy(static_source, static_destination, &copy_options)
        .expect("Failed to copy static assets.");
}

fn build_js_scss(manifest_dir: &str, out_dir: &str, profile: &str) {
    let vite_bin = format!("{manifest_dir}/node_modules/.bin/vite");
    let output_dir = format!("{out_dir}/dist");
    let (args, env) = if profile == "debug" {
        (
            vec![
                "build",
                "--mode",
                "development",
                "--sourcemap",
                "--outDir",
                &output_dir,
            ],
            "development",
        )
    } else {
        (vec!["build", "--outDir", &output_dir], "production")
    };

    let out = Command::new(vite_bin)
        .args(args)
        .env("NODE_ENV", env)
        .output()
        .expect("Failed to run command.");

    println!(
        "vite finished with error code: {}\nSTDOUT:\n{}\n---\nSTDERR:\n{}",
        out.status,
        String::from_utf8_lossy(&out.stdout),
        String::from_utf8_lossy(&out.stderr)
    );
}
