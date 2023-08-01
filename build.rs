fn main() {
    csbindgen::Builder::default()
        .input_extern_file("src/lib.rs")
        .csharp_dll_name("libbencodex")
        .generate_csharp_file("target/release/dotnet/NativeMethods.g.cs")
        .unwrap();
}