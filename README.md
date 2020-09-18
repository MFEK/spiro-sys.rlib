# Rust binding to libspiro

This is the low-level binding built with `bindgen`.

`build.rs` builds `libspiro` from the Git submodule ([GitHub `fontforge/libspiro`](https://github.com/fontforge/libspiro/issues)).

TODO: Safe Rust wrapper.

Note: LICENSE file refers only to my `build.rs`. Refer to `libspiro` for C code licensing and authorship.

## Example

Please see `tests/spiro_to_beziers.rs`, which outputs an SVG path for the below shape.
