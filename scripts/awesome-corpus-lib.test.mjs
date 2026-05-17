import test from "node:test";
import assert from "node:assert/strict";
import { normalizeGithubRepo } from "./awesome-corpus-lib.mjs";

test("normalizes tree and blob links to owner/repo", () => {
  const tree = normalizeGithubRepo("https://github.com/facebook/react/tree/main/packages/react");
  assert.equal(tree.key, "facebook/react");
  const blob = normalizeGithubRepo("https://github.com/vuejs/core/blob/main/README.md");
  assert.equal(blob.key, "vuejs/core");
});

test("rejects issue and pull links", () => {
  assert.equal(
    normalizeGithubRepo("https://github.com/o/r/issues/1").reject,
    "non_repo_path",
  );
});
