# M18 — Global Composition Cycle Fidelity

M1 established recursive composition while explicitly requiring cycle rejection. M16 later enforced unique composition-tree ownership globally while intentionally keeping unused declarations legal.

A remaining mismatch exists: cycle detection currently starts only from `screen.root`. As a result, an unreachable authored composition cycle can survive resolution even though cycles are invalid composition topology.

## Principle

Structural validity is global even when structural reachability is optional.

An authored composition may be unused, but if it is declared it must still be internally coherent.

## Required invariant

Every authored composition participates in cycle validation, whether or not it is reachable from `screen.root`.

Reject at least:

- reachable cycles;
- unreachable two-node or longer cycles;
- unreachable self-cycles.

Preserve:

- unused acyclic compositions;
- unused regions;
- M16 ownership rules;
- the explicit root contract;
- existing source syntax and resolved model.

## Boundary

M18 does not require every declaration to be reachable. It does not remove unused declarations, build a graph framework, add parent pointers, alter layout, alter rendering, or introduce reusable components.

The smallest correct implementation is a deterministic validation traversal that covers the entire authored composition set.
