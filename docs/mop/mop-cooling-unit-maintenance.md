---
title: MOP - Take One Cooling Unit Out of Service
doc_type: mop
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# MOP: Take One Cooling Unit Out of Service for Maintenance

> Example MOP for one computer room cooling unit in a group with at least one redundant unit. Adapt to the site design, the controls sequence, and the OEM instructions before use.

## 1. Document control
Use the block from the [MOP Template](mop-template.md).

## 2. Purpose and scope
Remove one cooling unit from service, maintain it, and return it, without the room leaving its temperature and humidity limits.

## 3. Risk and impact

| Question | Answer |
|---|---|
| Systems affected | Cooling for the room or zone |
| Redundancy during work | Reduced by one unit. State remaining units and required units: ____ / ____ |
| Worst credible outcome | A second unit fails during the window; temperatures rise |
| Response | Return the unit under maintenance if restorable (backout); EOP for loss of cooling |

## 4. Pre-site plan
- [ ] Change ticket approved
- [ ] Current heat load and the capacity of the remaining units compared
- [ ] Room limits taken from the design basis: [VERIFY: ASHRAE thermal guidelines section for allowable ranges]
- [ ] Controls sequence reviewed: how standby units start and how the group responds to a unit stop
- [ ] Parts (filters, belts, sensors) matched to the unit model; drain and water isolation points identified
- [ ] Leak protection and containment for any water-side work
- [ ] Backout reviewed; go/no-go recorded

## 5. Prerequisites (day of work)
- [ ] All other units running or available, no alarms
- [ ] Room temperatures stable and within limits; outdoor conditions not extreme
- [ ] Monitoring center informed; pre-task briefing held

## 6. Safety
Hazards: rotating fans, electrical, pressurized water or refrigerant, working over a raised floor. Lockout/tagout of the unit's electrical supply and valves: [VERIFY: OSHA 1910.147 lockout/tagout section].

## 7. Roles and communications
Supervisor, technician, verifier, room temperature monitor, monitoring contact.

## 8. Procedure steps

| # | Action | Expected response | Reading | Init. | Time |
|---|---|---|---|---|---|
| 1 | Record as-found: unit status, setpoints, supply and return temperatures, room sensor readings, alarms | Record complete | | | |
| 2 | Confirm the standby unit is running or start it manually | Standby unit running and controlling | | | |
| H1 | **HOLD POINT**: supervisor confirms go | Approval given | | | |
| 3 | Stop the unit through its controller | Unit stops; group control compensates | | | |
| 4 | Monitor room temperatures for the agreed stabilization period: ____ min | Temperatures stable and within limits | | | |
| 5 | Isolate electrical supply; apply locks and tags; verify absence of voltage | Safe to work | | | |
| 6 | Isolate water or refrigerant circuits if required by the scope | Isolated; no leaks | | | |
| 7 | Perform maintenance per the OEM task list | Tasks complete | | | |
| 8 | Restore isolations; remove locks and tags | Unit ready | | | |
| 9 | Start the unit and verify operation: fan, valve or compressor response, condensate drain, no leaks | Normal operation | | | |
| 10 | Return the group to its normal automatic rotation | As-found configuration | | | |
| 11 | Record as-left readings and compare with step 1 | Consistent with as-found | | | |

## 9. Backout procedure

**Triggers:** room temperature rising toward its limit; any alarm on a remaining unit; a leak; safety concern.

The point of no return is the first disassembly in step 7 that prevents an immediate restart. Schedule those tasks first, when the most window time remains.

| If stopped during steps | Backout steps | Expected end state |
|---|---|---|
| 3 to 4 | B1: Restart the unit. Verify temperatures recover. | As-found |
| 5 to 6 | B2: Restore isolations, remove locks, restart the unit. | As-found |
| 7 (after PNR) | Forward recovery: reassemble to a runnable state as fast as is safe, then steps 8 to 11. If temperatures approach the limit before the unit can run, go to the EOP for loss of cooling. | Unit running, or EOP in effect |

## 10. Restoration and verification
- [ ] Unit running in automatic; group rotation normal; no alarms
- [ ] Floor tiles, panels, and leak detection restored
- [ ] Monitoring center informed that the window is closed

## 11. Closeout
- [ ] Maintenance record attached; deviations and lessons recorded; change ticket closed
- [ ] Spares used are reordered
