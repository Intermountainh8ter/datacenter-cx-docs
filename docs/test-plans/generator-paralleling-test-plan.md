---
title: Generator Paralleling Functional Test Plan
doc_type: cx-script
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Generator Paralleling Functional Test Plan

> Example plan for a generic system of standby generators that synchronize onto a common paralleling bus through paralleling switchgear and a master control system, tested at Level 4 before the facility is live. Single-generator start, transfer, and load tests are covered by the [Generator and Transfer Functional Performance Test](../cx-scripts/fpt-generator-and-transfer.md), which should be complete first. Replace device names, timers, priorities, and limits with the approved sequence of operations and settings.

## 1. Document control

| Field | Entry |
|---|---|
| Script ID and revision | |
| Generators and paralleling switchgear (asset tags) | |
| Master control system and firmware version | |
| Reference documents (sequence of operations, one-line, coordination study, load add/shed schedule, revision) | |
| Author (Cx agent) / date | |
| Reviewed by (design engineer, switchgear vendor, generator vendor, controls) | |
| Test date | |

## 2. Purpose and scope
Prove that the generators start together on a demand signal, that the first unit ready closes to the dead bus, that the others synchronize and close in turn, that they share real and reactive load, that load is added and shed in the designed priority order, and that the system responds correctly to the loss of a unit, the failure of a synchronizer, and the loss of the master controls. State any modes excluded (for example, closed-transition retransfer to utility, peak shaving, or utility export, which need utility agreement and are often tested separately).

## 3. Risk and impact
Facility not live; load is load banks connected to the paralleling bus or to downstream distribution. Risks: an out-of-phase closure can damage generators, switchgear, and couplings; reverse power can motor a generator; load step responses can trip units; switching hazards. These risks are why every closure in this plan is first proven with the synchronism check in place.

## 4. Pre-site plan
- [ ] Generator and transfer functional test passed for every generator; vendor startup reports reviewed
- [ ] Paralleling switchgear startup and protective relay testing complete and reported. Relay test requirements: [VERIFY: NETA ATS protective relay and switchgear test sections]
- [ ] Protective device and generator protection settings match the approved coordination study
- [ ] Phase rotation verified on every generator and on the paralleling bus, and confirmed the same across all of them
- [ ] Synchronizing and synchronism-check (device 25) settings collected from the approved settings
- [ ] Load add and load shed schedule (priorities and steps) obtained and matched to the downstream breakers or transfer switches
- [ ] Load banks sized for the full bus capacity, with step control and remote shut-off; heat rejection and connection points confirmed
- [ ] Power quality recorders on the paralleling bus and on at least one generator output; time synchronized with the master control event log
- [ ] Method agreed with the vendors for simulating each failure (unit trip, failure to synchronize, master control failure, bus fault indication) without damaging equipment
- [ ] Fuel sufficient for all tests plus the turnover level
- [ ] Acceptance test requirements identified: [VERIFY: NFPA 110 acceptance test section]
- [ ] Noise and emissions notifications made
- [ ] Backout reviewed; go/no-go recorded

## 5. Prerequisites (day of test)
- [ ] All generators in auto; fluids, batteries, chargers, and heaters normal; no active alarms
- [ ] Paralleling switchgear in auto; all generator breakers open; bus dead and load banks at zero
- [ ] Master control system healthy; redundant controller (if fitted) in standby
- [ ] Downstream transfer equipment on normal source; loads identified
- [ ] Pre-task briefing held; recorders at each generator, the switchgear, and the master control screen

## 6. Safety
Rotating machinery, hot surfaces, noise, fuel, and medium- or low-voltage switching. Only the named operator operates switchgear. Electrical boundaries and PPE: [VERIFY: NFPA 70E sections for electrically safe work condition and energized work permit]. Stay clear of switchgear during first closures.

## 7. Test equipment

| Instrument | ID | Calibration due |
|---|---|---|
| Power quality recorder (bus) | | |
| Power quality recorder (generator) | | |
| Load bank(s) with step control | | |
| Phase rotation meter | | |
| Stopwatch or event recorder synchronized to the master controls | | |

## 8. Test steps

