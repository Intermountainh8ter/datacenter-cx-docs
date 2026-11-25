---
title: Backout Planning Guide
doc_type: guide
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Backout Planning Guide

A backout procedure returns the system to its last known good state when the work cannot continue safely or on time. It is written before the work starts, by the same people, with the same rigor as the forward steps.

## The five parts of a backout procedure

1. **Triggers.** Objective conditions that start the backout. Examples: an expected response is not observed; an unexpected alarm appears on a system outside the scope; the window has less time left than the backout needs; a safety concern is raised by anyone.
2. **Authority.** Who calls it. Anyone can call a stop. One named person (usually the MOP supervisor) decides between resume and backout.
3. **Point of no return (PNR).** The step after which going back is slower or riskier than going forward. Mark it in the step table. Before the PNR the default is back out. After the PNR the default is complete the work, and the procedure must say what "forward recovery" looks like.
4. **Steps.** Numbered, with expected responses, written per section of the forward procedure ("if stopped during steps 5 to 9, do B1 to B4"). Do not write "reverse the above steps".
5. **Verification.** How you prove the system is back to its as-found state: compare against the as-found readings recorded at the start.

## Time budgeting

| Item | Minutes |
|---|---|
| Change window | |
| Forward steps (estimated) | |
| Backout from the latest pre-PNR step (estimated) | |
| Latest time the PNR may be crossed = window end minus remaining forward steps minus contingency | |

If the latest PNR time passes and the PNR has not been crossed, back out.

## Good practice

- Record as-found states (breaker positions, setpoints, modes, alarms, loads) before step 1. The backout target is that record.
- Keep removed parts on site until the system is verified, so they can be refitted.
- Save configuration backups before any setting or firmware change, and test that the backup can be restored.
- Walk the backout during the pre-task briefing, not only the forward path.
- After any backout, hold a review and revise the procedure before the next attempt.

## Backout in emergency procedures

An EOP has no planned forward path to reverse. Its equivalent is a **return to normal** section: the conditions required before leaving the emergency configuration, the controlled steps to do so, and the verification that normal redundancy is restored.
