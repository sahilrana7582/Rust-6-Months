# Rust-6-Months

> A 6-month deep dive into Rust systems programming, from lifetimes and trait systems to lock-free data structures, async runtimes, storage engines, distributed systems, and operating systems.

## Mission

This repository documents my journey through a self-imposed **"God of War" Rust Mastery Roadmap**.

The objective is not merely to learn Rust syntax, but to develop a deep understanding of:

* Ownership & Borrowing
* Lifetimes & Variance
* Traits, GATs & Object Safety
* Macros (Declarative + Procedural)
* Unsafe Rust & Memory Layout
* Atomics & Memory Ordering
* Lock-Free Programming
* Async Internals & Executors
* Networking & Protocol Design
* Interpreters & Compilers
* Storage Engines
* Distributed Systems
* Bare-Metal Development

By the end of the journey, I aim to build multiple production-grade systems entirely from scratch and understand the reasoning behind Rust's design decisions.

---

## Roadmap Overview

| Cycle | Focus                                              |
| ----- | -------------------------------------------------- |
| C1    | Lifetimes, Variance, Zero-Copy Parsing             |
| C2    | Traits, GATs, Object Safety, Macros                |
| C3    | Generics, Const Generics, Type-State, Regex Engine |
| C4    | Procedural Macros                                  |
| C5    | Unsafe Rust, Memory Layout, Miri                   |
| C6    | Atomics, Threads, Channels                         |
| C7    | Lock-Free Programming, Loom                        |
| C8    | Futures, Pin, Wakers, Executors                    |
| C9    | Async Runtime (Mini Tokio)                         |
| C10   | Networking & Protocols                             |
| C11   | Interpreters                                       |
| C12   | Compilers & Garbage Collection                     |
| C13   | Storage Engines (LSM Tree)                         |
| C14   | Distributed Systems & Raft                         |
| C15   | Capstone (OS Kernel / Distributed Database)        |

---

## Repository Structure

```text
rust-6-months/
│
├── cycle-01/
├── cycle-02/
├── cycle-03/
├── cycle-04/
├── cycle-05/
├── cycle-06/
├── cycle-07/
├── cycle-08/
├── cycle-09/
├── cycle-10/
├── cycle-11/
├── cycle-12/
├── cycle-13/
├── cycle-14/
├── cycle-15/
│
├── notes/
├── resources/
├── daily-logs/
│
└── README.md
```

---

## Daily Workflow

Every day consists of:

### 1. Study

Reading books, articles, RFCs, source code, and documentation.

### 2. Build

Implementing projects from scratch with minimal reliance on existing crates.

### 3. Drill

Solving focused exercises to reinforce concepts.

### 4. Log

Writing a short "Today I Learned" note.

---

## Learning Rules

### Rule #1 — Understand Before Memorizing

Every feature must be understood from first principles.

If I cannot explain it, I do not know it.

---

### Rule #2 — Build Everything

Concepts are not considered learned until implemented.

Examples:

* Build my own `Vec`
* Build my own `Arc`
* Build my own Executor
* Build my own HTTP Server
* Build my own Storage Engine
* Build my own Consensus Algorithm

---

### Rule #3 — Read the Compiler

Compiler errors are learning opportunities.

Before asking for help:

1. Read the error.
2. Explain the error in my own words.
3. Attempt a fix.
4. Then seek guidance if needed.

---

### Rule #4 — Unsafe Must Be Proven Correct

Every unsafe block must have:

* Safety comments
* Miri validation
* Clear invariants

No exceptions.

---

### Rule #5 — Concurrency Must Be Verified

Every concurrent primitive must:

* Have tests
* Have stress tests
* Have Loom verification where applicable

---

## Progress Tracker

### Cycle 1 — Lifetimes, Variance & Type System

* [ ] Day 1
* [ ] Day 2
* [ ] Day 3
* [ ] Day 4
* [ ] Day 5
* [ ] Day 6
* [ ] Day 7

### Cycle 2 — Traits & Macros

* [ ] Day 1
* [ ] Day 2
* [ ] Day 3
* [ ] Day 4
* [ ] Day 5
* [ ] Day 6
* [ ] Day 7

### Cycle 3 — Generics & Regex Engine

* [ ] Completed

### Cycle 4 — Procedural Macros

* [ ] Completed

### Cycle 5 — Unsafe Rust

* [ ] Completed

### Cycle 6 — Atomics & Concurrency

* [ ] Completed

### Cycle 7 — Lock-Free Programming

* [ ] Completed

### Cycle 8 — Async Fundamentals

* [ ] Completed

### Cycle 9 — Async Runtime

* [ ] Completed

### Cycle 10 — Networking

* [ ] Completed

### Cycle 11 — Interpreters

* [ ] Completed

### Cycle 12 — Compilers & GC

* [ ] Completed

### Cycle 13 — Storage Engines

* [ ] Completed

### Cycle 14 — Distributed Systems

* [ ] Completed

### Cycle 15 — Capstone

* [ ] Completed

---

## Core Resources

### Books

* Rust for Rustaceans — Jon Gjengset
* Programming Rust (2nd Edition)
* Rust Atomics and Locks — Mara Bos
* The Rustonomicon
* Asynchronous Programming in Rust
* Crafting Interpreters

### Projects

* Mini LSM
* Proc Macro Workshop
* Mini Redis
* Too Many Linked Lists
* Talent Plan
* Writing an OS in Rust

---

## Success Criteria

At the end of this journey I should be able to:

* Reason about lifetimes without trial-and-error.
* Explain variance and HRTBs from memory.
* Write safe abstractions over unsafe code.
* Build lock-free data structures.
* Implement an async runtime.
* Design network protocols.
* Build storage engines.
* Implement distributed consensus.
* Read and contribute to advanced Rust codebases.
* Ship meaningful open-source contributions.

---

## Motto

> “The compiler is not my enemy.
> The compiler is the teacher that never gets tired.”