| # | Action or simulated condition | Expected response (with SOO reference) | Observed | Pass/Fail | Init. |
|---|---|---|---|---|---|
| 1 | Record as-found: settings, timers, priorities, run hours, fuel, firmware | Recorded; matches approved settings | | | |
| 2 | With each generator breaker racked to test (or with the vendor's method), verify the synchronism check blocks a closure when the sources are out of phase | Closure blocked; alarm or indication as designed | | | |
| 3 | Issue a generator demand (simulated loss of normal source) | All units start; start time of each recorded | | | |
| 4 | Observe the first unit ready | Closes to the dead bus; dead-bus logic prevents more than one unit closing to the dead bus | | | |
| 5 | Observe the remaining units | Each synchronizes and closes; time to all units on the bus recorded | | | |
| 6 | Observe load add | Load steps added in the designed priority order as capacity becomes available | | | |
| 7 | Increase load bank load in steps to the design load; record kW and kVAR on each unit at each step | Real and reactive load shared within the tolerance in the specification [VERIFY: project specification load-sharing tolerance] | | | |
| 8 | Apply and remove the largest designed load step; record bus voltage and frequency dip and recovery | Within the specification at each step | | | |
| 9 | Run at design load for the required duration; record engine and electrical data at intervals | Stable; no alarms; sharing maintained | | | |
| 10 | Trip one generator while paralleled at the redundant load (the load the system is designed to carry with one unit out) | Remaining units carry the load; no load shed; the tripped unit's breaker opens and the event is alarmed | | | |
| 11 | Increase load above the capacity of the remaining units, then trip one more unit (or simulate it) | Load shed in the designed priority order; remaining units stay online | | | |
| 12 | Restore the tripped units | Each resynchronizes, closes, and picks up load; shed loads restored in order | | | |
| 13 | Simulate failure of one unit to synchronize within its timer | Unit's breaker stays open; failure-to-synchronize alarm; others unaffected | | | |
| 14 | Simulate a reverse power condition on one unit (vendor method) | Unit's breaker trips on reverse power; alarm at monitoring | | | |
| 15 | If generator demand control (load-dependent start and stop) is in the design: reduce load below the stop threshold, then raise it above the start threshold | Units stop and restart in the designed order with soft unloading and loading | | | |
| 16 | Fail the primary master controller (vendor method) | Redundant controller takes over without dropping the bus, or the system holds its state as designed; alarm at monitoring | | | |
| 17 | Fail the master control communications to one unit | Unit behaves as designed (holds, or remains on the bus in its last mode); alarm at monitoring | | | |
| 18 | Verify manual paralleling mode (if fitted): synchronize and close one unit manually with the synchronism check in place | Closure allowed only within the synchronism window | | | |
| 19 | Restore the normal source | Units unload, open, cool down, and return to auto per the sequence; downstream transfer equipment retransfers | | | |
| 20 | Verify every event, alarm, and status at the monitoring system and the master control event log, in order and correctly timed | Complete and correct | | | |
| 21 | Record as-left settings and compare with step 1 | Unchanged, or changes approved and recorded | | | |

Rules: record what was observed, not "OK". A failed step is logged in the [Cx Issues Log](../templates/issues-log.md) with an ID and is retested in full after the root cause is found and corrected.

## 9. Acceptance criteria
From the specification, the sequence of operations, OEM data, the coordination study, and [VERIFY: NFPA 110 acceptance test section]. Load-step and transient response limits: [VERIFY: ISO 8528 performance class section, as stated in the specification]. Electrical acceptance tests: [VERIFY: NETA ATS sections for generators and paralleling switchgear].

| Criterion | Value | Source document and section |
|---|---|---|
| Time from demand to all units on the bus | | |
| Load-sharing tolerance (kW and kVAR) | | |
| Voltage and frequency dip and recovery at the largest step | | |
| Load add and shed order | | |

## 10. Backout procedure
**Triggers:** an out-of-phase closure or a closure without the synchronism check; generator protective trip not called for by the step; reverse power or loss of excitation on any unit; bus voltage or frequency outside limits; switchgear malfunction; unsafe condition.

| If stopped during | Abort steps | End state |
|---|---|---|
| Steps 2 to 5 (synchronizing) | Remove the generator demand; open any generator breaker that closed abnormally; allow cool-down; investigate before any further closure | All generator breakers open; units in auto or tagged out |
| Steps 6 to 12 (on the bus, under load) | Reduce load banks to zero in steps (at once for a fault); restore the normal source signal; allow units to unload and cool down unless a fault requires immediate stop | Load on normal source; bus dead |
| Steps 13 to 18 (simulated failures) | Remove simulations using the override log; reset with the vendor; confirm the master controls are healthy before continuing | System in auto, no simulations |

Every override, forced point, jumper, and temporary setting is written in the override log when applied and signed off when removed.

## 11. Restoration and sign-off
- [ ] Timers, priorities, and setpoints at approved values and verified against step 1
- [ ] Generator breakers open, units in auto, bus dead
- [ ] Fuel refilled to the turnover level
- [ ] Override log clear; load banks disconnected; recorder data collected
- [ ] Issues logged; retests scheduled

| Role | Name | Signature | Date |
|---|---|---|---|
| Cx agent | | | |
| Switchgear vendor | | | |
| Generator vendor | | | |
| Owner's witness | | | |
