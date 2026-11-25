---
title: EOP - UPS on Battery or UPS Failure
doc_type: eop
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# EOP: UPS on Battery or UPS Failure

> Example EOP for a generic UPS system. Replace with the OEM alarm guide and the site's sequence of operations.

## 1. Document control
Use the block from the [EOP Template](eop-template.md).

## 2. Entry conditions
UPS "on battery" with utility present, UPS "on bypass" not commanded, UPS major alarm, or loss of UPS output.

## 3. Pre-site plan (preparedness)
- [ ] OEM alarm list and emergency transfer steps posted at the UPS
- [ ] UPS vendor emergency contact and response time agreed; the vendor's pre-arrival information needs known (model, serial, firmware, alarm codes, photos)
- [ ] Battery monitoring in service and reviewed regularly; battery practices follow the applicable standard: [VERIFY: IEEE battery maintenance standard for the installed battery type]
- [ ] Critical UPS spares in stock per the spares program
- [ ] Operators drilled on reading the UPS mimic and identifying the power path

## 4. Immediate actions

| # | Action | Expected response |
|---|---|---|
| 1 | Read the UPS display: mode, alarms, battery time remaining. Photograph it | Status known |
| 2 | Confirm whether the critical load is still powered | Load status known |
| 3 | Check the UPS input: is the upstream breaker closed and source present? | Source status known |
| 4 | Notify the shift lead; call the UPS vendor | Acknowledged |

## 5. Follow-up actions

**A. On battery with input source lost:** restore the input (check upstream breaker and source). If utility is lost, go to the loss of utility EOP.

**B. On battery with input present (rectifier fault):** battery time is limited. If the bypass source is healthy and the UPS allows it, transfer to static bypass by the OEM steps before the battery is exhausted. The load is then unprotected; say so in every notification.

**C. On static bypass (inverter fault):** the load is unprotected. Do not attempt a return to inverter without the vendor. Keep generators ready. In a 2N design, confirm the other side is healthy and stop all work on it.

**D. Output lost:** confirm what is down. In a dual-path design verify loads continued on the other path. Do not re-energize until the fault is understood.

| # | Action | Expected response | Time |
|---|---|---|---|
| 1 | Record alarm codes and event log | Saved | |
| 2 | Check the battery room or cabinets for heat, smell, or leaks from a safe position | Normal, or evacuate and escalate | |
| 3 | Suspend other work on the power path | Made safe | |

## 6. Escalation and communication

| When | Who is told | By whom |
|---|---|---|
| At once | Shift lead, UPS vendor | Operator |
| Load unprotected or lost | Facility manager, owner, load owners | Shift lead |

## 7. Do not
- Do not repeatedly reset or restart the UPS.
- Do not close the maintenance bypass unless the OEM steps and interlocks for it are being followed.
- Do not enter a battery space with signs of overheating or gas.

## 8. Backout procedure (return to normal)
- **Conditions:** fault found and corrected by the vendor; UPS passes its self-checks; bypass source stable.
- **Steps:** return from bypass to inverter under a MOP (see the maintenance bypass MOP, Part B), with the vendor present.
- **Verification:** UPS on inverter, batteries recharging, no alarms, redundancy restated to the owner.

## 9. After the event
- [ ] Vendor report received; root cause recorded
- [ ] Spares used are reordered
- [ ] Post-event review held
