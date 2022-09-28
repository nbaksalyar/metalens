# Compilation process for Metalens

This document describes the compilation process for Metalens.

Given an abstract syntax tree (or, rather, a description of a Metalens program in JSON),
the compiler builds an _execution plan_ which consists of multiple _pipelines_.

## Operators

_Operators_ should not be confused with _nodes_. Nodes are high-level program building blocks.
Operators work on a lower level and can be built from formulas contained within nodes.

Each operator works in two modes:

- **Compilation**. This step builds an eBPF program or a JIT-compiled x86 program that processes data - depending
  on the current pipeline mode. The compilation uses the partial evaluation technique to produce the bytecode.
  It treats the execution plan as an _interpreter_ which can be translated into a compiled code using the Futamura projection.
  Because the formula language is functional and declarative, it lends itself into partial evaluation particularly well.
  We use the data-centric (produce/consume) model described in [Neumann2011].

- **Runtime**. The runtime synchronises and coordinates the different data sources.
  Pipelines operate on data buffers represented as Rust async/await `Streams` to pass data accross
  (it can be aggregated data sourced from an eBPF program, data at rest, data sourced from databases, etc.).

Non-blocking operators (like filter or map) can be fused together in a single pipeline for efficiency.

Metalens supports the following operators:

1. **Filter** (non-blocking)
1. **Map** (non-blocking)
1. **Reduce** (non-blocking) - performs aggregate operations.
1. **Uprobe** - `Source` operator.
1. **WebSocketSink** - `Sink` operator. Transfer data from a BPF ringbuf into a WebSocket stream. The target stream is determined by a node ID.
1. **JsonDataSink** - converts and outputs data into JSON.
1. **Window** - defines a window for the stream.
1. **Span** - defines a span for distributed tracing.
1. **Split**
1. **ReadBpfProperties**

Some operators are terminal; they require _materialization_ of data before it can be processed.
In the context of eBPF that could mean that this data might have to be processed on the runtime side
instead of a compiled program side.

Operators follow the simple callback-based interface:

```rust
trait Operator {
  fn exec(&mut self, cb: fn(record: Record));
}
```

#### Pipeline compilation example

Given a program `bashreadline`:

```
[ Uprobe("bash:readline") ]
  -> [ Filter(process.name == "bash") ]
    -> [ Monitor(count(*)) ]
```

it should be compiled into the following pipeline:

```
[WebSocketSink]
  |__ [Aggregate(Sum)]
    |__ [Filter]
      |__ [ReadBpfProperties(["process.name"])]
        |__ [Uprobe]
```

Formulas are compiled at the stage of translating a stream into a pipeline.

### Sink

eBPF version of `Sink` loads data from ring buffers and other types of eBPF maps and sends it further.

## Type information and inference

AST nodes and formulas are not annotated with type information. However, it's inferred from the context
and each operator in the pipeline defines a data type it works with.

## AST lowering

AST describes the _visual_ representation of a Metalens program. I.e. it can contain node types such as
`Chart` which doesn't have a concrete representation on the back-end side. Instead, the `Chart` node is
lowered into a `Sink` operator in the pipeline which collects data from the previous pipeline steps and
transfers it to a WebSocket stream.

[Formulas](formulas.md) are also lowered into basic operators.

## Resources

- Grizzly paper: https://www.nebula.stream/paper/grulich_grizzly_sigmod2020.pdf
- [Neumann2011] Efficiently Compiling Efficient Query Plans for Modern Hardware
- Partial evaluation: https://en.wikipedia.org/wiki/Partial_evaluation
- How to Architect a Query Compiler, Revisited: https://www.cs.purdue.edu/homes/rompf/papers/tahboub-sigmod18.pdf
