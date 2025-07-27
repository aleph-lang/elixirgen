# Elixir Code Generator for `aleph-syntax-tree`

This crate provides a code generator that transforms an abstract syntax tree (AST) from the [`aleph-syntax-tree`](https://github.com/aleph-lang/aleph-syntax-tree) crate into Elixir source code.

## Features

- Converts `AlephTree` expressions into idiomatic Elixir code
- Supports a wide range of constructs:
  - Literals: `Int`, `Float`, `Bool`, `String`, `Bytes`
  - Data structures: `Array`, `Tuple`
  - Control flow: `If`, `While`, `Match`, `Stmts`, `Let`, `LetRec`
  - Functional: `App`, `Return`, `Not`, `And`, `Or`
  - Array manipulation: `Get`, `Put`, `Remove`, `Length`
  - Miscellaneous: `Comment`, `Assert`, `Import`, etc.
- Automatically handles indentation and formatting
- Skips unsupported nodes with clean fallbacks

## Usage

### Add to your project

Make sure you have the [`aleph-syntax-tree`](https://github.com/aleph-lang/aleph-syntax-tree) crate available in your workspace.

### Example

```rust
use aleph_syntax_tree::syntax::AlephTree;
use your_crate::generate;

fn main() {
    let ast = AlephTree::Let {
        var: "x".into(),
        is_pointer: false,
        value: Box::new(AlephTree::Int { value: "42".into() }),
        expr: Box::new(AlephTree::Return {
            value: Box::new(AlephTree::Var { var: "x".into(), is_pointer: false }),
        }),
    };

    let elixir_code = generate(ast);
    println!("{}", elixir_code);
}
```

### Output

```elixir
x = 42
x
```

## Function Signature

```rust
pub fn generate(ast: AlephTree) -> String
```

Returns the Elixir code as a `String`.

## Limitations

- Complex and class-related nodes are currently ignored or return empty strings.
- COBOL-specific constructs are not supported in Elixir output.


