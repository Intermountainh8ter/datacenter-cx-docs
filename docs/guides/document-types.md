---
title: Document Types - MOP, SOP, EOP, Cx Script, IST
doc_type: guide
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Document Types: Which Procedure Do I Need?

| Type | Full name | Use it when | Written | Typical trigger |
|---|---|---|---|---|
| MOP | Method of Procedure | A planned, one-time or infrequent activity changes the state of critical infrastructure (maintenance, replacement, upgrade, test) | Per activity, reviewed and approved before each use | Approved change ticket and scheduled window |
| SOP | Standard Operating Procedure | A routine, repeatable task is performed the same way every time (rounds, periodic runs, turnover) | Once, then reviewed on a fixed cycle | Schedule or normal operations |
| EOP | Emergency Operating Procedure | An abnormal event needs an immediate, pre-agreed response (utility loss, cooling loss, leak) | In advance, drilled regularly | An alarm or observed condition |
| Cx script | Functional performance test script | A system must be proven against its design intent and sequence of operations before turnover | During commissioning, per system | Completed pre-functional checklists |
| IST script | Integrated systems test script | Multiple systems must be proven working together under load and under failure | Late commissioning, per scenario | All individual system tests passed |

## How they relate

- Commissioning scripts and IST prove the facility does what the design says. Their results become the baseline for operations.
- SOPs keep the facility in its proven state.
- MOPs control any planned departure from that state and return to it.
- EOPs control unplanned departures.
- A good EOP hands off to a MOP for the permanent repair, and every MOP names the EOPs that apply if the work goes wrong.

## What every procedure in this repository contains

1. Document control and approvals
2. Purpose, scope, and risk
3. **Pre-site plan** (see [Pre-Site Plan](pre-site-plan.md))
4. Prerequisites and safety
5. Roles and communications
6. Numbered steps with an expected response for each
7. **Backout procedure** (see [Backout Planning](backout-planning.md))
8. Restoration, verification, and closeout

## Suggested numbering

`<TYPE>-<SYSTEM>-<NNN>-R<rev>`, for example `MOP-UPS-003-R2`. Keep the system codes in one controlled list (ELEC, UPS, GEN, MECH, BMS, FIRE, SEC).
