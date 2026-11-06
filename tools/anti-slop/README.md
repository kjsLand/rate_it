# anti-slop (vendored)

Source: https://github.com/dmmulroy/anti-slop (MIT). It has no npm package; it is meant to be copied in.

Not vendored yet. To add it, either:

1. Run the install skill with your coding agent:
   `npx skills add dmmulroy/anti-slop --skill install-anti-slop`
2. Or copy the repository's `src/` folder into this folder, keep its LICENSE file,
   and install `@oxlint/plugins` at the same version as `oxlint`.

`ui/.oxlintrc.json` loads `tools/anti-slop/index.ts` and turns on all generic rules as errors.
After vendoring, check that the plugin path and the rule prefix (`anti-slop/`) match what the
install skill writes. Do not turn on the Effect rules; this app does not use Effect.
