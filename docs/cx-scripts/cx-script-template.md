---
title: Cx Functional Performance Test Script Template
doc_type: cx-script
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Cx Functional Performance Test (FPT) Script Template

> Template. A script tests the installed system against the approved sequence of operations and design documents. Acceptance values come from those documents and the governing standards, never from this template. A script starts only after the system's [controls point-to-point checkout](../checklists/controls-point-to-point-checkout.md) is accepted (the Level 4 entry gate).

## 1. Document control

| Field | Entry |
|---|---|
| Script ID and revision | |
| System and asset tags | |
| Reference documents (SOO, drawings, submittals, revision) | |
| Author (Cx agent) / date | |
| Reviewed by (design engineer, contractor, vendor) | |
| Test date | |

## 2. Purpose and scope
What functions are proven, in which modes, and what is deferred (for example, seasonal tests).

## 3. Risk and impact
State whether the facility is live. On a live site this script runs under a MOP and a change ticket. State what the test puts at risk and the protection in place.

## 4. Pre-site plan
- [ ] Pre-functional checklists and vendor startup reports closed and reviewed
- [ ] Script reviewed by the contractor and vendor; comments resolved
- [ ] Current sequence of operations and settings obtained; script matches that revision
- [ ] Test equipment listed, calibrated, certificates collected
- [ ] Load banks, temporary power, or heat load arranged and sized
- [ ] Participants confirmed: Cx agent, contractor, vendor, controls integrator, owner's witness
- [ ] Trend logging set up on the points to be recorded
- [ ] Simulation methods agreed for each failure (how each alarm or trip is created safely)
- [ ] Backout reviewed; go/no-go recorded

## 5. Prerequisites (day of test)
- [ ] System in normal automatic operation, no alarms
- [ ] Upstream and downstream systems ready
- [ ] Pre-task briefing held; safety controls in place

## 6. Safety
Hazards and controls for the test. Electrical boundaries and PPE: [VERIFY: NFPA 70E sections for electrically safe work condition and energized work permit].

## 7. Test equipment

| Instrument | ID | Calibration due |
|---|---|---|
| | | |

## 8. Test steps

| # | Action or simulated condition | Expected response (with SOO reference) | Observed | Pass/Fail | Init. |
|---|---|---|---|---|---|
| 1 | Record as-found state and setpoints | Recorded | | | |
| 2 | | | | | |

Rules: record what was observed, not "OK". A failed step is logged in the issues log with an ID and is retested in full after correction.

## 9. Acceptance criteria
List each criterion with its source document and section.

| Criterion | Value | Source document and section |
|---|---|---|
| | | |

## 10. Backout procedure
**Triggers:** unsafe condition; equipment damage risk; response that could affect systems outside the test; loss of a prerequisite.

| If stopped during | Abort steps | End state |
|---|---|---|
| Any simulated failure | Remove the simulation (jumpers, forced points, overrides, test settings) using the override log | Normal automatic operation |
| Load testing | Remove test load in steps; open and lock the load bank breaker | No test load |

Every override, forced point, jumper, and temporary setting is written in an **override log** when applied and signed off when removed. The script is not complete until the log is clear.

## 11. Restoration and sign-off
- [ ] Setpoints and settings returned to approved values and verified against step 1
- [ ] Override log clear; temporary equipment removed
- [ ] Issues logged; retests scheduled

| Role | Name | Signature | Date |
|---|---|---|---|
| Cx agent | | | |
| Contractor | | | |
| Owner's witness | | | |
