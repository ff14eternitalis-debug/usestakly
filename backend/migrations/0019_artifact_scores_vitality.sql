-- Persist the new `vitality` dimension on artifact_scores.
--
-- The unique index (external_artifact_id, formula_version) means rows scored
-- under formula v1.1 stay untouched : recompute under v2.0 inserts a *new*
-- row, preserving the audit trail of past MCP responses.
--
-- vitality is nullable because every existing v1.1 row predates the column ;
-- v2.0 rows must populate it.

ALTER TABLE artifact_scores
  ADD COLUMN vitality NUMERIC(4,3);
