# Description

The idea is to test how the type inference during compilation time (`rustc main.rs`) affects the build
time.

The source file `src/main.rs` generates (prints) a Rust source where multiple of variables are defined.
We can generate (after manual change of `src/main.rs`) them with and without the type described.

```bash
$ time rustc gen_without_types.rs
rustc gen_without_types.rs  9.94s user 0.59s system 100% cpu 10.525 total
$ time rustc gen_with_types.rs
rustc gen_with_types.rs  1.18s user 0.66s system 100% cpu 1.827 total
```
So, there is a clearly difference.

This is really just a simple test for trying to see if there is a case for explicit typing (apart from readability). It looks like it is.
