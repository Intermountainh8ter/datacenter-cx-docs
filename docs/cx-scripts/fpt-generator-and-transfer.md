---
title: Cx Script - Generator and Transfer Functional Performance Test
doc_type: cx-script
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Cx Script: Generator and Transfer Functional Performance Test

> Example script for a generic standby generator with automatic transfer equipment, tested before the facility is live. Align with the OEM procedures and the approved sequence of operations.

## 1. Document control
Use the block from the [Cx Script Template](cx-script-template.md).

## 2. Purpose and scope
Prove the generator starts on signal, accepts load, runs at rated load, and that transfer and retransfer follow the sequence of operations.

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
Load bank, power quality recorder, stopwatch or event recorder, thermal imager, sound meter if required.

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
| 11 | For multiple generators: test paralleling, load sharing, and loss of one unit at the redundant load | Per sequence of operations | | | |
| 12 | Verify fuel system functions: transfer pumps, level alarms, leak detection | Per design | | | |
| 13 | Verify all points at the monitoring system | Correct | | | |

## 9. Acceptance criteria
From the specification, OEM data, and [VERIFY: NFPA 110 acceptance test section]. Electrical acceptance tests: [VERIFY: NETA ATS sections for generators and transfer switches].

## 10. Backout procedure
**Triggers:** engine or alternator alarm; reading outside limits; leak or smoke; transfer equipment malfunction; unsafe condition.

| If stopped during | Abort steps | End state |
|---|---|---|
| Steps 2 to 4 | Restore the normal source signal; confirm retransfer; investigate | On normal source, generator in auto or tagged out |
| Steps 5 to 6 | Remove load bank load in steps (at once for a fault); restore normal source; allow cool-down unless the fault requires immediate stop | On normal source |
| Steps 9 to 12 | Remove simulations per the override log; reset with the vendor | Generator in auto |

## 11. Restoration and sign-off
- [ ] Timers and setpoints at approved values
- [ ] Fuel refilled to the turnover level
- [ ] Override log clear; load bank disconnected; issues logged

Sign-off block as in the template.
