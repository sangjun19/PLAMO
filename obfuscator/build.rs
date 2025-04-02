use lalrpop::process_root;

fn main() {
    process_root().unwrap();
    println!("cargo:rerun-if-changed=src/mbaexpr.lalrpop");
}