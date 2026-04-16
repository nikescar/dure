# Documentation Index

This directory contains documentation for the Dure distributed e-commerce platform.

## 🚀 Start Here

**New to the project?** Read these:
1. **[../CLAUDE.md](../CLAUDE.md)** - Complete project guide (start here!)
2. **[PROJECT_SUMMARY.md](./PROJECT_SUMMARY.md)** - Detailed architecture overview
3. **[QUICK_REFERENCE.md](./QUICK_REFERENCE.md)** - Commands, patterns, and common tasks
4. **[INSTALLING.md](./INSTALLING.md)** - Installation and setup instructions

## Quick Navigation

### Getting Started
- **[INSTALLING.md](./INSTALLING.md)** - Installation and setup instructions
- **[TROUBLESHOOTING.md](./TROUBLESHOOTING.md)** - Common issues and solutions

### Development
- **[GUIDELINES_RUST_CODING.md](./GUIDELINES_RUST_CODING.md)** - Rust coding standards and best practices
- **[GUIDELINES_GIT_COMMITS.md](./GUIDELINES_GIT_COMMITS.md)** - Git commit message conventions
- **[RUST-ENGINEER.md](./RUST-ENGINEER.md)** - Rust engineer agent guide for AI-assisted development

### Reference
- **[CLI_REFERENCE.md](./CLI_REFERENCE.md)** - Command-line interface documentation (Updated for Dure)

### Architecture & Design

- **[ARCHITECTURE.md](./ARCHITECTURE.md)** - System architecture overview (Updated for Dure)
- **[ARTIFACT_LOG_SCHEMA.md](./ARTIFACT_LOG_SCHEMA.md)** - Test artifact logging schema (Updated for Dure)

⚠️ **WARNING**: The following document may contain content from beads_rust and needs review:

- **[AGENT_INTEGRATION.md](./AGENT_INTEGRATION.md)** ⚠️ May contain beads-specific content

### Testing Documentation

⚠️ **WARNING**: These testing docs appear to be from the beads project and may not apply to Dure:

- **[TEST_HARNESS.md](./TEST_HARNESS.md)** ⚠️ Beads-specific testing
- **[E2E_COVERAGE_MATRIX.md](./E2E_COVERAGE_MATRIX.md)** ⚠️ Beads e2e tests
- **[E2E_SYNC_TESTS.md](./E2E_SYNC_TESTS.md)** ⚠️ Beads sync tests
- **[SYNC_SAFETY.md](./SYNC_SAFETY.md)** ⚠️ Beads sync model
- **[SYNC_MAINTENANCE_CHECKLIST.md](./SYNC_MAINTENANCE_CHECKLIST.md)** ⚠️ Beads sync maintenance

### Agent Documentation

The `agent/` subdirectory contains AI agent integration documentation:

- **[agent/QUICKSTART.md](./agent/QUICKSTART.md)** - Quick start for AI agents
- **[agent/SCHEMA.md](./agent/SCHEMA.md)** - Agent schema definitions
- **[agent/EXAMPLES.md](./agent/EXAMPLES.md)** - Example agent interactions
- **[agent/ROBOT_MODE.md](./agent/ROBOT_MODE.md)** - Robot mode documentation
- **[agent/ERRORS.md](./agent/ERRORS.md)** - Error handling for agents

⚠️ These may also be beads-specific and need review.

## Documentation Status

### ✅ Verified/Updated for Dure
- GUIDELINES_RUST_CODING.md
- GUIDELINES_GIT_COMMITS.md
- RUST-ENGINEER.md (generic Rust guidelines)
- **ARCHITECTURE.md** - Updated with Dure architecture
- **CLI_REFERENCE.md** - Updated with Dure commands
- **ARTIFACT_LOG_SCHEMA.md** - Updated for Dure testing

### ⚠️ Needs Review/Update
- AGENT_INTEGRATION.md - May reference beads commands
- All testing documentation - Appears beads-specific
- All sync documentation - Appears beads-specific (may not apply to Dure)
- Agent subdirectory - Unclear relevance to Dure

### 📝 Missing Documentation
- **Dure Architecture** - Need actual Dure system architecture
- **E-commerce Workflows** - Store setup, product management, order processing
- **Payment Integration** - Portone, KakaoPay integration guide
- **Hosting Guide** - Firebase/Supabase setup and deployment
- **API Documentation** - Internal APIs and data models
- **WASM Deployment** - Building and deploying WASM front-end
- **Mobile Build Guide** - Detailed Android build process
- **Security Model** - Identity management, key storage, attestation

## Documentation Priorities

1. **Create Dure Architecture Doc** - Replace ARCHITECTURE.md with actual Dure design
2. **Document E-commerce Features** - Store management, products, orders, payments
3. **Clean Up Beads References** - Remove or update beads-specific docs
4. **Add Deployment Guides** - Firebase, Supabase, WASM hosting
5. **Security Documentation** - Identity, keys, payment security

## For AI Assistants

When working with this codebase:

1. Read **[../CLAUDE.md](../CLAUDE.md)** first for complete project context
2. Check this file (INDEX.md) to know which docs are valid
3. Treat documents marked with ⚠️ as unreliable for Dure development
4. Focus on actual code in `mobile/src/` for understanding current architecture
5. Use PROJECT_SUMMARY.md for detailed architecture
6. Use RUST-ENGINEER.md for Rust development guidelines

## Contributing to Documentation

- Follow Markdown conventions
- Include code examples where applicable
- Update this index when adding new docs
- Remove beads references when updating docs
- Add real Dure examples and workflows
