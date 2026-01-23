extend the generator to include ScaffolderError too

enforce that every error has an E‑code

engine.rs
A zero‑clone version using Arc<str>

A span‑aware parser that returns multiple labels

A multi‑error diagnostic for multiple empty segments

A builder‑style API for constructing diagnostics

A git diff patch you can apply directly



the refactored DSL macro that emits spans

a full end‑to‑end example showing Ariadne highlighting a DSL error

a diagnostic enum that uses these spans

a snapshot test harness for diagnostics

real DSL spans into your diagnostics

real source snippets into Ariadne

snapshot tests for diagnostics

integrate this with your global EngineError type

unify ScaffolderError + EngineError

wire spans from your DSL into these diagnostics

snapshot Ariadne output in your test harness

    rewriting the snapshots

    generating the new test config factory

    writing the new engine macro

    updating the examples

    adding DSL validation

    cleaning up dead code

    Pruning leftover boilerplate

Once the migration stabilizes, the fastest wins usually come from eliminating structural noise that no longer serves the new architecture. These are the areas that typically yield the biggest clarity gains:
🌿 Low‑hanging cleanup targets

    Redundant phase structs: structs that used to carry fields for the old DSL but now simply mirror the real Config model.

    Phantom generics or unused markers: leftover type parameters that were only needed for the legacy builder pattern.

    Duplicated field propagation code: especially in transitions like PhaseA -> PhaseB where the new DSL already guarantees correctness.

    Old helper modules: utilities for parsing, merging, or validation that the new typed phases make obsolete.

    Legacy error enums: variants that can be collapsed now that the DSL has deterministic structure.

    Snapshot scaffolding artifacts: test helpers or fixtures that were only needed to support the old DSL’s quirks.

🧹 High‑impact structural pruning

    Collapse multi-step builders into single, typed constructors when the new DSL already enforces invariants.

    Remove transitional compatibility layers that were only needed while both DSLs coexisted.

    Inline trivial macros that no longer justify their existence now that the DSL is declarative.

    Delete “just in case” fields that were added during the migration but never actually used by the real config model.

This is the moment where the codebase gets lighter, more intentional, and more readable for future contributors.
🧭 Future‑proofing the DSL phases

Now that the DSL is stable, the next step is designing it so it stays stable — even as new features, modules, or config fields appear.
🛡️ Core strategies for long-term resilience

    Phase boundaries as contracts: treat each phase struct as a stable API surface, not an implementation detail.

    Forward‑compatible field propagation: ensure every new config field has a single, predictable path through all phases.

    Minimal phase responsibilities: keep each phase focused on one conceptual transformation to avoid future entanglement.

    Extensible macro entrypoints: design macros so adding new DSL keywords doesn’t require rewriting existing ones.

    Schema‑driven generation: consider generating parts of the DSL from a canonical config schema to avoid drift.

    Stable serialization boundaries: ensure the final Config output is the only place where serialization rules live.

    Feature‑gated expansions: allow new DSL capabilities to be introduced behind feature flags without breaking old configs.

🚀 Architectural patterns that age well

    Enum‑based phase transitions: makes it explicit which transitions are legal and prevents accidental bypasses.

    Trait‑based phase capabilities: lets you add new behaviors without modifying existing phase structs.

    Macro‑generated field propagation: ensures new fields automatically flow through phases with zero boilerplate.

    Declarative validation layers: keep validation rules out of the phases themselves so they can evolve independently.

If you want, we can go deeper into either direction — for example:

    designing a macro to auto‑propagate fields across phases

    creating a checklist for safe boilerplate removal

    mapping out a long‑term DSL evolution roadmap
