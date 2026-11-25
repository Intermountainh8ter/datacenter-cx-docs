---
title: Cx Script - Data Hall Heat-Rise Test
doc_type: cx-script
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Cx Script: Data Hall Heat-Rise Test (Loss of Cooling at Load)

> Example script for measuring how fast a data hall warms when cooling stops, using load banks and no production equipment. The result is a baseline for operations and for the loss of cooling EOP.

## 1. Document control

| Field | Entry |
|---|---|
| Script ID and revision | |
| Data hall and cooling systems under test (generic identifiers) | |
| Test load and its distribution | |
| Stop limit and sensor location (from the pre-site plan) | |
| Reference documents (cooling and plant functional test reports, sequence of operations, load bank layout, revision) | |
| Author (Cx agent) / date | |
| Reviewed by (owner, design engineer, mechanical vendor, fire system vendor) | |
| Test date | |

## 2. Purpose and scope
Measure the time from loss of cooling to the agreed temperature limit at a stated load, and the time for the hall to recover once cooling returns.

## 3. Risk and impact
The test deliberately removes cooling. Risks: overheating load banks, cables, or installed equipment; fire detection or suppression responding to heat; cooling that does not restart.

## 4. Pre-site plan
- [ ] Cooling unit and plant functional tests passed, including automatic restart
- [ ] Test load and its distribution across the hall agreed (uniform, or matching the planned layout)
- [ ] **Stop limit agreed in writing** with the owner and design engineer, set below any limit for installed equipment, load banks, and fire detection: ____ at ____ (sensor location)
- [ ] Design reference for hall conditions recorded: [VERIFY: ASHRAE thermal guidelines section for allowable ranges]
- [ ] Fire system vendor consulted on the detection and suppression response to the expected temperatures; protective measures agreed without leaving the hall unprotected
- [ ] Temperature sensors placed: cabinet inlets at several heights and locations, hot aisle, ceiling, and near detectors; logging at short intervals
- [ ] Load banks have remote shut-off, tested, with a person assigned to it
- [ ] Manual restart steps for cooling printed; mechanical vendor attending
- [ ] Containment and doors in their normal operating state
- [ ] Backout reviewed; go/no-go recorded

## 5. Prerequisites (day of test)
- [ ] Hall at steady state on the test load for the agreed period
- [ ] All cooling units and plant in automatic, no alarms
- [ ] All sensors logging; time synchronized with the monitoring system

## 6. Safety
Hot surfaces and hot air in the hall. Limit time in the hot aisle. Fire watch during the test.

## 7. Test equipment

| Instrument | ID | Calibration due |
|---|---|---|
| Temperature loggers (one row per logger, with location) | | |
| Load banks with remote shut-off | | |
| Timing device synchronized to the monitoring system | | |

## 8. Test steps

| # | Action or simulated condition | Expected response | Observed | Pass/Fail | Init. |
|---|---|---|---|---|---|
| 1 | Record as-found: load, supply and return temperatures, all logger readings, cooling status | Recorded | | | |
| H1 | **HOLD POINT**: test director polls load bank operator, cooling operator, and recorders | All ready | | | |
| 2 | Stop cooling by the agreed method (stop units, or remove power to cooling); record the time | Cooling stopped | | | |
| 3 | Record inlet temperatures at fixed short intervals | Rising trend recorded | | | |
| 4 | Continue until the stop limit is reached at the agreed sensor, or the agreed maximum time passes | Limit or time reached; time recorded | | | |
| 5 | Restore cooling; record the time | Units and plant restart | | | |
| 6 | Record the time for inlet temperatures to return to the normal range | Recovery time recorded | | | |
| 7 | Repeat at other load levels or with partial cooling loss, if in scope, after full recovery | Recorded | | | |
| 8 | Check for any alarm or fire system response during the test | As expected | | | |
| 9 | Record as-left: load bank state, cooling status, and temperatures; compare with step 1 | Normal | | | |

Rules: record what was observed, not "OK". A failed step is logged in the [Cx Issues Log](../templates/issues-log.md) with an ID and is retested in full after the root cause is found and corrected.

## 9. Acceptance criteria
This is a characterization test: the result is the time-to-limit and recovery curve. Compare with the design prediction if one exists, and with the ride-through the design relies on (for example, the cooling restart time measured in the plant test).

| Result | Recorded value | Compared with |
|---|---|---|
| Test load and distribution | | |
| Time from cooling stop to the stop limit | | Design prediction, if any |
| Peak temperature and its location | | |
| Time from cooling restart to the normal range | | Cooling restart time from the plant test |
| Ride-through the design relies on | | |

## 10. Backout procedure
**Triggers:** stop limit reached; any sensor rising faster than expected near equipment or detectors; fire alarm pre-signal; cooling does not restart; unsafe condition.

| If stopped during | Abort steps | End state |
|---|---|---|
| Steps 2 to 4, and each repeat in step 7 | **Shed the load banks first** with the remote shut-off, then restart cooling | No heat load, cooling running |
| Cooling does not restart | Keep load banks off; start cooling manually by the posted steps; open doors only if the site plan says this helps | Hall cooling down |
| Fire alarm response | Stop the test; load banks off; follow the fire EOP until the cause is confirmed | Emergency response in effect |

## 11. Restoration and sign-off
- [ ] Cooling in normal automatic operation; hall at normal temperatures
- [ ] Any fire system measures restored and verified by the vendor
- [ ] Loggers collected; data saved with the test load and layout
- [ ] Time-to-limit and recovery time given to operations for the loss of cooling EOP

| Role | Name | Signature | Date |
|---|---|---|---|
| Cx agent | | | |
| Mechanical vendor | | | |
| Owner's witness | | | |
| Operations representative (baseline received) | | | |
