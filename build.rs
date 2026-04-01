use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let c_dir = manifest_dir.join("CowsAndBulls-in-C"); // percorso relativo alla root del crate

 cc::Build::new()
        .include(&c_dir)
        .include(&c_dir.join("src"))
        .define("_CRT_SECURE_NO_WARNINGS", None)
        .files(&[
            //src/api/
            c_dir.join("src/api/cab_api.c"),
            c_dir.join("src/api/cab_io_api.c"),
            c_dir.join("src/api/cab_string.c"),
            //src/core/
            c_dir.join("src/core/attempts.c"),
            c_dir.join("src/core/guess.c"),
            c_dir.join("src/core/index_array.c"),
            c_dir.join("src/core/word_set_filter.c"),
            c_dir.join("src/core/word_set.c"),
            c_dir.join("src/core/word.c"),
            c_dir.join("src/core/vocabolary.c"),
            //src/cmd/
            c_dir.join("src/cmd/cmd.c"),
            c_dir.join("src/cmd/cmd_attempts.c"),
            c_dir.join("src/cmd/cmd_list.c"),
            //src/io
            c_dir.join("src/io/cab_files.c"),
            c_dir.join("src/io/cab_input.c"),
            c_dir.join("src/io/cab_output.c"),
            //src/game
            c_dir.join("src/game/cab_game.c"),
            c_dir.join("src/game/cab_session.c"),
        ])
        .compile("cab_api");

    println!("cargo:rerun-if-changed=src/api/cab_api.c");
    println!("cargo:rerun-if-changed=src/api/cab_io_api.c");
    println!("cargo:rerun-if-changed=src/api/cab_string.c");
    println!("cargo:rerun-if-changed=src/core/word.c");
    println!("cargo:rerun-if-changed=src/core/word_set.c");
    println!("cargo:rerun-if-changed=src/core/guess.c");
    println!("cargo:rerun-if-changed=src/core/attempts.c");
    println!("cargo:rerun-if-changed=src/core/index_array.c");
    println!("cargo:rerun-if-changed=src/core/word_set_filter.c");
    println!("cargo:rerun-if-changed=src/core/vocabolary.c");
    println!("cargo:rerun-if-changed=src/cmd/cmd.c");
    println!("cargo:rerun-if-changed=src/cmd/cmd_attempts.c");
    println!("cargo:rerun-if-changed=src/cmd/cmd_list.c");
    println!("cargo:rerun-if-changed=src/io/cab_files.c");
    println!("cargo:rerun-if-changed=src/io/cab_input.c");
    println!("cargo:rerun-if-changed=src/io/cab_output.c");
    println!("cargo:rerun-if-changed=src/game/cab_session.c");
    println!("cargo:rerun-if-changed=src/game/cab_game.c");
}