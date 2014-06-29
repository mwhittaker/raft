To build `crate_mods.rs` as a library and build `main.rs` as a binary run:
    
    rustc crate_mods.rs
    rustc -L . main.rs
    ./main
