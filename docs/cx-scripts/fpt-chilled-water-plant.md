---
title: Cx Script - Chilled Water Plant Functional Performance Test
doc_type: cx-script
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Cx Script: Chilled Water Plant Functional Performance Test

> Example script for a generic chilled water plant with multiple chillers, pumps, and heat rejection units. Align with the OEM procedures and the approved sequence of operations.

## 1. Document control
Use the block from the [Cx Script Template](cx-script-template.md).

## 2. Purpose and scope
Prove the plant starts, stages, controls to setpoint, survives the loss of any one component, and restarts after a power loss in the time the design requires.

## 3. Risk and impact
Facility not live. Risks: equipment damage from running without flow, freezing an evaporator, water hammer, and rapid room heat rise if load banks are running.

## 4. Pre-site plan
- [ ] Vendor startup reports for chillers, pumps, drives, and heat rejection received
- [ ] Flushing, cleaning, water treatment, and balancing reports received
- [ ] Sequence of operations and setpoint list at current revision and loaded
- [ ] Sensors and flow meters checked against reference instruments
- [ ] Load source arranged (load banks in the hall, or a plant load bank) to hold the plant at test load
- [ ] Method agreed with vendors for each simulated failure, so that safeties are not defeated
- [ ] Trend logging set on temperatures, flows, pressures, valve positions, speeds, and equipment status
- [ ] Weather considered: outdoor conditions needed for the modes to be tested; seasonal tests deferred and logged
- [ ] Power-loss test coordinated with the electrical team
- [ ] Backout reviewed; go/no-go recorded

## 5. Prerequisites (day of test)
- [ ] Plant in automatic, no alarms; cooling units in the hall running
- [ ] Test load applied and stable

## 6. Safety
Rotating equipment, refrigerant, electrical, hot and cold surfaces, water treatment chemicals. Chiller vendor present for chiller failure tests.

## 7. Test equipment
Reference thermometers, pressure gauges, flow measurement, multimeter, load banks.

## 8. Test steps

| # | Action or simulated condition | Expected response | Observed | Pass/Fail | Init. |
|---|---|---|---|---|---|
| 1 | Record as-found setpoints, modes, and lead/standby assignments | Recorded | | | |
| 2 | Plant start from off on a call for cooling | Valves, pumps, heat rejection, and chiller start in the sequence order with proofs at each stage | | | |
| 3 | Hold steady at test load | Supply temperature and differential pressure at setpoint, stable | | | |
| 4 | Raise load in steps | Next chiller and pump stage on at the set conditions without hunting | | | |
| 5 | Lower load in steps | Stages off at the set conditions; minimum flow protected | | | |
| 6 | Change the supply temperature setpoint | Plant follows; stable control | | | |
| 7 | Fail the lead chiller (vendor method) | Standby chiller starts; supply temperature recovers within the design time | | | |
| 8 | Fail the lead chilled water pump | Standby pump starts; flow maintained; no chiller trip | | | |
| 9 | Fail the lead heat rejection pump or fan | Standby starts; chiller stays on line | | | |
| 10 | Fail a control valve or its actuator signal | Fails to the position in the design; alarm | | | |
| 11 | Fail a critical sensor (supply temperature, differential pressure, flow) | Control uses the backup or default per the sequence; alarm | | | |
| 12 | Remove and restore power to the plant | Plant restarts automatically; time from power return to stable supply temperature recorded | | | |
| 13 | Fail the plant controller or its network | Equipment holds last state or runs on local control per the sequence | | | |
| 14 | Test lead/standby rotation | Rotates per the sequence | | | |
| 15 | Test economizer or free-cooling changeover, if installed and conditions allow | Changes over without loss of supply temperature control | | | |
| 16 | Test makeup water, expansion, and low-pressure or leak alarms | Per design | | | |
| 17 | Verify all points and alarms at the monitoring system | Correct | | | |

## 9. Acceptance criteria
Sequence of operations and specification. Record the restart time from step 12 for use in the heat-rise test and in operations.

## 10. Backout procedure
**Triggers:** loss of flow with a chiller running; supply temperature or room temperature approaching the test limit; refrigerant or water leak; equipment fault; unsafe condition.

| If stopped during | Abort steps | End state |
|---|---|---|
| Steps 2 to 6 | Return setpoints to step 1 values; plant to automatic | Normal |
| Steps 7 to 11, 13 | Remove the simulation per the override log; reset alarms with the vendor; confirm the plant recovers | Normal |
| Step 12 | Restore power at once; if the plant does not restart, start it manually by the posted steps | Plant running |
| Room temperature at the limit | Shed load banks first, then recover the plant | No heat load |

## 11. Restoration and sign-off
- [ ] Setpoints at approved values; override log clear
- [ ] Deferred seasonal tests logged with owner and date
- [ ] Issues logged

Sign-off block as in the template.
