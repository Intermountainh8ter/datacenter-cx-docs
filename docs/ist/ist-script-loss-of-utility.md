---
title: IST Script - Loss and Return of Utility at Design Load
doc_type: ist
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# IST Script: Loss and Return of Utility at Design Load

> Example scenario script for a generic facility on load banks with no production load, written from the [IST Script Template](../templates/ist-script-template.md). Replace device names, delays, and limits with the approved sequence of operations.

## 1. Document control

| Field | Entry |
|---|---|
| Script ID and revision | |
| IST plan reference | |
| Test director | |
| Date and start time | |

## 2. Purpose and scope
Prove the whole facility rides through a utility failure and the return of utility at design load, with all systems in automatic.

## 3. Risk and impact
A failed transfer drops the load banks, not production load. Hazards: switching, generator operation, and rapid heat rise if cooling does not restart.

## 4. Pre-site plan
- [ ] Generator, UPS, transfer, cooling, and monitoring functional tests passed
- [ ] Simulation point agreed: which utility breaker is opened, by whom, with utility agreement if needed
- [ ] Load banks placed and set to design load; remote shut-off tested
- [ ] Recorders assigned to utility switchgear, each generator, each UPS, cooling plant, data hall, and monitoring screen
- [ ] Power quality recorders on the critical bus; time synchronized
- [ ] Temperature limit that ends the test agreed: ____
- [ ] Fuel sufficient; batteries fully charged
- [ ] Backout walked through at the pre-task briefing; go/no-go recorded

## 5. Prerequisites (day of test)
- [ ] All systems in automatic, normal lineup, no alarms
- [ ] Steady state at design load held for the planned period
- [ ] All recorders in position with communications checked

## 6. Safety
Controlled access to electrical rooms and generator areas. Hearing protection. Only the named operator performs switching.

## 7. Test equipment

| Instrument | ID | Calibration due | Location / system recorded |
|---|---|---|---|
| Power quality recorder (critical bus) | | | |
| Power quality recorder (utility or generator bus) | | | |
| Temperature loggers in the data hall | | | |
| Load banks with remote shut-off | | | |
| Event recorder or synchronized clocks for each recorder | | | |

## 8. Test steps

| # | Action | Expected response | Observed / time | Pass/Fail |
|---|---|---|---|---|
| 1 | Record as-found: lineup, loads, temperatures, fuel, battery state | Recorded | | |
| H1 | **HOLD POINT**: test director polls every recorder for ready | All ready | | |
| 2 | Open the utility breaker (simulation point) | Utility lost | | |
| 3 | Observe UPS | Load carried on battery without disturbance | | |
| 4 | Observe generators | Start, reach ready within the design time | | |
| 5 | Observe transfer | Load on generator after the set delay | | |
| 6 | Observe UPS return to rectifier | Battery recharging; no alarms | | |
| 7 | Observe cooling | Units, pumps, and plant restart in sequence within the design time | | |
| 8 | Record peak hall temperature and time to recover | Within limits | | |
| 9 | Run on generator for the planned duration; record loads and fuel | Stable | | |
| H2 | **HOLD POINT**: test director polls for ready to restore utility | All ready | | |
| 10 | Close the utility breaker | Utility present | | |
| 11 | Observe retransfer after the stable-source delay | Load on utility without loss | | |
| 12 | Observe generator cool-down and stop | Stops; returns to auto | | |
| 13 | Observe cooling through retransfer | Continues or restarts per sequence | | |
| 14 | Verify alarms and events at monitoring match what happened, in order | Complete and correctly timed | | |
| 15 | Record as-left and compare with step 1 | Normal lineup | | |

Rules: record what was observed, with the time, not "OK". A failed step is logged in the [Cx Issues Log](../templates/issues-log.md) with an ID; the scenario is rerun in full after the root cause is found and corrected and any affected functional test is repeated.

## 9. Acceptance criteria
From the sequence of operations and design basis. [VERIFY: NFPA 110 acceptance test section]

| Criterion | Value | Source document and section |
|---|---|---|
| UPS output disturbance on loss of utility | | |
| Time from loss of utility to generators ready and load transferred | | |
| Cooling restart time and peak hall temperature | | |
| Retransfer delay and disturbance on return of utility | | |
| Alarm and event sequence at monitoring | | |

## 10. Backout procedure
**Triggers:** generators do not start or transfer before the UPS low-battery warning; cooling does not restart and the hall reaches the agreed limit; any equipment fault; unsafe condition.

| If stopped during steps | Backout steps | End state |
|---|---|---|
| 2 to 5 (generators not carrying load) | B1: Close the utility breaker at once. Confirm UPS returns to rectifier. | On utility |
| 6 to 9 (on generator, cooling problem) | B2: Shed load banks using the remote shut-off. Start cooling manually. If cooling still does not run, close the utility breaker and allow retransfer. | On generator or utility, no heat load |
| 10 to 13 (retransfer fails) | B3: Stay on generator. Do not retry. Reduce load banks. Vendor investigates. | On generator, stable |

After any backout: record the state, log the issue, and do not rerun until the cause is corrected and the affected functional test is repeated.

## 11. Restoration and sign-off
- [ ] Normal lineup; batteries recharging; fuel level recorded
- [ ] Data collected from all recorders and instruments
- [ ] Issues logged

| Role | Name | Signature | Date |
|---|---|---|---|
| Test director | | | |
| Owner's representative | | | |
