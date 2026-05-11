# SQLx Examples

Create or refresh README files for each SQLx example first, using a consistent, Copilot-friendly structure that states purpose, prerequisites, and runnable commands. This prepares clean source material for a later central README update.

**Steps**
1. Define a shared README template for examples: brief overview, what the example demonstrates, setup/prerequisites, and command usage. Include one short “Copilot usage” hint so AI agents can quickly infer intent and CLI behavior.
2. Update existing READMEs to match template and fix wording/typos while preserving project-specific details. This applies to `kantoniko`, `sqlx-sqlite-counter`, and `sqlx-sqlite-family-tree`.
3. Create missing READMEs for examples without documentation using code-informed descriptions and concrete commands. This applies to `sqlx-sqlite-migration` and `sqlx-sqlite-todo`. *depends on 1*
4. Normalize command style across all example READMEs (consistent `cargo run` examples, env var usage where required, and expected behavior/output summaries). *parallel with step 2/3 once drafts exist*
5. Quick consistency pass for grammar and formatting so all example READMEs are concise and machine-parsable. *depends on 2-4*
6. Defer central book README changes until example README baseline is complete and reviewed. (Out of scope for this step)

**Relevant files**
- `/home/gabor/github/code-maven/rust.code-maven.com/books/sqlx/examples/sqlx-sqlite-counter/README.md` — align command docs and add explicit behavior notes.
- `/home/gabor/github/code-maven/rust.code-maven.com/books/sqlx/examples/sqlx-sqlite-family-tree/README.md` — fix typos and standardize command descriptions.
- `/home/gabor/github/code-maven/rust.code-maven.com/books/sqlx/examples/sqlx-sqlite-migration/README.md` — new file describing CSV import + migration flow.
- `/home/gabor/github/code-maven/rust.code-maven.com/books/sqlx/examples/sqlx-sqlite-todo/README.md` — new file describing add/done/list workflow and `DATABASE_FILE` requirement.

**Verification**
1. Ensure each example directory contains a README file and no README is empty.
2. Validate command snippets against each example’s `src/main.rs` argument parsing and environment requirements.
3. Spot-check markdown readability and consistent section order across all five READMEs.
4. Optionally run a markdown lint/preview check if available in the workspace.

**Decisions**
- Included scope: only per-example README creation/standardization.
- Excluded scope: central `/home/gabor/github/code-maven/rust.code-maven.com/books/sqlx/README.md` and book chapters for now.
- Assumption: “before creating the central file” means example README work is the immediate priority gate.

