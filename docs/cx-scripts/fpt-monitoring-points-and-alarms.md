---
title: Cx Script - Monitoring System Points and Alarms
doc_type: cx-script
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Cx Script: Monitoring System (BMS/EPMS) Points and Alarms

> Example script for verifying a building or power monitoring system. The approved points list is the test basis.

## 1. Document control

| Field | Entry |
|---|---|
| Script ID and revision | |
| System (BMS, EPMS, or both) and server | |
| Software and controller firmware versions | |
| Points list, alarm matrix, and graphics revisions | |
| Configuration backup reference | |
| Sample size and selection method (if sampling) | |
| Author (Cx agent) / date | |
| Reviewed by (controls integrator, design engineer, owner's operations) | |
| Test date | |

## 2. Purpose and scope
Prove that every point reads true from the field device to the operator screen, every alarm reaches the right people with the right priority, and the system behaves safely when it fails.

## 3. Risk and impact
Forced points and simulated signals can cause real equipment to act. On a live site this script runs under a MOP.

## 4. Pre-site plan
- [ ] Approved points list, alarm matrix (priority, delay, recipients), and graphics at current revision
- [ ] Controls point-to-point checkout accepted for the points under test: this is the Level 4 entry gate (see the [Controls Point-to-Point Checkout Sheet](../checklists/controls-point-to-point-checkout.md)). This script samples and confirms that work; it does not replace it
- [ ] Sample size agreed if not testing every point (critical points are always tested in full)
- [ ] Simulation tools arranged: signal simulator, decade box, test software access
- [ ] User accounts for testers created with the right access level
- [ ] Notification recipients warned that test alarms will be sent
- [ ] Backup of the system configuration taken before testing
- [ ] Backout reviewed; go/no-go recorded

## 5. Prerequisites (day of test)
- [ ] System online, all controllers communicating, time synchronized
- [ ] Field devices installed and calibrated

## 6. Safety
Work at field devices follows the hazards of that equipment. No forcing of outputs to equipment without the equipment owner present.

## 7. Test equipment

| Instrument | ID | Calibration due |
|---|---|---|
| Signal simulator | | |
| Decade box | | |
| Reference instruments (temperature, pressure, electrical) | | |
| Laptop with read access and test software | | |

## 8. Test steps

| # | Action or simulated condition | Expected response | Observed | Pass/Fail | Init. |
|---|---|---|---|---|---|
| 1 | Record as-found: controller list, firmware, configuration backup reference | Recorded | | | |
| 2 | Analog inputs: compare each with a reference instrument, or inject low, mid, and high values | Screen value within tolerance; correct units and tag | | | |
| 3 | Digital inputs: change state at the field device | Screen state changes; correct sense (not inverted) | | | |
| 4 | Outputs: command from the screen with the equipment owner present | Field device responds; feedback matches | | | |
| 5 | Communication points (meters, UPS, generators): compare screen values with the device display | Match | | | |
| 6 | Alarms: drive each tested point past its limit | Alarm after the set delay, correct text, priority, and recipients | | | |
| 7 | Alarm return to normal and acknowledgment | Logged with user and time | | | |
| 8 | Trends: confirm critical points are logged at the set interval and can be exported | Data present | | | |
| 9 | Graphics: confirm each screen matches the installed system and drawings | Match | | | |
| 10 | Disconnect one controller from the network | Communication alarm; equipment continues on local control | | | |
| 11 | Remove power from one controller, then restore | Restarts to the correct state without manual action | | | |
| 12 | Fail the server or front end | Controllers continue; data backfilled on return if designed | | | |
| 13 | Check time synchronization across controllers and server | Consistent timestamps | | | |
| 14 | Check user access levels | Each role can do only what it should | | | |
| 15 | Record as-left: points in override (expected: none), alarm delays and limits, configuration backup reference; compare with step 1 | No overrides; values approved | | | |

Rules: record what was observed, not "OK". A failed step is logged in the [Cx Issues Log](../templates/issues-log.md) with an ID and is retested in full after the root cause is found and corrected.

## 9. Acceptance criteria
The approved points list, alarm matrix, and specification. Record the pass rate and every failed point by tag.

| Criterion | Value | Source document and section |
|---|---|---|
| Points tested and pass rate (critical points: all tested) | | |
| Analog accuracy against the reference | | |
| Alarm delay, text, priority, and recipients per the alarm matrix | | |
| Controller restart without manual action | | |
| Time synchronization across controllers and server | | |

## 10. Backout procedure
**Triggers:** a forced or simulated point causes an unintended equipment action; loss of control of live equipment; configuration corruption.

| If stopped during | Abort steps | End state |
|---|---|---|
| Any point test | Remove the simulation or force at once; confirm the point returns to its live value | Live values |
| Steps 10 to 12 | Reconnect or restore power; confirm communication | All controllers online |
| Configuration damaged | Restore the backup taken in the pre-site plan; verify against step 1 | As-found configuration |

Every force and override is entered in the override log and cleared before sign-off. Run the system's "points in override" report as the final check.

## 11. Restoration and sign-off
- [ ] No points in override; alarm delays and limits at approved values
- [ ] Test alarms cleared; recipients told that testing is complete
- [ ] Failed points logged in the issues log

| Role | Name | Signature | Date |
|---|---|---|---|
| Cx agent | | | |
| Controls integrator | | | |
| Owner's operations representative | | | |
| Owner's witness | | | |
