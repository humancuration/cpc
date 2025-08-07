# Shtairir Text Grammar (pest skeleton) and IR Mapping

This document defines a high-level pest grammar skeleton for textual Shtairir that maps directly to IR constructs defined in 05-ir.md. The grammar is intentionally minimal; it prefers explicitness and a one-to-one mapping to nodes, ports, edges, params, and subgraphs.

Conventions
- Modules and blocks follow 02-blocks-and-modules.
- Types follow 03-types-and-signatures using TypeSpec strings.
- Version resolution follows 04-versioning-and-compatibility; textual sources may omit versions for unconstrained slots, resolved later by registry/lockfile.

Scope of this skeleton
- It is schematic and omits some whitespace/comment rules and advanced literals.
- The parser will build an AST; a separate lowering step produces IR v1.

Pest Grammar Skeleton (high-level)

program = { SOI ~ ws* ~ use_stmt* ~ ws* ~ decl* ~ ws* ~ EOI }

use_stmt = { "use" ~ ws1 ~ mod_path ~ ws1 ~ "as" ~ ws1 ~ ident ~ ws* ~ ";" }
mod_path = @{ ident ~ ("." ~ ident)* }           // e.g., org.cpc.std
ident    = @{ (ASCII_ALPHANUMERIC | "_")+ }
type_ident = @{ ASCII_ALPHA ~ (ASCII_ALPHANUMERIC | "_")* }  // CamelCase recommended for subgraph names
ws       = _{ (" " | "\t" | "\r" | "\n")+ }
ws1      = _{ (" " | "\t")+ }

decl = _{ subgraph_decl | let_stmt | wire_stmt | output_stmt | if_let_stmt }

subgraph_decl = {
  "fn" ~ ws1 ~ type_ident ~ ws* ~ "(" ~ ws* ~ io_params? ~ ws* ~ ")" ~ ws* ~ "->" ~ ws* ~ io_list ~ ws* ~ block
}
io_params = { io_param ~ (ws* ~ "," ~ ws* ~ io_param)* }
io_param = { ident ~ ws* ~ ":" ~ ws* ~ type_spec }

io_list = { io_item ~ (ws* ~ "," ~ ws* ~ io_item)* }
io_item = { ident ~ ws* ~ ":" ~ ws* ~ type_spec }

type_spec = @{ (!(")" | "," | ";" | "\n" | "{" | "}") ~ ANY)+ } // TypeSpec as in 03-types

block = { "{" ~ ws* ~ decl* ~ ws* ~ "}" }

let_stmt = {
  "let" ~ ws1 ~ ident ~ ws* ~ "=" ~ ws* ~ block_call ~ ws* ~ ";"
}

block_call = {
  alias_ref ~ "." ~ block_path ~ version_opt? ~ ws* ~ "(" ~ ws* ~ arg_list? ~ ws* ~ ")"
}
alias_ref = @{ ident }                    // from `use ... as alias;`
block_path = @{ ident ~ ("." ~ ident)* }  // e.g., math.add
version_opt = @{ "@" ~ semver }?
semver = @{ ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT+ }

arg_list = { named_arg ~ (ws* ~ "," ~ ws* ~ named_arg)* }
named_arg = { ident ~ ws* ~ ":" ~ ws* ~ expr }

wire_stmt = {
  wire_source ~ ws* ~ wire_op ~ ws* ~ wire_target ~ ws* ~ ("[" ~ ws* ~ wire_opts ~ ws* ~ "]")? ~ ws* ~ ";"
}
wire_source = { ref_port }
wire_target = { ref_port }
ref_port = { ident ~ "." ~ ident }        // node_id.port
wire_op = _{ "->" | "~>" }                // data edge vs event edge
wire_opts = { wire_opt ~ (ws* ~ "," ~ ws* ~ wire_opt)* }
wire_opt = _{ "buffer=latest" | ("buffer=queue(" ~ ASCII_DIGIT+ ~ ")") }

output_stmt = {
  "export" ~ ws1 ~ ident ~ ws* ~ "=" ~ ws* ~ ref_port ~ ws* ~ ";"
}

expr = _{ literal | ref_port | struct_lit | list_lit | map_lit }

literal = _{
    bool_lit
  | int_lit
  | decimal_lit
  | string_lit
  | timestamp_lit
}
bool_lit = @{ "true" | "false" }
int_lit = @{ "-"? ~ ASCII_DIGIT+ }
decimal_lit = @{ "-"? ~ ASCII_DIGIT+ ~ "." ~ ASCII_DIGIT+ }
string_lit = @{ "\"" ~ (!"\"" ~ ANY)* ~ "\"" }
timestamp_lit = @{ "\"" ~ ASCII_ALPHANUMERIC+ ~ "Z" ~ "\"" } // simplified RFC3339

list_lit = { "[" ~ ws* ~ (expr ~ (ws* ~ "," ~ ws* ~ expr)*)? ~ ws* ~ "]" }
map_lit = { "{" ~ ws* ~ (map_entry ~ (ws* ~ "," ~ ws* ~ map_entry)*)? ~ ws* ~ "}" }
map_entry = { ident ~ ws* ~ ":" ~ ws* ~ expr }

