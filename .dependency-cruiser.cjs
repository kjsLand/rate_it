/** Layer rules for the TypeScript UI. See docs/SPEC.md, "Code quality". */
module.exports = {
  forbidden: [
    {
      name: "only-ipc-calls-rust",
      comment:
        "Screens must call Rust through the typed functions in ui/src/ipc/. " +
        "Only ui/src/ipc/ may import @tauri-apps/api or the generated bindings.",
      severity: "error",
      from: { path: "^ui/src", pathNot: "^ui/src/ipc/" },
      to: { path: ["^node_modules/@tauri-apps/api", "^ui/src/bindings\\.ts$"] },
    },
    {
      name: "no-circular",
      severity: "error",
      from: {},
      to: { circular: true },
    },
    {
      name: "no-orphans",
      severity: "warn",
      from: {
        orphan: true,
        pathNot: ["\\.d\\.ts$", "(^|/)tsconfig\\.json$", "^ui/src/main\\.tsx$"],
      },
      to: {},
    },
  ],
  options: {
    doNotFollow: { path: "node_modules" },
    tsConfig: { fileName: "ui/tsconfig.json" },
    tsPreCompilationDeps: true,
  },
};
