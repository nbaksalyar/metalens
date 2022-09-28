# Formula language

Formulas are intervowen with the main program data flow and create an implicit data flow.

Formulas are parsed into an AST first. Every expression is typed.
They use an AST that is separate from the main program AST.

## Operators

### Binary operators

* `=`, `!=` - checks values for equality.
* `>`, `<`, `>=`, `<=` - compares values.
* `in`, `not in` - checks if a value is included in another set of values or not. Can be also used for searching substrings.
* `+` - adds numbers or concatenates strings.
* `*`, `/`, `-` - numerical arithmetic operators.

## Aggregate functions

### Decomposable functions

Decomposable aggregate functions can be computed incrementally.
In this case we need to store only the aggregate value in a BPF map.

* `sum(Vec[Num])` - summates values in a set or a stream.
* `max(Vec[Num])`, `min(Vec[Num])` - returns a minimal or a maximal value in a stream.
* `avg(Vec[Num])` - returns an average value in a stream.
* `count(Vec[T])` - produces a number of values in a set or a stream.

### Non-decomposable functions

Non-decomposable functions need to have access to all records of a window,
so we need to materialise and store values in a BPF map.

* `percentile(Vec[T], k: f64)` - gets `k`th percentile from a set or a stream.
* `intersection(Vec[T], Vec[T])` - returns all values that are contained in both sets.

## Scalar functions

* `len(String)` - returns a length of a string or a window.
* `nstime()` - returns current time in nanoseconds.

## String interpolation

## Node references

Internally, formulas should use node IDs for references. Node names should be used for representation only.
This way, if a node is renamed, the link between a formula and a node will be retained.

If a link is broken, a node/formula code can be highlighted in red.
