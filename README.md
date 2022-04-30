# Rusty Ray Tracing in One Weekend

## About

An implementation of [_Ray Tracing in One Weekend_](https://raytracing.github.io/books/RayTracingInOneWeekend.html) in Rust.  
This repository covers the first book only.

## Dependencies

- [rand](https://crates.io/crates/rand) - to generate random numbers.
- [glam](https://crates.io/crates/glam) - exclusively for Vec3, because I'm too lazy to implement it myself.  
  It can also transparently use SIMD through Vec3A type, which gives significant increase in performance (~30% on my CPU, depends on scene, CPU, etc.) at the cost of increased memory usage. Should be the same if the processor does not support SIMD.
- [rayon](https://crates.io/crates/rayon) - multithreading. Had to make some adjustments to how the result is outputted, but the performance gain is worth it.
