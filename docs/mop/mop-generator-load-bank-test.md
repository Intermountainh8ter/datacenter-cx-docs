---
title: MOP - Generator Load Bank Test
doc_type: mop
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# MOP: Generator Load Bank Test

> Example MOP for testing one standby generator against a portable or permanent load bank, with the generator isolated from the critical load. Adapt to the site and the OEM instructions before use.

## 1. Document control
Use the block from the [MOP Template](mop-template.md).

## 2. Purpose and scope
Prove the generator can carry load for a sustained period and record its performance. The critical load is not transferred in this MOP.

## 3. Risk and impact

| Question | Answer |
|---|---|
| Systems affected | Generator under test; standby power redundancy |
| Redundancy during work | The generator under test is not available to the critical load unless the load bank is shed and the unit returned to auto. State the remaining standby capacity: ____ |
| Worst credible outcome | Utility failure during the test with the unit unavailable |
| Response | Shed the load bank at once, return the unit to auto (backout B2); EOP for loss of utility |

## 4. Pre-site plan
- [ ] Change ticket approved; neighbors or authorities notified if noise or emissions rules apply
- [ ] Test duration and load steps taken from the governing requirement: [VERIFY: NFPA 110 section on periodic load testing] and the OEM recommendation
- [ ] Load bank sized for the test load, with cables, connection point, and protective device confirmed
- [ ] Load bank auto-shed on utility failure available and included in the plan, if the design supports it
- [ ] Fuel level sufficient for the test plus the required reserve; refuel plan confirmed
- [ ] Last maintenance report reviewed; no open defects on the unit
- [ ] Calibrated instruments confirmed (power analyzer, temperature measurement)
- [ ] Hot exhaust and discharge air path checked for the load bank location
- [ ] Weather acceptable; no utility work notices
- [ ] Backout reviewed; go/no-go recorded

## 5. Prerequisites (day of work)
- [ ] Utility stable; other generators in auto with no alarms
- [ ] Unit under test: fluid levels, battery, block heater, and no leaks checked
- [ ] Load bank connected, inspected, and its controls tested
- [ ] Monitoring center informed; pre-task briefing held

## 6. Safety
Hazards: rotating parts, hot surfaces, exhaust, noise, electrical connection of temporary cables, fuel. Cable connection and disconnection only with the breaker open, locked, and tested: [VERIFY: NFPA 70E sections for electrically safe work condition and energized work permit]. Hearing protection and barriers around the load bank discharge.

## 7. Roles and communications
Supervisor, generator operator, load bank operator, verifier, monitoring contact. A direct voice channel between the generator and the load bank is required.

## 8. Procedure steps

| # | Action | Expected response | Reading | Init. | Time |
|---|---|---|---|---|---|
| 1 | Record as-found: mode selector, breaker positions, alarms, fuel level, run hours | Record complete | | | |
| 2 | Verify the load bank breaker is open and the load bank is at zero load | Confirmed | | | |
| H1 | **HOLD POINT**: supervisor confirms go | Approval given | | | |
| 3 | Place the unit in the test configuration per the OEM and site design (isolated from the critical bus) | Unit shows expected mode; other units remain in auto | | | |
| 4 | Start the unit | Starts, reaches rated voltage and frequency | | | |
| 5 | Close the load bank breaker | Load bank energized, zero load | | | |
| 6 | Apply the first load step: ____ % | Voltage and frequency recover within the limits in the test plan | | | |
| 7 | Hold; record V, A, kW, Hz, oil pressure, coolant temperature, exhaust temperature | Stable readings | | | |
| 8 | Repeat steps 6 and 7 for each load step up to the test load | Stable at each step | | | |
| 9 | Hold at the test load for the planned duration, recording at fixed intervals | No alarms; temperatures stabilize | | | |
| 10 | Remove load in steps to zero | Stable | | | |
| 11 | Open the load bank breaker | Load bank de-energized | | | |
| 12 | Run the cool-down period per the OEM, then stop | Normal shutdown | | | |
| 13 | Return the unit to auto and normal configuration | Unit ready, no alarms | | | |
| 14 | Record as-left readings and fuel level | Consistent with as-found; fuel above reserve | | | |

## 9. Backout procedure

**Triggers:** utility failure or disturbance; any engine, generator, or load bank alarm; a reading outside the test plan limits; leak, smoke, or abnormal noise; safety concern.

This MOP has no point of no return. The unit can be returned to auto from any step.

| If stopped during steps | Backout steps | Expected end state |
|---|---|---|
| 3 to 5 (no load applied) | B1: Open load bank breaker if closed; return unit to auto. | As-found |
| 6 to 10 (under load) | B2: Shed all load bank load at once; open the load bank breaker; return unit to auto. If the trigger was a utility failure, the unit then follows its automatic sequence. If the trigger was a unit fault, shut down per OEM and tag the unit out of service. | Unit in auto, or tagged out with standby capacity restated |
| 11 to 13 | B3: Complete steps 11 to 13. | As-found |

After a fault-triggered backout, tell operations the remaining standby capacity and raise a repair ticket.

## 10. Restoration and verification
- [ ] Unit in auto; breakers and selectors in as-found positions
- [ ] Load bank disconnected with the breaker open and locked; covers refitted
- [ ] Fuel ordered if below the site's refill level
- [ ] Monitoring center informed that the window is closed

## 11. Closeout
- [ ] Test data sheet attached; results compared with the previous test
- [ ] Deviations and lessons recorded; change ticket closed
