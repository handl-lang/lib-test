//! `test` — Handl stdlib port.

/// Checkout root of this repo (build-time).
pub const ROOT: &str = env!("CARGO_MANIFEST_DIR");

pub const ASSERT_HNL: &str = include_str!("assert.hnl");

pub const BENCHMARK_HNL: &str = include_str!("benchmark.hnl");

pub const FUZZ_HNL: &str = include_str!("fuzz.hnl");

pub const PROPERTY_HNL: &str = include_str!("property.hnl");

pub const SNAPSHOT_HNL: &str = include_str!("snapshot.hnl");

pub const TEST_HNL: &str = include_str!("test.hnl");
