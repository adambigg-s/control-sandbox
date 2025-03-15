fn main() {
    println!("cargo:rerun-if-changed=controller.c");
    cc::Build::new()
        .file("../controller.c")
        .compiler("clang")
        .compile("controller");
}
