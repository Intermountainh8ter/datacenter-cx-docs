---
title: Cx Script - Fire Alarm and Suppression Interface Test
doc_type: cx-script
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Cx Script: Fire Alarm and Suppression Interface Test

> Example script for the interfaces between the fire systems and the rest of the facility. It does not replace the fire system's own acceptance test by the installer and the authority having jurisdiction.

## 1. Document control

| Field | Entry |
|---|---|
| Script ID and revision | |
| Fire alarm panel and suppression systems (asset tags) and zones under test | |
| Fire panel firmware or program version | |
| Cause-and-effect matrix revision | |
| Reference documents (fire alarm drawings, sequences of operation for interfaced systems, revision) | |
| Author (Cx agent) / date | |
| Reviewed by (fire system vendor, design engineer, mechanical and controls contractors) | |
| Fire watch arranged (name and contact) | |
| Test date | |

## 2. Purpose and scope
Prove that each fire system output causes the right response in the other systems, and only that response: air handling shutdown, damper closure, door release, power shutdown where designed, and signals to monitoring.

## 3. Risk and impact
**Accidental suppression discharge and unintended power shutdown are the main risks.** On a live site this script runs under a MOP.

## 4. Pre-site plan
- [ ] Fire system acceptance test by the installer complete and accepted: [VERIFY: NFPA 72 sections on acceptance testing]
- [ ] Approved cause-and-effect matrix (input, output, expected action) at current revision; this script follows it line by line
- [ ] Fire system vendor attending; only the vendor operates the fire panel
- [ ] **Suppression releases physically disabled** by the vendor's approved method before any test that could trigger them, and a named person owns re-arming
- [ ] Any emergency power off or shunt-trip outputs identified; the method for testing each without dropping load agreed (test at the output with the trip circuit isolated, then a separate proving test where safe)
- [ ] Alarm receiving center and fire service told that testing will occur, and when it ends
- [ ] Building occupants told; fire watch arranged while detection or suppression is impaired
- [ ] Mechanical and controls contractors attending to witness their equipment
- [ ] Backout reviewed; go/no-go recorded

## 5. Prerequisites (day of test)
- [ ] Fire panel normal, no troubles
- [ ] Air handling, dampers, and doors in normal automatic operation
- [ ] Vendor confirms releases disabled and signals to the receiving center on test

## 6. Safety
Clean-agent and gas systems are a hazard to people if discharged. No one works in a protected space with releases armed during testing.

## 7. Test equipment

| Instrument | ID | Calibration due |
|---|---|---|
| Vendor's detector test equipment | | |
| Multimeter (release and trip circuit measurement) | | |
| Cause-and-effect matrix, used as the record sheet (revision in the ID column) | | |

## 8. Test steps

| # | Action or simulated condition | Expected response | Observed | Pass/Fail | Init. |
|---|---|---|---|---|---|
| 1 | Record as-found: panel state, disabled devices, release status | Recorded | | | |
| 2 | First-stage detection in a zone (one detector) | Alarm at panel and monitoring; actions per the matrix for first stage only | | | |
| 3 | Second-stage detection in the same zone | Pre-discharge sequence starts: sounders, timers, air handling shutdown, damper closure per the matrix | | | |
| 4 | Verify each damper position at the damper | Closed, with feedback at monitoring | | | |
| 5 | Verify air handling and cooling unit response in the zone | Per the matrix (stop, or continue, as designed) | | | |
| 6 | Verify door release and access control response | Per the matrix | | | |
| 7 | Operate the abort function during the timer | Countdown held per the design | | | |
| 8 | Allow the timer to complete with releases disabled | Release output signal measured at the disabled release circuit | | | |
| 9 | Operate the manual release station | Release output signal measured; actions per the matrix | | | |
| 10 | Verify power shutdown outputs with the trip circuit isolated | Output signal present only for the inputs in the matrix | | | |
| 11 | Trigger a detector in an adjacent zone | No action in the zone under test beyond what the matrix shows | | | |
| 12 | Aspirating or early-warning detection levels, if installed | Each level gives its own alarm and action | | | |
| 13 | Create a panel trouble (open circuit, power supply loss) | Trouble at panel and monitoring; no alarm actions | | | |
| 14 | Remove main power from the panel | Panel runs on its standby supply; trouble signal | | | |
| 15 | Reset the fire panel | Dampers, air handling, and doors return to normal only as the design says (automatic or manual reset) | | | |
| 16 | Repeat steps 2 to 15 for each zone | Per the matrix | | | |

Rules: record what was observed, not "OK". A failed step is logged in the [Cx Issues Log](../templates/issues-log.md) with an ID and is retested in full after the root cause is found and corrected.

## 9. Acceptance criteria
Every line of the cause-and-effect matrix proven, and no action observed that is not in the matrix.

| Criterion | Value | Source document and section |
|---|---|---|
| Matrix lines proven (proven / total) | | |
| Actions observed that are not in the matrix (expected: none) | | |
| Release output signal measured at each disabled release circuit | | |
| Power shutdown outputs present only for the inputs in the matrix | | |

## 10. Backout procedure
**Triggers:** any sign that a release is not disabled; an unintended shutdown; a real alarm during testing; unsafe condition.

| If stopped during | Abort steps | End state |
|---|---|---|
| Any step | Vendor silences and resets the panel; all test signals removed | Panel normal, releases still disabled |
| Real alarm during testing | Stop all testing. Treat it as real. Follow the fire EOP. Vendor returns the system to full service at once if safe | Emergency response in effect |
| Unintended shutdown of equipment | Stop. Restore the equipment by its own procedure. Find the unintended link before continuing | Equipment restored; issue logged |
| Step 14 (panel main power removed) | Restore main power; confirm the panel returns to normal and its standby supply is recharging | Panel normal on main power |

**Re-arming is a controlled step:** the named person and the vendor re-arm each release, verify it against the as-found record, and sign the override log.

## 11. Restoration and sign-off
- [ ] All releases re-armed and verified; panel normal with no disabled devices
- [ ] Trip circuits reconnected and verified
- [ ] Receiving center and fire service told that testing is complete; fire watch stood down
- [ ] As-left panel state, disabled devices, and release status recorded and compared with step 1
- [ ] Override log clear; issues logged

| Role | Name | Signature | Date |
|---|---|---|---|
| Cx agent | | | |
| Fire system vendor | | | |
| Mechanical contractor | | | |
| Controls contractor | | | |
| Owner's witness | | | |
