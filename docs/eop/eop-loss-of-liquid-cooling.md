---
title: EOP - Loss of Liquid Cooling or Coolant Leak
doc_type: eop
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# EOP: Loss of Liquid Cooling or Coolant Leak

> Example EOP for a generic facility where coolant distribution units (CDUs) supply a secondary fluid network to direct-to-chip cold plates through row or rack manifolds. Liquid-cooled equipment can reach its thermal limit much faster than air-cooled equipment, so the time available is short and comes from commissioning data, not from this document. Replace with the site's sequence of operations, valve charts, and the IT equipment owner's agreed response. For loss of the air cooling or the chilled water plant, use the [Loss of Cooling EOP](eop-loss-of-cooling.md).

## 1. Document control

| Field | Entry |
|---|---|
| EOP ID and revision | |
| Event covered | |
| Owner / approver / date | |
| Last drill date | |
| Posted locations | |

## 2. Entry conditions
CDU alarm (pump failure, low flow, low differential pressure, high supply temperature, low reservoir level, controller fault); leak detection alarm at a CDU, manifold, or rack; loss of facility water to the CDUs; IT equipment thermal alarms on liquid-cooled racks; or a rapid rise in secondary supply temperature on trends.

## 3. Pre-site plan (preparedness)
- [ ] Time from loss of liquid cooling to the IT equipment's limit at current load known, from the [CDU and Secondary Fluid Network Functional Test Plan](../test-plans/cdu-liquid-loop-functional-test-plan.md) (loss of facility water step) and updated from any real event
- [ ] Response agreed in writing with the IT equipment owner: who decides to throttle or shut down liquid-cooled equipment, in what order, and at what condition
- [ ] Manual operation steps for each CDU (standby pump start, local or manual mode) posted at the CDU
- [ ] Valve chart posted for the secondary network, showing which racks each manifold and branch valve feeds
- [ ] Leak detection zone map posted, with the designed response per zone (alarm only, or automatic isolation)
- [ ] Makeup stock of the specified fluid, at the specified concentration, on site and labeled; nothing else used for top-up
- [ ] Spill kits suited to the coolant staged at CDUs and manifolds; the coolant's safety data sheet available
- [ ] CDU vendor and IT equipment owner emergency contacts current, with pre-arrival information needs known (model, serial, firmware, alarm codes, photos)
- [ ] Critical CDU spares in stock per the spares program (pump or pump cartridge, sensors, filters, controller)
- [ ] Room dew point monitored where CDUs or manifolds are installed

## 4. Immediate actions

| # | Action | Expected response |
|---|---|---|
| 1 | Protect people: keep everyone clear of coolant near energized equipment; mark slip hazards | Area controlled |
| 2 | Confirm on two indications (CDU display and monitoring system, or leak alarm and a visual check) | Event confirmed |
| 3 | Identify the type: CDU or pump failure, loss of facility water, leak, low level, or controls | Type known |
| 4 | Check whether the standby pump or standby CDU started automatically | Running, or start it by the posted manual steps |
| 5 | Notify the shift lead **and the IT equipment owner at once**: the time to limit is short | Acknowledged |

## 5. Follow-up actions

**A. CDU or pump failure:** start the standby pump or CDU by the posted steps. If none is available, tell the IT equipment owner the time to limit and prepare the agreed response.

**B. Loss of facility water:** the CDUs can no longer reject heat. Start the [Loss of Cooling EOP](eop-loss-of-cooling.md) for the plant in parallel. Tell the IT equipment owner the time to limit.

**C. Coolant leak:** find the source. Isolate the smallest section that stops it (rack, then branch, then manifold), using the valve chart. Before closing a valve that feeds running equipment, tell the IT equipment owner which racks will lose cooling, so they can throttle or shut down first if time allows. Contain the coolant, keep it off electrical equipment, and estimate the volume lost.

**D. Low reservoir level without a visible leak:** top up with the specified fluid only, and look for the loss (joints, quick disconnects, drip trays, relief valve discharge).

**E. Controls failure:** run the CDU in its local or manual mode per the posted steps; confirm flow and supply temperature at the CDU display.

| # | Action | Expected response | Time |
|---|---|---|---|
| 1 | Record secondary supply and return temperatures, flow, and differential pressure at fixed intervals | Trend known | |
| 2 | Estimate the time to limit from the trend and the commissioning baseline | Time known and told to the IT equipment owner | |
| 3 | If the limit will be reached before cooling returns, the IT equipment owner starts the agreed throttle or shutdown sequence | Heat load reduced | |
| 4 | Check the room dew point against the supply temperature if any setpoint was changed | Supply above dew point | |
| 5 | Record alarm codes and the CDU event log; photograph the display | Saved | |
| 6 | Suspend other work on the liquid cooling system and the racks it serves | Made safe | |

## 6. Escalation and communication

| When | Who is told | By whom |
|---|---|---|
| At once | Shift lead, IT equipment owner | Operator |
| No standby pump or CDU available, or loss of facility water | Facility manager, owner, CDU vendor | Shift lead |
| Coolant on electrical equipment, or IT equipment throttled or shut down | Facility manager, owner, load owners | Shift lead |

## 7. Do not
- Do not top up with water or any fluid other than the specified one, at the specified concentration.
- Do not close a manifold or branch valve that feeds running equipment without telling the IT equipment owner first.
- Do not lower the supply temperature setpoint to compensate; it can fall below the dew point and cause condensation.
- Do not run CDU pumps with a low-low reservoir level.
- Do not re-pressurize a section that leaked until the repair is made.
- Do not disable leak detection to clear nuisance alarms.

## 8. Backout procedure (return to normal)
- **Conditions:** cause found and corrected; any leak repaired and leak-tested; fluid level and concentration within the specification; air vented.
- **Steps:** reopen isolated sections slowly, watching the repair, and vent the high points; return CDUs and pumps to automatic one at a time, confirming each takes control; the IT equipment owner restores load in the agreed order while supply temperature and flow are watched.
- **Verification:** flow, differential pressure, and supply temperature at setpoint; leak detection dry and reset; no alarms; redundancy restated to the owner.

## 9. After the event
- [ ] Fluid sample sent for analysis after any top-up or contamination risk
- [ ] Coolant loss quantity and makeup recorded; makeup stock replenished
- [ ] Time-to-limit baseline updated from the real event
- [ ] Spares used are reordered
- [ ] Post-event review held
