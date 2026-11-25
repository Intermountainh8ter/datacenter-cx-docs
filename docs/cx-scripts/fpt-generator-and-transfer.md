---
title: Cx Script - Generator and Transfer Functional Performance Test
doc_type: cx-script
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Cx Script: Generator and Transfer Functional Performance Test

> Example script for a generic standby generator with automatic transfer equipment, tested before the facility is live. Align with the OEM procedures and the approved sequence of operations.

## 1. Document control

| Field | Entry |
|---|---|
| Script ID and revision | |
| Generator (asset tag, serial number) | |
| Transfer equipment (asset tags) | |
| Generator controller and transfer controller firmware versions | |
| Reference documents (sequence of operations, one-line, settings, submittals, revision) | |
| Author (Cx agent) / date | |
| Reviewed by (design engineer, electrical contractor, generator vendor) | |
| Test date | |

## 2. Purpose and scope
Prove the generator starts on signal, accepts load, runs at rated load, and that transfer and retransfer follow the sequence of operations.

Run this script once per generator. Where several generators parallel onto a common bus, this script proves each unit on its own first; paralleling, load sharing, and load add and shed are then proven with the [Generator Paralleling Functional Test Plan](../test-plans/generator-paralleling-test-plan.md).

## 3. Risk and impact
Facility not live. Risks: engine and alternator damage from faults under load, fuel leaks, switching hazards.

## 4. Pre-site plan
- [ ] Vendor startup report received; factory test report reviewed
- [ ] Acceptance test requirements identified: [VERIFY: NFPA 110 acceptance test section]
- [ ] Load bank sized for rated load; connection and heat rejection confirmed
- [ ] Fuel quantity for all tests plus the turnover level arranged
- [ ] Timers and setpoints in the transfer controls collected from the approved settings
- [ ] Method agreed for simulating utility loss (which breaker or signal)
- [ ] Method agreed for simulating each engine alarm and shutdown safely with the vendor
- [ ] Noise or emissions notifications made
- [ ] Backout reviewed; go/no-go recorded

## 5. Prerequisites (day of test)
- [ ] Generator in auto; fluids, battery, charger, heater normal; no leaks
- [ ] Transfer equipment in auto on normal source
- [ ] Load bank at zero; downstream loads identified

## 6. Safety
Rotating parts, hot surfaces, noise, fuel, electrical switching. [VERIFY: NFPA 70E sections for electrically safe work condition and energized work permit]

## 7. Test equipment

| Instrument | ID | Calibration due |
|---|---|---|
| Load bank with step control and remote shut-off | | |
| Power quality recorder | | |
| Stopwatch or event recorder synchronized to the controller clocks | | |
| Thermal imager | | |
| Sound level meter (if required) | | |

## 8. Test steps

| # | Action or simulated condition | Expected response | Observed | Pass/Fail | Init. |
|---|---|---|---|---|---|
| 1 | Record as-found: settings, timers, run hours, fuel | Recorded | | | |
| 2 | Simulate loss of normal source | Start signal sent after the set delay | | | |
| 3 | Time from loss of source to generator ready | Within the design time | | | |
| 4 | Observe transfer to generator | Transfers after the set delay; load energized | | | |
| 5 | Apply load in the steps defined by the specification; record V, Hz dip and recovery | Within specification at each step | | | |
| 6 | Run at rated load for the required duration; record engine and electrical data at intervals | Stable; no alarms | | | |
| 7 | Restore normal source | Retransfer after the set stable-source delay | | | |
| 8 | Observe cool-down and stop | Runs for the set cool-down, stops, returns to ready | | | |
| 9 | Simulate each pre-alarm and shutdown (vendor method) | Correct local indication, correct action, alarm at monitoring | | | |
| 10 | Test failure to start (crank cycle) if required | Per design; alarm at monitoring | | | |
| 11 | For multiple generators: confirm this unit is ready for the [Generator Paralleling Functional Test Plan](../test-plans/generator-paralleling-test-plan.md) and record that plan's script ID here | Paralleling tested under its own plan | | | |
| 12 | Verify fuel system functions: transfer pumps, level alarms, leak detection | Per design | | | |
| 13 | Verify all points at the monitoring system | Correct | | | |
| 14 | Record as-left settings and timers and compare with step 1 | Unchanged, or changes approved and recorded | | | |

Rules: record what was observed, not "OK". A failed step is logged in the [Cx Issues Log](../templates/issues-log.md) with an ID and is retested in full after the root cause is found and corrected.

## 9. Acceptance criteria
From the specification, OEM data, and [VERIFY: NFPA 110 acceptance test section]. Electrical acceptance tests: [VERIFY: NETA ATS sections for generators and transfer switches].

| Criterion | Value | Source document and section |
|---|---|---|
| Time from loss of normal source to generator ready | | |
| Transfer and retransfer delays | | |
| Voltage and frequency dip and recovery at each load step | | |
| Duration at rated load | | |
| Cool-down time | | |

## 10. Backout procedure
**Triggers:** engine or alternator alarm; reading outside limits; leak or smoke; transfer equipment malfunction; unsafe condition.

| If stopped during | Abort steps | End state |
|---|---|---|
| Steps 2 to 4 | Restore the normal source signal; confirm retransfer; investigate | On normal source, generator in auto or tagged out |
| Steps 5 to 6 | Remove load bank load in steps (at once for a fault); restore normal source; allow cool-down unless the fault requires immediate stop | On normal source |
| Steps 7 to 8 (retransfer fails) | Stay on generator. Do not retry. Reduce load bank load; vendor investigates the transfer controls. Retransfer manually only under the vendor's direction | On generator, stable, or on normal source by manual retransfer |
| Steps 9 to 12 | Remove simulations per the override log; reset with the vendor | Generator in auto |

## 11. Restoration and sign-off
- [ ] Timers and setpoints at approved values
- [ ] Fuel refilled to the turnover level
- [ ] Override log clear; load bank disconnected; issues logged
- [ ] Recorder data and controller event logs collected

| Role | Name | Signature | Date |
|---|---|---|---|
| Cx agent | | | |
| Electrical contractor | | | |
| Generator vendor | | | |
| Owner's witness | | | |
