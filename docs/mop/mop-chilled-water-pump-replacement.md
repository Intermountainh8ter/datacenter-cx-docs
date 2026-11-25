---
title: MOP - Replace One Chilled Water Pump
doc_type: mop
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# MOP: Replace One Chilled Water Pump

> Example MOP for one pump in a group with at least one redundant pump. Adapt to the site piping, the controls sequence, and the OEM instructions before use.

## 1. Document control

| Field | Entry |
|---|---|
| MOP ID and revision | |
| Title | |
| Author / date | |
| Technical reviewer / date | |
| Vendor reviewer / date | |
| Approver (facility owner) / date | |
| Change ticket | |
| Scheduled window (start, end) | |
| Risk level | Low / Medium / High (see the risk classes in the [Change and Work Authorization SOP](../sop/sop-change-and-work-authorization.md)) |

## 2. Purpose and scope
Take one pump out of service, replace the pump or motor, and return it to the automatic rotation.

## 3. Risk and impact

| Question | Answer |
|---|---|
| Systems affected | Chilled water flow to the cooling units |
| Redundancy during work | Reduced by one pump. Required / remaining: ____ / ____ |
| Worst credible outcome | A second pump fails; loss of flow. Or a leak or air entering the loop on isolation or refill |
| Response | EOP for loss of cooling; EOP for water leak |

## 4. Pre-site plan
- [ ] Change ticket approved
- [ ] Replacement pump and motor match the approved submittal: duty point, impeller, rotation, frame, seal type, voltage
- [ ] Valve chart reviewed; isolation valves for this pump identified and confirmed to hold (exercise them in advance if allowed)
- [ ] Controls sequence reviewed: how the standby pump starts, and how the system reacts to a pump locked out
- [ ] Lifting plan and equipment for the pump weight and location
- [ ] Gaskets, bolts, seal, coupling, alignment tools, and water treatment chemicals on hand
- [ ] Drain route and containment for the water volume between isolation valves
- [ ] Speed drive settings recorded if the motor changes
- [ ] Backout reviewed; latest PNR time calculated; go/no-go recorded

## 5. Prerequisites (day of work)
- [ ] All other pumps available, no alarms; plant stable
- [ ] Room temperatures within limits; weather not extreme
- [ ] Monitoring center informed; pre-task briefing held

## 6. Safety
Hazards: stored pressure, hot or cold water, rotating parts, lifting, electrical. Lock and tag the motor supply and the isolation valves: [VERIFY: OSHA 1910.147 lockout/tagout section]. Relieve pressure before opening flanges.

## 7. Roles and communications

| Role | Name | Contact |
|---|---|---|
| MOP supervisor (stop/go authority) | | |
| Mechanical technicians | | |
| Electrician | | |
| Verifier | | |
| Plant monitor | | |
| Monitoring contact | | |

Communication method: ____. Use three-part communication for every instruction.

## 8. Procedure steps

| # | Action | Expected response | Reading | Init. | Time |
|---|---|---|---|---|---|
| 1 | Record as-found: pumps running, speeds, differential pressure, flows, supply and return temperatures, alarms | Record complete | | | |
| 2 | Start the standby pump, or confirm it is running | Running and stable | | | |
| H1 | **HOLD POINT**: supervisor confirms go | Approval given | | | |
| 3 | Stop the pump to be replaced through the control system; disable it from the rotation | Pump stops; system pressure and flow hold | | | |
| 4 | Monitor plant and room temperatures for the agreed period: ____ min | Stable | | | |
| 5 | Isolate and lock the motor supply; verify absence of voltage | Safe | | | |
| 6 | Close and lock suction and discharge isolation valves | Closed | | | |
| 7 | Relieve pressure and drain the isolated section to containment; confirm the valves hold | No continuing flow | | | |
| P | **POINT OF NO RETURN**: first flange or coupling opened | | | | |
| 8 | Disconnect, remove the old pump, and install the new one; align per the OEM method | Alignment within the OEM tolerance, recorded | | | |
| 9 | Reconnect piping with new gaskets; torque flanges | Complete | | | |
| 10 | Reconnect the motor; check insulation resistance and rotation direction per the OEM method | Correct | | | |
| 11 | Open the suction valve slowly; fill and vent the pump | No leaks; air removed | | | |
| 12 | Open the discharge valve; remove locks | Ready | | | |
| H2 | **HOLD POINT**: supervisor confirms ready to start | Approval given | | | |
| 13 | Start the pump in hand; check current, vibration, noise, seal, and differential pressure | Normal for the duty point | | | |
| 14 | Return the pump to automatic and to the rotation | Control system accepts it | | | |
| 15 | Record as-left readings; check water treatment levels | Consistent with as-found | | | |

## 9. Backout procedure

**Triggers:** standby pump unstable; system pressure or temperature moving out of range; isolation valve does not hold; safety concern.

**Latest time to cross the PNR:** ____

| If stopped during steps | Backout steps | Expected end state |
|---|---|---|
| 3 to 4 | B1: Re-enable and restart the pump. | As-found |
| 5 to 7 (isolated, not opened) | B2: Refill and vent if drained; open valves; remove locks; restart. | As-found |
| 8 onward (after PNR) | Forward recovery: complete the installation, or blank the open flanges so the isolation is secure, and leave the pump position locked out. Restate redundancy and plan a follow-up MOP. | New pump running, or position safely blanked |

If an isolation valve passes after the PNR, treat it as a leak: stop work, contain, and follow the EOP for water leak.

## 10. Restoration and verification
- [ ] All pumps available in automatic; no alarms
- [ ] No leaks after a set period at operating pressure
- [ ] Vibration baseline recorded for the new pump

## 11. Closeout
- [ ] Asset records, spares register, and drawings updated for the new pump
- [ ] Change ticket closed; old pump disposed of or kept as a rebuildable spare

| Sign-off | Name | Date |
|---|---|---|
| MOP supervisor | | |
| Facility owner | | |
