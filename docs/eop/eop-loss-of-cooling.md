---
title: EOP - Loss of Cooling
doc_type: eop
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# EOP: Loss of Cooling

> Example EOP for a generic data hall. Temperature limits and time-to-limit come from the design basis and commissioning data, not from this document.

## 1. Document control
Use the block from the [EOP Template](eop-template.md).

## 2. Entry conditions
High temperature alarm in the hall, multiple cooling unit alarms, loss of chilled water flow or plant, or a rapid temperature rise on trends.

## 3. Pre-site plan (preparedness)
- [ ] Hall temperature limits posted, from the design basis: [VERIFY: ASHRAE thermal guidelines section for allowable ranges]
- [ ] Time-to-limit at current load known from commissioning or a heat-rise test
- [ ] Manual start steps for cooling units, pumps, and plant posted at the equipment
- [ ] Portable cooling or fans and their power points identified, if part of the site plan
- [ ] Load-shedding priority list agreed with load owners in advance
- [ ] Mechanical vendor emergency contact current, with pre-arrival information needs known
- [ ] Critical cooling spares in stock per the spares program

## 4. Immediate actions

| # | Action | Expected response |
|---|---|---|
| 1 | Confirm on two indications (room sensors and a second source) | Rise confirmed |
| 2 | Identify what stopped: units, pumps, plant, controls, or power | Scope known |
| 3 | Check whether standby units started automatically | Running, or start them manually |
| 4 | Notify the shift lead | Acknowledged |

## 5. Follow-up actions

| # | Action | Expected response | Time |
|---|---|---|---|
| 1 | Restart stopped units by the posted manual steps | Units running | |
| 2 | If controls failed, place units in local or hand mode per the design | Units running without the controller | |
| 3 | If power to cooling was lost, check its breakers and source | Power restored | |
| 4 | Record hall temperatures at fixed intervals and estimate time to limit | Trend known | |
| 5 | Open containment or doors only if the site plan says this helps | Per site plan | |
| 6 | Deploy portable cooling if available | Running | |
| 7 | If the limit will be reached before cooling returns, start the agreed load-shedding list with the load owners | Heat load reduced | |

## 6. Escalation and communication

| When | Who is told | By whom |
|---|---|---|
| At once | Shift lead, mechanical vendor | Operator |
| Temperature still rising after standby units start | Facility manager, owner, load owners | Shift lead |

## 7. Do not
- Do not shut down IT load without the load owner, unless the pre-agreed limit is reached.
- Do not bypass safeties on chillers or compressors.
- Do not defeat fire or smoke systems to move air.

## 8. Backout procedure (return to normal)
- **Conditions:** cause found and corrected; hall temperatures within limits and falling.
- **Steps:** return units from hand to automatic one at a time, confirming the controller takes over each; remove portable equipment; restore containment; restore shed load in the order agreed with load owners.
- **Verification:** normal unit rotation, setpoints as recorded in the baseline, no alarms, redundancy restated.

## 9. After the event
- [ ] Trend data saved; post-event review held
- [ ] Spares used are reordered
- [ ] Time-to-limit data updated from the real event
