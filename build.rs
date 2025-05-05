fn main() {
    cc::Build::new()
        .files(["CSTL/lib/type.c", "CSTL/lib/vector.c", "CSTL/lib/xstring.c"])
        .std("c11")
        .compile("CSTL");

    let bindings = bindgen::builder()
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![allow(non_upper_case_globals)]")
        .headers([
            "CSTL/lib/alloc.h",
            "CSTL/lib/type.h",
            "CSTL/lib/vector.h",
            "CSTL/lib/xstring.h",
        ])
        .allowlist_item("CSTL_\\w*")
        .generate_cstr(true)
        .prepend_enum_name(false)
        .disable_nested_struct_naming()
        .generate()
        .expect("failed to generate bindings");

    bindings
        .write_to_file("src/lib.rs")
        .expect("failed to write bindings");
}
