# Project Intelligence & Operational Logic

This file is the operational core. Gemini CLI MUST follow these protocols to maintain project integrity.

## 🎯 Architectural Intent
- **Core Mission:** Chronos: The Graph-based AI Cognition Layer with Formal Proofs.
- **Primary Stack:** Rust (Axum), Cntm-Graph, BlowTime, Mojo (CUDA), Lean Proof Assistant.
- **System Nature:** The orchestration layer that fuses symbolic math with neural networks to provide AI with verifiable memory and reasoning.

## 🧬 Automated Lifecycle Management
1. **Research Sync:** When `./scripts/update_notebookLM.sh` is executed:
   - You MUST update `DESIGN_DECISIONS.md` with new ADRs found in research.
   - **Constraint:** Maintain a rolling log of the **latest 10 ADRs**.
2. **Logic Verification:**
   - All new graph mutations and cognitive kernels MUST pass Lean verification before commit.
   - Run `lean verify src/logic` if available.
3. **PR Creation Protocol:** When instructed to create a Pull Request:
   - **Summarize:** Analyze all commit messages since the last merge to `main`.
   - **Template:** Read `.github/PULL_REQUEST_TEMPLATE.md` and populate it.
   - **Assign:** Automatically set the current developer as the Assignee.
4. **Pre-Commit Action:** Before every commit, you MUST:
   - Run `tree -a -I 'node_modules|.git|target' > STRUCTURE.tree`.
   - Trigger stack-specific formatting (`cargo fmt`, `mojo format`).
   - Run `pre-commit run --all-files`.

## 🛠️ Tooling & Standards
- **Translation:** All technical specifications are English. `locales/` MUST be kept in sync.
- **Workflow Mastery:** Use `/superpower:executing-plans` for feature work.
- **Security:** Security level is High (Mission Critical). All security protocols must be formally verified.

## 📂 Template Inventory
You manage: ARCHITECTURE.md, ROADMAP.md, CONTRIBUTING.md, DESIGN_DECISIONS.md, STRUCTURE.tree, SECURITY.md, LICENSE.md, FAQ.md, GOVERNANCE.md, SUPPORT.md, TROUBLESHOOTING.md, PHILOSOPHY.md, MANIFESTO.md, and `locales/README.{th,ja,zh}.md`.
