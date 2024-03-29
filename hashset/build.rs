/// This is a hack to support dynamic K values.
/// K values are implemented as a const generic in our code
/// as we expect it to remain constant across executions
/// and benefit from compile-time optimizations.
/// This build script will set the value of K at compile-time
/// from an environment variable, so one can easily build
/// the project "just in time" with the desired K value.
/// This will not re-build if the K value does not change.
fn build_constants() {
    let out_dir: std::path::PathBuf = std::env::var("OUT_DIR")
        .expect("Failed to obtain OUT_DIR")
        .into();
    let mut code = Vec::new();

    println!("cargo:rerun-if-env-changed=K");
    let k: usize = std::env::var("K")
        .unwrap_or_else(|_| "25".into())
        .parse()
        .expect("Failed to parse K");
    assert!(k >= 1, "K must be ≥ 1");
    assert!(k <= 63, "K must be ≤ 63");
    assert!(k % 2 == 1, "K must be odd");
    code.push(format!("pub const K: usize = {k};"));

    let kmer_bits = 2 * k;
    code.push(format!("pub const KMER_BITS: usize = {kmer_bits};"));

    let canon_bits = kmer_bits - 1;
    code.push(format!("pub const CANON_BITS: usize = {canon_bits};"));

    let kt = select_type(kmer_bits);
    code.push(format!("pub type KT = {kt};"));

    std::fs::write(out_dir.join("constants.rs"), code.join("\n"))
        .expect("Failed to write const file");
}

fn select_type(n_bits: usize) -> &'static str {
    match n_bits.next_power_of_two() {
        1 | 2 | 4 | 8 => "u8",
        16 => "u16",
        32 => "u32",
        64 => "u64",
        128 => "u128",
        _ => panic!("Cannot fit {n_bits} bits in a primitive type"),
    }
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    build_constants();
}
