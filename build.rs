fn main() {
    cc::Build::new()
        .files(["CSTL/lib/type.c", "CSTL/lib/vector.c", "CSTL/lib/xstring.c"])
        .std("c11")
        .compile("CSTL");
}
