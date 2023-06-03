use std::path::PathBuf;

fn main(){
    // build the plugins
    for plugin in ["embedding", "format", "infer"]{
        let path = PathBuf::from("./plugins").join(plugin);
        let status = std::process::Command::new("cargo")
            .args(&["build", "--target", "wasm32-unknown-unknown", "--release"])
            .current_dir(path)
            .status()
            .expect("failed to build plugin");
        assert!(status.success());
    }

    println!("cargo:rerun-if-changed=plugins");
}