if_let_stmt = {
  "let" ~ ws1 ~ ident ~ ws* ~ "=" ~ ws* ~ "if" ~ ws* ~ "(" ~ ws* ~ cond_expr ~ ws* ~ ")" ~ ws* ~ block ~ ws* ~ "else" ~ ws* ~ block ~ ws* ~ ";"
}
cond_expr = { ref_port | literal }

Comments (suggested, not formalized above)
- line_comment = _{ "//" ~ (!"\n" ~ ANY)* }
- block_comment = _{ "/*" ~ (!"*/" ~ ANY)* ~ "*/" }

IR Mapping (illustrative snippets)

1) Module Imports
Text:
  use org.cpc.std as std;

IR:
- No nodes. Compiler records alias table: { "std": "org.cpc.std" }.

2) Block Instantiation with Params
Text:
  use org.cpc.std as std;
  let add = std.math.add@1.0.0(a: 1, b: 2);

IR Node:
- instance_id: "add"
- ref: "org.cpc.std/math.add@1.0.0"
- params: { "a": 1, "b": 2 }
- determinism/effects copied from registry block spec
- No edges created yet; params are immutable per node instance.

3) Wiring / Dataflow and Event Channels
Text:
  let now = std.time.now();
  let sum = std.math.add(a: 0, b: 0);
  now.now -> sum.a;
  now.tick ~> sum.trigger;

IR:
- Nodes: n_now, n_sum
- Edges:
  - { source: {node_id:"now", port:"now"}, target:{node_id:"sum", port:"a"}, kind:"data", buffer:"latest" }
  - { source: {node_id:"now", port:"tick"}, target:{node_id:"sum", port:"trigger"}, kind:"event" }

4) Subgraph / Function Definition with Inputs/Outputs
Text:
  fn Accumulate(input: Int) -> (total: Int) {
    let z = std.math.add(a: input, b: 1);
    export total = z.sum;
  }

IR:
- Subgraph "Accumulate" with:
  - inputs: [{ id:"in_input", name:"input", type:"Int", kind:"data" }]
  - outputs: [{ id:"out_total", name:"total", type:"Int", kind:"data" }]
  - consts: [{ const_id:"c1", type:"Int", value:1 }]
  - nodes: [{ instance_id:"z", ref:"org.cpc.std/math.add@1.0.0", params:{} }]
  - edges: in_input.input -> z.a (data), c1.value -> z.b (data), z.sum -> out_total.total (data)

5) Literals and Types
Text:
  let fmt = std.string.format(template: "Hello {name}", values: { name: "Ada" });

IR:
- Node "fmt" ref "org.cpc.std/string.format@1.0.0"
- For inputs modeled as params: stays in node.params.
- For inputs modeled as ports: compiler creates Const for literals and edges into ports.

6) if/merge Construct (sugar)
Text:
  let r = if (cond.val) {
    let a = std.math.add(a: 1, b: 2);
    export out = a.sum;
  } else {
    let b = std.math.add(a: 10, b: 20);
    export out = b.sum;
  };

IR Expansion:
- Subgraphs:
  - "IfThen": inputs: none; outputs: (out:T). Contains node a and export.
  - "IfElse": inputs: none; outputs: (out:T). Contains node b and export.
- Nodes:
  - call_then: Call{ref_subgraph:"IfThen"}
  - call_else: Call{ref_subgraph:"IfElse"}
  - select: std.flow.select(condition: Bool, a:T, b:T) -> out:T
- Edges:
  - cond.val -> select.condition (data)
  - call_then.out -> select.a (data)
  - call_else.out -> select.b (data)
  - select.out -> export r

7) Graph Outputs
Text:
  export sum = add.sum;

IR:
- Graph output binding:
  - outputs: [{ id:"out_sum", name:"sum", type: inferred, kind: from source port }]
  - edge: add.sum -> out_sum.sum

Notes and Validation
- Wire operators:
  - "->" produces an IR Edge with kind:"data".
  - "~>" produces an IR Edge with kind:"event".
- Wire options:
  - buffer=latest (default) or buffer=queue(N) for data edges. Backpressure policy is TODO.
- Identifiers:
  - let names become node instance_ids (snake_case recommended).
  - Subgraph names CamelCase recommended; unique within compilation unit.
- All references are validated against the registry and block specs; types/kinds must match; coercions only when allowed by 03-types.

Example Text → IR Sketch

Text:
  use org.cpc.std as std;
  let now = std.time.now();
  let add = std.math.add(a: 5, b: 0);
  now.now -> add.b;
  export sum = add.sum;

IR:
- Matches example in 05-ir.md; const 5 into add.a, now.now into add.b, export add.sum.

Implementation Hints
- The AST should preserve source spans for nodes and subgraphs to populate node.source.text_span in IR.
- The lowering stage should:
  - Build const pool for literals feeding ports.
  - Apply default param values.
  - Validate required params.
  - Infer output types where necessary using registry-provided signatures.