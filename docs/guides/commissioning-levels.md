---
title: Commissioning Levels One-Pager
doc_type: one-pager
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Commissioning Levels at a Glance

Many data center projects organize commissioning into five levels. This is an industry convention, not a single published standard, and the exact definitions vary by owner and project. Always use the definitions in the project's commissioning plan.

| Level | Common name | What happens | Evidence produced | Documents in this repository |
|---|---|---|---|---|
| 1 | Factory witness testing | Equipment is tested at the factory before shipment | Factory test reports, punch list | [FAT / FWT Witness Checklist](../checklists/fat-fwt-witness-checklist.md) |
| 2 | Delivery and installation verification | Equipment is received, inspected, and checked as installed | Receiving inspections, installation checklists | [Pre-Functional Checklist](../checklists/pre-functional-checklist.md), sections A and B |
| 3 | Pre-functional and startup | Each unit is energized and started by the vendor or installer; controls points are checked end to end | Startup reports, pre-functional checklists, point-to-point sheets | [Pre-Functional Checklist](../checklists/pre-functional-checklist.md), sections C to E; [Controls Point-to-Point Checkout](../checklists/controls-point-to-point-checkout.md) |
| 4 | Functional performance testing | Each system is tested against its sequence of operations | Completed Cx scripts, issues log | [Cx Script Template](../cx-scripts/cx-script-template.md) and examples; [test plans](../test-plans/generator-paralleling-test-plan.md) |
| 5 | Integrated systems testing | All systems are tested together under load and failure scenarios | IST report, final issues log | [IST Plan](../ist/integrated-systems-test-plan.md), [IST Script Template](../templates/ist-script-template.md), [IST Readiness Checklist](../ist/ist-readiness-checklist.md), [Level 5 Verification Checklist](../checklists/level-5-verification-checklist.md) |

## Gates between levels

A gate is a recorded decision, by a named approver, that a level is complete enough for the next to start. The typical entry requirements are below; the project's commissioning plan sets the actual ones.

| Gate | Entry to the next level requires | Evidence at the gate |
|---|---|---|
| 1 to 2 (ship) | Witnessed factory tests passed; "before shipment" punch items closed | Signed punch list and release to ship |
| 2 to 3 (energize) | Installation complete and inspected; equipment matches the approved submittal | Closed installation sections of the pre-functional checklist |
| 3 to 4 (functional test) | Vendor startup complete; pre-functional checklist closed; **controls point-to-point checkout accepted** for every point the script uses | Startup reports, closed checklists, accepted point-to-point sheets |
| 4 to 5 (integrated test) | Functional tests passed for every system in the IST scenarios; issues affecting IST closed or accepted in writing | Completed scripts, issues log, IST readiness go/no-go |
| 5 to turnover | IST report accepted after owner verification; open items each have an owner, date, and written acceptance | Level 5 verification checklist, final issues log |

## Rules for every gate

- A level does not start until the previous level's open issues are closed or accepted in writing.
- Each gate has a named approver, and the approval is recorded with its date.
- Deferred tests (for example, seasonal tests) are logged with an owner and a date.
- A failed test is retested only after its root cause is recorded in the [Cx Issues Log](../templates/issues-log.md). A pass on retry with no known cause does not close the issue.
- A correction that changes a setting, sequence, or component sends the affected system back to the level where that item was first tested.
- Tests run at reduced load, or without the required witness, are reported as such, never merged with full results.
- Every override, forced point, and jumper is cleared before the gate is approved.

See also: the [Cx Phases One-Pager](../one-pagers/cx-phases.md), which maps these levels onto the project phases.
