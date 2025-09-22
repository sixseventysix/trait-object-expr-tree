# Trait-based Mathematical AST

Testing trait objects (`Box<dyn ExprNode>`) to build mathematical expression trees. Each node type (`Const`, `Var`, `Add`, `Mul`) implements the `ExprNode` trait for evaluation and code generation.

## How Trait Object Dispatch Works

`Box<dyn ExprNode>` uses a fat pointer with two components:
- `data_ptr`: Points to the actual object data (`Add`, `Mul`, etc.)
- `vtable_ptr`: Points to a function table for that type

Each concrete type gets its own vtable:
```
VTable_Add = {
    evaluate: Add::evaluate,
    codegen: Add::codegen,
}
```

When you call `expr.evaluate()`:
1. Dereference `vtable_ptr` to get the vtable
2. Jump to `vtable.evaluate` function pointer
3. Pass `data_ptr` as `self`

This enables runtime polymorphism with clean extensibility.
