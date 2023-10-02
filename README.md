### Bazel / rules_rust / prost example

10/02/2023

At the time of this writing the documentation for `rules_rust` isn't quite enough to get a working server. There are a few omissions. This is working code. It consolidates a couple examples and includes a few fixes.

 * `futures-core` is required in `WORKSPACE` and the `tonic_runtime` `rust_library_group` as tonic uses it when generating streaming gRPCs.
 * This shows where to put everything, making clearer which code goes in WORKSPACE or not.
 * It keeps the example proto in its own module to make it clearer what depends on what (and what doesn't)
 * Makes it clearer that only one crates_repository is needed, where the docs use different crates_repository names and configurations in different sections.
 * An out-of-date patch for protoc-gen-prost is removed
 * Deprecated parameters are removed
 * Example grpcurl commands are included for each server method type

