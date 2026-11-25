---
title: IST Script Template
doc_type: ist
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# IST Script Template

> Template for one integrated systems test (Level 5) scenario. Copy once per scenario in the [IST Plan Outline](../ist/integrated-systems-test-plan.md). For a worked example, see the [IST Script: Loss and Return of Utility](../ist/ist-script-loss-of-utility.md). Expected results and acceptance criteria come from the approved sequence of operations, the design basis, and the governing standards, never from this template.

## 1. Document control

| Field | Entry |
|---|---|
| Script ID and revision | |
| Scenario number and name (from the IST plan) | |
| IST plan reference and revision | |
| Sequence of operations and settings revision | |
| Author / date | |
| Reviewed by (design engineer, contractors, vendors) | |
| Test director | |
| Planned date and start time | |

## 2. Purpose and scope
State what this scenario proves, which systems take part, the load at which it runs, and anything excluded.

## 3. Risk and impact
State the starting state (load banks only, no production load, unless stated otherwise), what the scenario deliberately fails, the worst credible outcome, and the protections in place. Name the limit that ends the test (for example, a hall temperature or a battery state): ____

## 4. Pre-site plan
- [ ] Level 4 functional tests passed for every system in this scenario; open issues accepted in writing
- [ ] [IST Readiness Checklist](../ist/ist-readiness-checklist.md) complete and go/no-go recorded
- [ ] Script reviewed by the design engineer, contractors, and vendors; comments resolved
- [ ] Simulation method for each failure agreed (which breaker, signal, or valve, operated by whom)
- [ ] Load bank quantity, placement, and setting agreed; remote shut-off tested
- [ ] Recorders assigned to each system and location; instruments calibrated; clocks synchronized
- [ ] Trend logging and power quality recording set up on the points this scenario relies on
- [ ] Fuel, water, and battery state sufficient for the scenario and its backout
- [ ] Backout walked through at the pre-task briefing

## 5. Prerequisites (day of test)
- [ ] All systems in automatic, normal lineup, no active alarms
- [ ] Steady state at the planned load held for the planned period
- [ ] Every recorder in position with communications checked
- [ ] Override log open and empty

## 6. Safety
Hazards and controls for this scenario. Electrical boundaries and PPE: [VERIFY: NFPA 70E sections for electrically safe work condition and energized work permit]. Only named operators perform switching. Anyone may call stop.

## 7. Test equipment

| Instrument | ID | Calibration due | Location / system recorded |
|---|---|---|---|
| | | | |

## 8. Test steps

Each step states the expected result and the acceptance criterion it is judged against, with the source. Record what was observed, with the time, not "OK". Hold points (H) require the test director to poll every recorder before continuing.

| Step | Action or simulated condition | Expected result | Acceptance criterion (with source) | Observed / time | Pass/Fail | Init. |
|---|---|---|---|---|---|---|
| 1 | Record as-found state: lineup, loads, temperatures, fuel, battery state, active alarms | Recorded; normal lineup | Matches the starting state in section 3 | | | |
| H1 | **HOLD POINT**: test director polls every recorder for ready | All ready | All recorders confirm | | | |
| 2 | | | | | | |
| 3 | | | | | | |
| H2 | **HOLD POINT**: test director polls for ready to restore | All ready | All recorders confirm | | | |
| 4 | | | | | | |
| n | Verify alarms and events at the monitoring system match what happened, in order | Complete and correctly timed | Every event in the script appears with the correct time | | | |
| n+1 | Record as-left state and compare with step 1 | Normal lineup | Matches step 1 | | | |

Rules: a failed step is logged in the [Cx Issues Log](cx-issues-log-template.md) with an ID. The scenario is rerun in full after the root cause is found and corrected, and after any affected Level 4 test is repeated.

## 9. Acceptance criteria summary

| Criterion | Value | Source document and section |
|---|---|---|
| | | |

## 10. Backout procedure
**Triggers:** a limit in section 3 reached; equipment fault or damage risk; a system outside the scenario affected; loss of a prerequisite; unsafe condition.

| If stopped during steps | Backout steps | End state |
|---|---|---|
| | B1: | |
| | B2: | |

After any backout: record the state, log the issue, clear the override log, and do not rerun until the cause is corrected.

## 11. Restoration and sign-off
- [ ] Normal lineup restored; batteries recharging; fuel level recorded
- [ ] Override log clear; temporary equipment removed or made safe
- [ ] Data collected from all recorders, instruments, and the monitoring system
- [ ] Issues logged with this script's ID

| Role | Name | Signature | Date |
|---|---|---|---|
| Test director | | | |
| Owner's representative | | | |
| Cx agent | | | |
