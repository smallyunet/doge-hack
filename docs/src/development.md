# Development

## Repo layout

- `src/`: library + CLI
- `tests/`: integration tests for CLI
- `docs/`: mdBook source

## Docs deployment

This repo is set up to deploy docs to GitHub Pages via a workflow.

After enabling GitHub Pages in repository settings, pushes to `main` will publish the mdBook output.
