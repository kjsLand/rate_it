# anti-slop (vendored)

Source: https://github.com/dmmulroy/anti-slop (MIT), commit `13909c9e07590f5a85d93c62712259fe14a883f7`
(version 0.1.2). It has no npm package; it is meant to be copied in.

What was copied: the upstream `src/` folder, without the `effect/` rules (this app does not use
Effect) and without the `*.test.ts` files. The upstream `LICENSE` is kept in this folder, and
`vendor/eslint-stylistic/` keeps its own license and provenance.

`ui/.oxlintrc.json` loads `tools/anti-slop/index.ts`. `@oxlint/plugins` must be the same exact
version as `oxlint` in `package.json`. Oxlint loads this TypeScript plugin through Node, so Node 24
or later is necessary (see `.nvmrc`).
