---
title: Cx Script - Cooling Unit Functional Performance Test
doc_type: cx-script
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Cx Script: Cooling Unit Functional Performance Test

> Example script for a generic chilled-water computer room cooling unit in a group. Align with the OEM procedures and the approved sequence of operations.

## 1. Document control

| Field | Entry |
|---|---|
| Script ID and revision | |
| Cooling units (asset tags) and group | |
| Unit and group controller firmware versions | |
| Reference documents (sequence of operations, setpoint list, water balancing report, submittals, revision) | |
| Author (Cx agent) / date | |
| Reviewed by (design engineer, mechanical contractor, cooling unit vendor, controls) | |
| Test date | |

## 2. Purpose and scope
Prove each unit controls to setpoint, responds to failures, restarts after power loss, and works with the group (lead/standby, rotation).

## 3. Risk and impact
Facility not live. If heat load banks are used, an uncontrolled temperature rise is the main risk.

## 4. Pre-site plan
- [ ] Vendor startup reports received; water balancing report received
- [ ] Sequence of operations and setpoint list at current revision
- [ ] Sensors calibrated or checked against a reference instrument
- [ ] Heat load (load banks) arranged if capacity is to be shown, with power and placement plan
- [ ] Method agreed for each simulated failure (fan fault, sensor fault, loss of water flow, leak, loss of power)
- [ ] Trend logging set on supply and return air, valve position, fan speed, and room sensors
- [ ] Backout reviewed; go/no-go recorded

## 5. Prerequisites (day of test)
- [ ] Chilled water available at design conditions
- [ ] Units in automatic, no alarms; room closed up as in normal operation

## 6. Safety
Rotating fans, electrical panels, hot load banks, open floor tiles. Electrical boundaries and PPE for work in unit panels: [VERIFY: NFPA 70E sections for electrically safe work condition and energized work permit].

## 7. Test equipment

| Instrument | ID | Calibration due |
|---|---|---|
| Reference thermometer | | |
| Reference hygrometer | | |
| Airflow instrument (if required) | | |
| Multimeter | | |
| Heat load banks with remote shut-off (if capacity is shown) | | |

## 8. Test steps

| # | Action or simulated condition | Expected response | Observed | Pass/Fail | Init. |
|---|---|---|---|---|---|
| 1 | Record as-found setpoints and modes | Recorded | | | |
| 2 | Command unit start and stop locally and from the monitoring system | Starts and stops; status correct | | | |
| 3 | Compare each unit sensor with the reference instrument | Within the specified tolerance | | | |
| 4 | Stroke the control valve through its range | Position feedback matches command | | | |
| 5 | Vary fan speed command through its range | Speed feedback matches command | | | |
| 6 | Change the temperature setpoint up, then down | Valve and fan respond in the correct direction; stable control without hunting | | | |
| 7 | Simulate fan failure | Alarm; standby unit starts | | | |
| 8 | Simulate loss of water flow or high supply temperature | Alarm; group responds per sequence | | | |
| 9 | Trigger the unit leak or condensate alarm | Alarm; unit action per sequence | | | |
| 10 | Remove and restore power to the unit | Unit restarts automatically in the set time and mode | | | |
| 11 | Fail the group controller or its network | Units continue on local control per sequence | | | |
| 12 | Test lead/standby rotation | Rotates per sequence | | | |
| 13 | With heat load applied: stop one unit at the redundant load level | Room stays within limits | | | |
| 14 | Verify all points and alarms at the monitoring system | Correct | | | |
| 15 | Record as-left setpoints and modes and compare with step 1 | Unchanged, or changes approved and recorded | | | |

Rules: record what was observed, not "OK". A failed step is logged in the [Cx Issues Log](../templates/issues-log.md) with an ID and is retested in full after the root cause is found and corrected.

## 9. Acceptance criteria
From the sequence of operations and specification. Room conditions: [VERIFY: ASHRAE thermal guidelines section for allowable ranges].

| Criterion | Value | Source document and section |
|---|---|---|
| Sensor accuracy against the reference instrument | | |
| Temperature control stability at setpoint | | |
| Restart time and mode after power loss | | |
| Standby start on unit failure | | |
| Room conditions at the redundant load level | | |

## 10. Backout procedure
**Triggers:** room temperature approaching the test limit; leak; equipment fault; unsafe condition.

| If stopped during | Abort steps | End state |
|---|---|---|
| Steps 2 to 6 | Return setpoints to step 1 values; unit to automatic | Normal |
| Steps 7 to 9, 12 | Remove the simulation per the override log; reset alarms | Normal |
| Steps 10 to 11 (unit or group does not recover) | If heat load is applied and the room is warming, shed the heat load first. Restore power or reconnect the network; start the unit from its local controls; vendor investigates | Unit running on local control; issue logged |
| Step 13 | Switch off heat load first, then restart all units | Room cooling, no heat load |

## 11. Restoration and sign-off
- [ ] Setpoints at approved values; override log clear
- [ ] Load banks off and removed; floor tiles replaced
- [ ] Issues logged

| Role | Name | Signature | Date |
|---|---|---|---|
| Cx agent | | | |
| Mechanical contractor | | | |
| Cooling unit vendor | | | |
| Controls contractor | | | |
| Owner's witness | | | |
