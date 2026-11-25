---
title: EOP - Generator Failure While on Generator Power
doc_type: eop
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# EOP: Generator Failure While on Generator Power

> Example EOP for a generic plant with more than one generator. Replace with the site's load-shed sequence and the generator manufacturer's alarm guide.

## 1. Document control
Use the block from the [EOP Template](eop-template.md).

## 2. Entry conditions
The facility is running on generators and one or more of them fails to start, trips, or shows a shutdown pre-alarm.

## 3. Pre-site plan (preparedness)
- [ ] Generator capacity table posted: load each combination of running units can carry
- [ ] Automatic load-shed sequence known; the manual load-shed priority list agreed with load owners in advance
- [ ] Manual start, manual synchronizing, and breaker closing steps posted at the equipment
- [ ] Generator vendor emergency contact and response time agreed; pre-arrival information needs known (model, serial, alarm codes, photos)
- [ ] Critical generator spares in stock: starting batteries, charger, filters, belts, hoses, sensors, controller
- [ ] Temporary generator connection point and supplier identified, if part of the site plan
- [ ] Operators drilled on reading generator alarm codes

## 4. Immediate actions

| # | Action | Expected response |
|---|---|---|
| 1 | Identify which unit failed and read its alarm. Photograph the display | Cause indication known |
| 2 | Check total load against the capacity of the running units | Within capacity, or overload identified |
| 3 | If overloaded and automatic load shed has not acted, shed load by the agreed priority list | Load within capacity |
| 4 | Check UPS status and battery state | UPS on rectifier, or on battery with time noted |
| 5 | Notify the shift lead; call the generator vendor | Acknowledged |

## 5. Follow-up actions

| # | Action | Expected response | Time |
|---|---|---|---|
| 1 | Check the running units: load, temperatures, oil pressure, fuel | Stable | |
| 2 | Inspect the failed unit from a safe position: leaks, smoke, damage | Condition known | |
| 3 | Restart only if the alarm is one the OEM guide allows operators to reset, and only once | Running, or left stopped | |
| 4 | Do the checks operators are trained for: fuel supply and valves, starting battery and charger, emergency stop not pressed, air intake clear | Fault found or ruled out | |
| 5 | Recalculate fuel run time for the running units | Recorded | |
| 6 | Contact the utility for the restoration estimate | Logged | |
| 7 | Suspend all other work | Made safe | |

## 6. Escalation and communication

| When | Who is told | By whom |
|---|---|---|
| At once | Shift lead, generator vendor | Operator |
| No redundancy left, or load shed performed | Facility manager, owner, load owners | Shift lead |
| Capacity below the critical load | Owner; temporary generator supplier if planned | Facility manager |

## 7. Do not
- Do not crank a failed unit repeatedly.
- Do not bypass engine protective shutdowns.
- Do not restore shed load until capacity is confirmed.
- Do not close a generator breaker onto a live bus by hand without synchronizing equipment and training.

## 8. Backout procedure (return to normal)
- **Conditions:** utility stable and retransfer complete per the loss of utility EOP, or the failed unit repaired and proven.
- **Steps:** restore shed load in the reverse of the shed order, watching capacity at each step; return the repaired unit to auto only after a test run witnessed by the vendor; if the unit remains out of service, tag it and post the reduced capacity table.
- **Verification:** all available units in auto, capacity and redundancy restated to the owner, fuel refilled.

## 9. After the event
- [ ] Vendor report and root cause received
- [ ] Spares used are reordered
- [ ] Load bank test of the repaired unit scheduled under a MOP
- [ ] Post-event review held
