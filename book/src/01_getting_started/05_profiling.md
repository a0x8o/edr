# Profiling

## Perf/DTrace

The [cargo-flamegraph](https://github.com/flamegraph-rs/flamegraph) tool can be used to collect performance profiling data using [perf](<https://en.wikipedia.org/wiki/Perf_(Linux)>) on Linux and [DTrace](https://en.wikipedia.org/wiki/DTrace) on MacOS/Windows and then visualize it as a flamegraph. This only works when executing `edr` from Rust, so it's mostly used to profile the [scenarios](../02_development/01_tools.md#scenarios) in the repository.

## Instructions

Install the `cargo-flamegraph` tool by running:

```bash
cargo install flamegraph
```

(If you're on Linux, check the [readme](https://github.com/flamegraph-rs/flamegraph?tab=readme-ov-file#installation) for distro specific instructions.)

Then create the flamegraph from the repo root, for example for the `seaport` scenario with:

```bash
CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph -o flamegraph_seaport.svg --root --release -- scenario crates/tools/scenarios/neptune-mutual-blue-protocol_8db6480.jsonl.gz
```

The flamegraph will be saved to `flamegraph_seaport.svg`.

## Event Tracing

It's possible to profile the execution of `edr` by collecting [execution traces](https://docs.rs/tracing/latest/tracing/) and then turning them into flamegraphs. This has the advantage that the contents of the flamegraph can be filtered on the tracing level, and it works when EDR is ran from JS.

### Instructions

```bash
pnpm build:tracing
```

When you now run `edr`, it will generate a `tracing.folded` file in the current working directory. Once the profiling run has completed, we can use [`inferno`](https://docs.rs/tracing-flame/latest/tracing_flame/#generating-the-image) to generate flamegraphs from the collected data. To install `inferno`, run:

```bash
cargo install inferno
```

When we want to analyze the run with its exact order preserved, run:

```bash
cat tracing.folded | inferno-flamegraph --flamechart > tracing-flamechart.svg
```

Alternatively, when we don't care about those details, a flamegraph with identical stack frames collapsed can be generated by running:

```bash
cat tracing.folded | inferno-flamegraph > tracing-flamegraph.svg
```
