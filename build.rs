use std::env;
use std::path::PathBuf;

fn main() {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    let c_dir = manifest_dir.join("CowsAndBulls-in-C"); // percorso relativo alla root del crate

    cc::Build::new()
        .include(&c_dir)
        .include(&c_dir.join("src"))
        .files(&[
            //src/api/
            c_dir.join("src/api/cab_api.c"),
            //src/core/
            c_dir.join("src/core/attempts.c"),
            c_dir.join("src/core/guess.c"),
            c_dir.join("src/core/index_array.c"),
            c_dir.join("src/core/word_set_filter.c"),
            c_dir.join("src/core/word_set.c"),
            c_dir.join("src/core/word.c"),
            //src/files
            c_dir.join("src/files/cab_files.c"),
            c_dir.join("src/files/letter_dispositions.c"),
            //src/game
            c_dir.join("src/game/cab_game.c"),
            c_dir.join("src/game/cab_io.c"),
            c_dir.join("src/game/cab_session.c"),
            //src/tools
            c_dir.join("src/tools/cab_data_analysis.c"),
            //src/utils    
            c_dir.join("src/util/utils.c"),
        ])
        .compile("cab_api");

    println!("cargo:rerun-if-changed=src/api/cab_api.c");
    println!("cargo:rerun-if-changed=src/core/word.c");
    println!("cargo:rerun-if-changed=src/core/word_set.c");
    println!("cargo:rerun-if-changed=src/core/guess.c");
    println!("cargo:rerun-if-changed=src/core/attempts.c");
    println!("cargo:rerun-if-changed=src/core/index_array.c");
    println!("cargo:rerun-if-changed=src/core/word_set_filter.c");
    println!("cargo:rerun-if-changed=src/files/cab_files.c");
    println!("cargo:rerun-if-changed=src/files/letter_dispositions.c");
    println!("cargo:rerun-if-changed=src/game/cab_io.c");
    println!("cargo:rerun-if-changed=src/game/cab_session.c");
    println!("cargo:rerun-if-changed=src/game/cab_game.c");
    println!("cargo:rerun-if-changed=src/tools/cab_data_analysis.c");
    println!("cargo:rerun-if-changed=src/util/utils.c");
}