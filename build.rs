fn main() {
    // rerun if any of the source files change
    println!("cargo:rerun-if-changed=CSTL/lib");
    cc::Build::new()
        .files(["CSTL/lib/type.c", "CSTL/lib/vector.c", "CSTL/lib/xstring.c", "CSTL/lib/list.c", "CSTL/lib/alloc.c"])
        .std("c11")
        .compile("CSTL");
}
