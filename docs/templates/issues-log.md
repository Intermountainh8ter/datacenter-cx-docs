---
title: Cx Issues Log Template
doc_type: template
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Cx Issues Log Template

| ID | Date | System | Level found | Description | Severity | Owner | Root cause | Status | Verification | Retest date | Closed by |
|---|---|---|---|---|---|---|---|---|---|---|---|
| 001 | | | | | | | | Open | | | |

**Severity (suggested, adapt per project):** Critical (blocks test), Major (fix before turnover), Minor (fix before warranty end), Observation.

**Status values:** Open, In progress, Ready for retest, Closed, Deferred (needs owner approval).

## Field definitions (for import into a CMMS or tracking tool)

Keep one row per issue and one value per cell, so the log can be exported as CSV and mapped to fields in a maintenance management system without rework. The field names in the second column are suggestions; match them to the receiving system.

| Column | Field name | Type | Entry |
|---|---|---|---|
| ID | `issue_id` | Text, unique | Project prefix plus a sequence number. Never reused, even after closure. |
| Date | `discovered_on` | Date (YYYY-MM-DD) | Date the issue was found. |
| System | `system` | Text from a fixed list | System and asset tag, using the same names as the asset register (for example `UPS-1`, `GEN-2`). |
| Level found | `discovered_at_level` | 1 to 5 | Commissioning level at which the issue was found (see the [Commissioning Levels One-Pager](../guides/commissioning-levels.md)). |
| Description | `description` | Text | What was observed, not the assumed cause. |
| Severity | `severity` | Critical, Major, Minor, Observation | As defined above. |
| Owner | `owner` | Company and role | Party responsible for the correction. |
| Root cause | `root_cause` | Text | The cause established by investigation. "Unknown" is a valid entry while open, but an issue cannot close with it. |
| Status | `status` | Open, In progress, Ready for retest, Closed, Deferred | As defined above. |
| Verification | `verification` | Text | How the correction was proven: the test or step repeated, its result, and a reference to the completed script or record. |
| Retest date | `retest_on` | Date (YYYY-MM-DD) | Date of the verifying test. |
| Closed by | `closed_by` | Name and role | Person who accepted the verification. Not the person who made the correction. |

## Closure rules

- An issue closes only when **Root cause** and **Verification** are both filled in.
- A retest that passes without a recorded root cause is logged, but the issue stays open until the cause is recorded or the owner's representative accepts the residual risk in writing.
- Deferred issues carry an owner and a target date, and are reviewed at every level gate.
