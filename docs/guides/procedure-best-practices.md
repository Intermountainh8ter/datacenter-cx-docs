---
title: Procedure Writing and Execution Best Practices
doc_type: guide
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Procedure Writing and Execution Best Practices

## Writing

- **One action per step.** If a step contains "and", split it.
- **Every step has an expected response.** "Close breaker" is incomplete. "Close breaker; indicator shows closed, downstream voltage present on all phases" can be verified.
- **Name the device exactly.** Use the asset tag and label text as installed, and state where the operator stands.
- **Record, do not just check.** Ask for the reading, not a tick, wherever a value matters.
- **Mark hold points.** A hold point needs a named person's approval to pass. Put one before every state change that reduces redundancy.
- **Mark the point of no return.** See the backout planning guide.
- **State the risk at each stage.** Say plainly when the load is on a single source or without backup.
- **Write for the person doing it for the first time**, at the end of a long shift.
- **Leave site values as blanks**, filled from the approved design and manufacturer data. Never carry numbers over from another site.
- **Cite the source of every limit.** A limit with no source is a guess.

## Review

- A second qualified person reviews every procedure line by line against current drawings.
- The equipment manufacturer or service vendor reviews steps on their equipment.
- For high-risk work, walk the procedure at the equipment with no operations performed (a dry run).
- Changes after approval go back through approval. No pen-and-ink changes during the window, except through the stop and re-approve process.

## Execution

- **Pre-task briefing** with everyone involved: scope, roles, hazards, hold points, backout, communications.
- **Performer and verifier.** One reads and performs, one confirms the device and the response before and after each action.
- **Three-part communication.** Instruction given, repeated back, confirmed.
- **Follow the steps in order.** If a step cannot be done as written, stop. Do not improvise.
- **Stop-work authority belongs to everyone.**
- **Log actual times and readings** in the step table as you go.
- **One activity at a time** on redundant paths. Never start work on the B side while the A side is abnormal.

## Electrical safety

Requirements for establishing an electrically safe work condition, energized work justification, approach boundaries, and PPE come from the applicable code and the site's electrical safety program: [VERIFY: NFPA 70E sections for electrically safe work condition and energized work permit]. Hazardous energy control requirements: [VERIFY: OSHA 1910.147 lockout/tagout section].

## After the work

- Compare as-left to as-found. Explain every difference.
- Close the change ticket with actual times, deviations, and readings.
- Feed lessons back into the procedure before it is filed for reuse.
- Replenish any spare parts used.
