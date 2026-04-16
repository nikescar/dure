fn main() {
    rust2go::Builder::new()
        .with_go_src("./go")
        .with_regen_arg(rust2go::RegenArgs {
            src: "./src/lib.rs".into(),
            dst: "./go/gen.go".into(),
            ..Default::default()
        })
        .build();
}
