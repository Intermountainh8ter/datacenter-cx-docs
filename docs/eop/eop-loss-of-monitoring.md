---
title: EOP - Loss of Monitoring System
doc_type: eop
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# EOP: Loss of the Monitoring System (BMS/EPMS)

> Example EOP. The design decides what continues to run without the monitoring system; confirm it from the sequence of operations and commissioning results.

## 1. Document control
Use the block from the [EOP Template](eop-template.md).

## 2. Entry conditions
Operator screens frozen or blank, widespread communication alarms, loss of alarm notifications, or values that have stopped updating.

## 3. Pre-site plan (preparedness)
- [ ] Known from commissioning: which equipment keeps running on local control when the monitoring system, a controller, or the network fails
- [ ] Manual rounds sheet for monitoring-loss conditions prepared, listing the readings to take at the equipment and how often
- [ ] Local alarm indications identified at each major system (panel lights, horns, displays)
- [ ] Current configuration and database backups stored off the server, with a tested restore procedure
- [ ] Controls integrator emergency contact and response time agreed
- [ ] Spare controllers, power supplies, and network switches in stock, with current configuration
- [ ] Network drawings current; server and switch power sources known

## 4. Immediate actions

| # | Action | Expected response |
|---|---|---|
| 1 | Confirm the scope: one screen, one controller, one network segment, or the whole system | Scope known |
| 2 | Check the critical systems at the equipment: power path, UPS, generators in auto, cooling running | Running normally on local control |
| 3 | Start manual rounds on the monitoring-loss sheet at the set interval | Rounds under way |
| 4 | Notify the shift lead; call the controls integrator | Acknowledged |

## 5. Follow-up actions

| # | Action | Expected response | Time |
|---|---|---|---|
| 1 | Check power to the server, workstations, network switches, and controllers | Power present or fault found | |
| 2 | Check network link indicators and any recently changed connection | Fault found or ruled out | |
| 3 | Restart a workstation or application only as the integrator's procedure allows | Screen restored, or no change | |
| 4 | Tell the remote monitoring center that alarms from site are not reliable | Acknowledged | |
| 5 | Suspend planned work that depends on monitoring | Made safe | |
| 6 | Add staff for manual rounds if the outage will be long | Coverage arranged | |
| 7 | Consider whether the loss could be a security event; if so, follow the security incident process and do not reconnect suspect devices | Decision logged | |

## 6. Escalation and communication

| When | Who is told | By whom |
|---|---|---|
| At once | Shift lead, controls integrator | Operator |
| Whole system lost, or not restored within ____ | Facility manager, owner | Shift lead |

## 7. Do not
- Do not restart controllers that are running equipment unless the integrator directs it and the equipment owner is present.
- Do not load an old configuration without confirming it is the current approved one.
- Do not assume "no alarms" means "no problems" while monitoring is down.

## 8. Backout procedure (return to normal)
- **Conditions:** cause found and corrected; all controllers communicating; time synchronized.
- **Steps:** restore one controller or segment at a time; as each returns, compare screen values with the readings taken at the equipment; check that no points were left in override; confirm alarm notifications reach recipients with a test alarm.
- **Verification:** values match the field, trends logging again, gaps in history noted, manual rounds stood down by the shift lead.

## 9. After the event
- [ ] Root cause recorded; backups refreshed
- [ ] Spares used are reordered
- [ ] Post-event review held; monitoring-loss rounds sheet improved from the experience
