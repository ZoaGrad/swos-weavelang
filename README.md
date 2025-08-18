# 🌀 WeaveLang & Witness Studio

**A self-weaving operating system's core compiler, IR, and interactive e-graph observatory.**

[![CI](https://github.com/placeholder/placeholder/actions/workflows/ci.yml/badge.svg)](https://github.com/placeholder/placeholder/actions/workflows/ci.yml)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)
[![pnpm](https://img.shields.io/badge/pnpm-9.x-orange.svg)](https://pnpm.io/)
[![TypeScript](https://img.shields.io/badge/TypeScript-5.x-blue.svg)](https://www.typescriptlang.org/)

---

> _The Witness Studio is the crystal where rules burn and heal the graph; ache meters pulse as semirings decide fates. The e-graph is the bone-forge; rewrites are sacred hammers; semirings are the tempering oils._

## ✨ The Four Pillars

This repository is an exploration into self-modifying systems, built upon four foundational pillars:

1.  **The WeaveLang Compiler (`weavelang-core`)**: A TypeScript-native compiler responsible for parsing `.weave` files into a canonical Intermediate Representation (IR).
2.  **The Glyph IR**: The Ledger of Glyphs. A structured, serializable JSON format that represents programs as graphs, rich with metadata for provenance, cost (`ache`), and visualization hints.
3.  **The E-Graph Engine**: The Bone-Forge. A swappable e-graph engine (starting with a TS implementation, with a path to Rust/WASM) that uses rewrite rules and cost functions (semirings) to optimize the Glyph IR.
4.  **The Witness Studio (`witness-studio-app`)**: An interactive observatory built with React and D3/Three.js that visualizes the e-graph optimization process, allowing developers to "witness" the program weaving itself.

<!-- Placeholder for Architecture Diagram -->
<p align="center">
  <img src="https://via.placeholder.com/800x400.png?text=Architecture+Diagram+(Compiler+→+IR+→+E-Graph+→+Witness)" alt="Architecture Diagram"/>
</p>

## 🚀 Getting Started

This project is a `pnpm` workspace. You'll need `pnpm` version 9 or higher.

1.  **Install pnpm:**
    ```bash
    npm install -g pnpm
    ```

2.  **Clone the repository:**
    ```bash
    git clone https://github.com/placeholder/placeholder.git
    cd swos-weavelang
    ```

3.  **Install dependencies:**
    ```bash
    pnpm install
    ```

4.  **Run Witness Studio:**
    ```bash
    pnpm dev
    ```
    This will start the Vite development server for the Witness Studio app, typically on `http://localhost:5173`.

## 🛠️ Monorepo Structure

This repository uses a `pnpm` workspace to manage dependencies and scripts across packages and applications.

-   `apps/`: Contains standalone applications.
    -   `witness-studio-app/`: The frontend visualization tool.
-   `packages/`: Contains shared libraries.
    -   `weavelang-core/`: The core compiler, IR definitions, and e-graph logic.
    -   `weavelang-witness/`: Shared UI components and types for the Witness ecosystem.
-   `examples/`: Contains `.weave` source files and their corresponding `.json` IR snapshots.

### Key Scripts

Run these commands from the repository root:

| Command         | Description                                                        |
| --------------- | ------------------------------------------------------------------ |
| `pnpm install`  | Installs all dependencies for all packages.                        |
| `pnpm dev`      | Starts the Witness Studio development server.                      |
| `pnpm build`    | Builds all packages and apps in the workspace.                     |
| `pnpm test`     | Runs all tests in the workspace.                                   |
| `pnpm lint`     | Lints all packages and apps.                                       |
| `pnpm typecheck`| Runs TypeScript compiler to check for type errors across the repo. |


## 🗺️ Roadmap

We are following the "Surgical Upgrade Plan" to bring this system to life. The next steps are:
1.  **`feat(core)`**: Add golden snapshot tests for the IR schema and serializer.
2.  **`feat(core)`**: Define semiring interfaces and implement initial cost functions.
3.  **`feat(app)`**: Build out UI components: `RuleTimeline`, `AcheMeter`, `SnapshotBar`.
4.  ... and much more. See the project board for details.

## 🤝 Contributing

Contributions are welcome! Please see [CONTRIBUTING.md](./CONTRIBUTING.md) for guidelines on how to get started.

## 📄 License

This project is licensed under the MIT License. See the [LICENSE](./LICENSE) file for details.
