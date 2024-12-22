fn main() {
	if cfg!(all(target_arch = "wasm32", not(target_os = "wasi"))) {
		println!("cargo:rustc-cfg=wasm");
		println!("cargo::rustc-check-cfg=cfg(wasm)");
	}
	if cfg!(any(
		feature = "kv-mem",
		feature = "kv-fdb",
		feature = "kv-tikv",
		feature = "kv-rocksdb",
		feature = "kv-surrealkv",
		feature = "kv-surrealcs",
	)) {
		println!("cargo:rustc-cfg=storage");
		println!("cargo::rustc-check-cfg=cfg(storage)");
	}
}
