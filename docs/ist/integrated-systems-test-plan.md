---
title: Integrated Systems Test Plan Outline
doc_type: ist
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Integrated Systems Test (IST) Plan Outline

> Skeleton for the IST of a generic facility, for example a 1.5 MW modular hall or a 30-unit fuel-cell plant. Fill in per project.

## 1. Purpose and scope
Prove that power, cooling, controls, and life-safety systems work together at design load and through failure scenarios. State the systems under test, the systems excluded, and the state of the facility at the start.

## 2. Risk and impact
IST deliberately creates failures. State the protections: no production load, load banks only, stop authority, fire watch if detection is impaired, and limits that end a test.

## 3. Pre-site plan
- [ ] All functional performance tests passed; open issues accepted in writing (see the readiness checklist)
- [ ] Scenario list and order agreed with the owner and design engineer
- [ ] A script written for each scenario, each with its own backout
- [ ] Load bank plan: quantity, placement by cabinet or zone, power connections, and control (remote shut-off)
- [ ] Trend logging and power quality recording set up; time synchronized across systems
- [ ] Fuel and water quantities for the full test program
- [ ] Vendors confirmed on site or on call for each system
- [ ] Authorities, utility, and neighbors notified where required
- [ ] Daily schedule with recovery time between scenarios (battery recharge, temperature recovery)
- [ ] Go/no-go meeting held and recorded

## 4. Roles

| Role | Responsibility |
|---|---|
| Test director | Runs the script; single voice for commands |
| Owner's representative | Witness; accepts results |
| Stop authority | Named person who ends a test; anyone may call stop |
| Recorders | One per system: electrical, mechanical, controls |
| Vendors | Respond to their equipment |
| Safety lead | Controls the area and hazards |

## 5. Scenario list (example)

| # | Scenario | Expected response |
|---|---|---|
| 1 | Steady state at design load | All systems stable within limits for the planned duration |
| 2 | Loss of utility | UPS carries load; generators start and accept load; cooling restarts |
| 3 | Return of utility | Retransfer per sequence; no load loss |
| 4 | Loss of one generator while on generator | Remaining units carry the load |
| 5 | Loss of one UPS module or one UPS system | Load continues on redundant capacity |
| 6 | Loss of one distribution path | Dual-fed loads continue on the other path |
| 7 | Loss of one cooling unit, pump, or chiller | Standby starts; temperatures within limits |
| 8 | Total loss of cooling (heat-rise test) | Time-to-limit recorded for operations |
| 9 | Loss of the control system or its network | Equipment continues on local control |
| 10 | Fire alarm interfaces | Dampers, shutdowns, and signals per the design |

## 6. Acceptance criteria
From the owner's project requirements and design basis. Standards-based limits: [VERIFY: NFPA 110 acceptance test section], [VERIFY: ASHRAE thermal guidelines section for allowable ranges].

## 7. Data to record
Timestamps for each event, electrical and thermal trends, alarm logs, and observer notes. Do not include site names or identifiers in files published from this template.

## 8. Backout procedure
**Program-level triggers:** injury or unsafe condition; equipment damage; a temperature or electrical limit reached; loss of a system that the next scenario depends on.

| Situation | Action | End state |
|---|---|---|
| Stop called during a scenario | Test director orders the scenario's own backout; load banks are reduced or shed first if heat or power is the concern | Normal lineup, load banks at a safe level |
| System fails and cannot be restored the same day | Suspend the program; record the state; secure load banks; hand the facility to the contractor in a safe state | Safe state, program suspended |
| Resume | Only after the cause is corrected, the affected functional test is repeated, and a new go/no-go is recorded | Program restarts at the failed scenario |

## 9. Deficiency handling and closeout
Log each issue in the Cx issues log with a scenario reference; retest the full scenario after correction. Final report: results per scenario, trends, open items, and the operations baseline (time-to-limit, start times, transfer times).
