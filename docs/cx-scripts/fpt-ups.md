---
title: Cx Script - UPS Functional Performance Test
doc_type: cx-script
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Cx Script: UPS Functional Performance Test

> Example script for a generic UPS system tested on a load bank before any critical load is connected. Align every step with the OEM test procedure and the approved sequence of operations.

## 1. Document control
Use the block from the [Cx Script Template](cx-script-template.md).

## 2. Purpose and scope
Prove the UPS carries load in all modes, transfers without interruption, supports the load on battery for the design time, and reports correctly to monitoring.

## 3. Risk and impact
No critical load is connected. Risks are to equipment and people: battery discharge, load bank heat, and switching.

## 4. Pre-site plan
- [ ] Vendor startup complete and report received
- [ ] Battery installation checks complete: [VERIFY: IEEE battery installation and acceptance testing standard for the installed battery type]
- [ ] Load bank sized for the test load, with a connection point and heat rejection path confirmed
- [ ] Power quality recorder available to capture transfers
- [ ] Battery recharge time allowed for in the schedule between discharge tests
- [ ] Design values collected: rated load, battery run time, transfer limits, alarm list
- [ ] Vendor technician confirmed for the test date
- [ ] Backout reviewed; go/no-go recorded

## 5. Prerequisites (day of test)
- [ ] UPS in normal mode, batteries fully charged, no alarms
- [ ] Load bank connected and at zero; room cooling running
- [ ] Monitoring points for the UPS already point-checked

## 6. Safety
Battery and DC hazards, arc flash at connection points, load bank heat. [VERIFY: NFPA 70E sections for electrically safe work condition and energized work permit]

## 7. Test equipment
Load bank, power quality recorder, multimeter, thermal imager, battery monitoring readout.

## 8. Test steps

| # | Action or simulated condition | Expected response | Observed | Pass/Fail | Init. |
|---|---|---|---|---|---|
| 1 | Record as-found: mode, settings, firmware, alarms | Recorded | | | |
| 2 | Apply load in steps up to rated load; record input and output V, A, kW, kVA, Hz at each step | Output within design tolerance at each step | | | |
| 3 | Hold rated load for the planned burn-in period; thermal scan connections | Stable; no abnormal hot spots | | | |
| 4 | Apply and remove a load step of the size in the specification | Output transient within specification, captured on the recorder | | | |
| 5 | Command transfer to static bypass and back | No interruption; recorder shows transfer within specification | | | |
| 6 | Open the UPS input breaker (loss of input) | UPS goes to battery; no output disturbance; alarm at monitoring | | | |
| 7 | Run on battery at the test load for the design run time or to the agreed end voltage; record battery voltage and time | Meets design run time | | | |
| 8 | Restore input | Rectifier walks in; batteries recharge; alarm clears | | | |
| 9 | After recharge, simulate inverter fault per the OEM method | Automatic transfer to static bypass; alarm at monitoring | | | |
| 10 | For multi-module systems: remove one module from service at the redundant load level | Remaining modules carry the load; alarm at monitoring | | | |
| 11 | Test the maintenance bypass sequence and its interlocks | Interlocks prevent wrong sequence; no interruption | | | |
| 12 | Test emergency power off only if required and agreed, with the load bank at zero | Per design | | | |
| 13 | Verify each alarm and status point at the monitoring system | Text, priority, and timestamp correct | | | |

## 9. Acceptance criteria
From the project specification and OEM data. Electrical acceptance tests: [VERIFY: NETA ATS sections for UPS systems and batteries].

## 10. Backout procedure
**Triggers:** output outside tolerance; battery temperature or voltage abnormal; hot spot found; load bank fault; any unsafe condition.

| If stopped during | Abort steps | End state |
|---|---|---|
| Steps 2 to 5 (load testing) | Remove load in steps; return UPS to normal mode | UPS normal, zero load |
| Steps 6 to 8 (on battery) | Restore input at once; remove load; let batteries recharge before any retest | UPS normal, batteries charging |
| Steps 9 to 12 (simulated faults) | Remove the simulation per the override log; return to normal mode with the vendor | UPS normal |

## 11. Restoration and sign-off
- [ ] Settings as recorded in step 1 or as approved
- [ ] Batteries recharged; load bank disconnected with breaker open and locked
- [ ] Override log clear; issues logged

Sign-off block as in the template.
