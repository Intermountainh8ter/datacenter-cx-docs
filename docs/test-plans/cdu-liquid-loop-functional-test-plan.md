---
title: CDU and Secondary Fluid Network Functional Test Plan
doc_type: cx-script
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# CDU and Secondary Fluid Network Functional Test Plan

> Example plan for a generic coolant distribution unit (CDU) serving a secondary fluid network (often called the technology cooling system) that supplies direct-to-chip cold plates through row or rack manifolds, with heat rejected to a primary facility water system through the CDU's heat exchanger. Tested at Level 4 before IT equipment is connected, using liquid-cooled heat load units at the manifold connections. Replace device names, setpoints, and limits with the approved sequence of operations, the CDU manufacturer's data, and the IT equipment's fluid requirements.

## 1. Document control

| Field | Entry |
|---|---|
| Script ID and revision | |
| CDU(s) and manifolds (asset tags) | |
| CDU controller firmware version | |
| Reference documents (sequence of operations, P&ID, CDU submittal, IT fluid specification, revision) | |
| Secondary fluid type and specified concentration | |
| Author (Cx agent) / date | |
| Reviewed by (design engineer, mechanical contractor, CDU vendor, controls) | |
| Test date | |

## 2. Purpose and scope
Prove that the secondary fluid network is clean, tight, filled with fluid of the specified quality, and free of air; that the CDU controls supply temperature, flow, and differential pressure as designed across its load range; that redundant pumps and controls fail over without interrupting flow; that leak detection works at every sensor; and that the CDU responds correctly to loss of facility water, loss of power, and loss of communications. State what is excluded (for example, testing with live IT equipment, which belongs in Level 5).

## 3. Risk and impact
Facility not live; no IT equipment connected. Risks: fluid leaks onto electrical equipment or floors; contamination that later fouls cold plates (particulates, biological growth, wrong fluid chemistry); condensation if supply temperature falls below the room dew point; pump damage from running dry or dead-headed. A fluid that is not clean and correct at this stage becomes the IT vendor's warranty problem later.

## 4. Pre-site plan
- [ ] Pre-functional checklists and CDU vendor startup report closed and reviewed
- [ ] Pressure test of the secondary network complete and recorded, with test pressure and duration from the design documents. Leak testing requirements: [VERIFY: ASME B31.9 leak testing section]
- [ ] Flushing plan agreed, including temporary filtration or bypass around cold plate and manifold connections, and the cleanliness acceptance method
- [ ] Fluid specification obtained from the IT equipment and CDU manufacturers (fluid type, concentration, inhibitor, filtration, wetted materials), and conflicts between them resolved in writing. Fluid quality guidance: [VERIFY: ASHRAE Liquid Cooling Guidelines for Datacom Equipment Centers, secondary (TCS) fluid quality section]
- [ ] Laboratory arranged for fluid samples, with the parameters and acceptance values to be reported
- [ ] Liquid-cooled heat load units sized for the design load, with connections compatible with the manifold quick disconnects
- [ ] Facility water system available at design conditions, or the test limits adjusted and recorded
- [ ] Room dew point monitoring available in the space
- [ ] Leak detection layout drawing (sensor locations and zones) obtained
- [ ] Spill kits and absorbent materials staged at the CDU and every manifold
- [ ] Method agreed with the vendor for simulating each failure (pump trip, sensor failure, controller failure, loss of facility water, leak)
- [ ] Trend logging set up on supply and return temperature, flow, differential pressure, pump speed, facility water valve position, and reservoir or expansion tank level
- [ ] Backout reviewed; go/no-go recorded

## 5. Prerequisites (day of test)
- [ ] Network flushed and accepted; final fluid installed; laboratory results within the specification
- [ ] Air vented at all high points; reservoir or expansion tank at normal level
- [ ] CDU in automatic, normal mode, no active alarms
- [ ] Facility water system running and stable
- [ ] Heat load units connected, leak-checked at each quick disconnect, and set to zero
- [ ] Pre-task briefing held; recorders at the CDU, the manifolds, and the monitoring screen

## 6. Safety
Pressurized fluid, slip hazards, electrical equipment near fluid, chemical handling (follow the fluid's safety data sheet), and hot surfaces on heat load units. Electrical boundaries and PPE: [VERIFY: NFPA 70E sections for electrically safe work condition and energized work permit].

## 7. Test equipment

| Instrument | ID | Calibration due |
|---|---|---|
| Portable ultrasonic flow meter | | |
| Reference temperature probes | | |
| Reference pressure gauges | | |
| Refractometer or concentration test kit | | |
| Liquid-cooled heat load units with control | | |
| Dew point meter | | |

## 8. Test steps

| # | Action or simulated condition | Expected response (with SOO reference) | Observed | Pass/Fail | Init. |
|---|---|---|---|---|---|
| 1 | Record as-found: setpoints, firmware, fluid level, filter differential pressure, fluid concentration | Recorded; matches approved settings and fluid specification | | | |
| 2 | Take a fluid sample at the CDU and at the most remote manifold; send for analysis | Results within the fluid specification (record when received) | | | |
| 3 | Verify flow, supply temperature, and differential pressure at the CDU display against reference instruments | Within instrument accuracy in the specification | | | |
| 4 | With heat load at zero, confirm flow to each manifold branch against the design flow (balancing record) | Each branch within the design tolerance | | | |
| 5 | Increase heat load in steps to the design load; record supply and return temperature, flow, differential pressure, and facility water valve position at each step | Supply temperature held at setpoint; differential pressure held at setpoint | | | |
| 6 | Confirm the supply temperature stays above the room dew point by the designed margin, and the dew point protection logic (if fitted) is active | Supply temperature above dew point; protection active | | | |
| 7 | Change the supply temperature setpoint within its allowed range | CDU reaches the new setpoint without instability | | | |
| 8 | At design load, trip the running pump (vendor method) | Standby pump starts within the designed time; flow maintained; alarm at monitoring; record any flow dip and supply temperature excursion | | | |
| 9 | Restore the tripped pump; test lead/lag rotation | Rotation per sequence without flow interruption | | | |
| 10 | Fail the supply temperature sensor (disconnect or simulate) | CDU moves to its designed fail-safe or backup sensor; alarm at monitoring | | | |
| 11 | Fail the CDU controller (or the primary controller, if redundant) | Pumps continue on the designed fallback mode; alarm at monitoring | | | |
| 12 | Interrupt facility water flow to the CDU (close the isolation valve slowly, vendor and mechanical contractor present) | Loss-of-facility-water alarm; heat load units shed or stopped by the test team before the agreed temperature limit; record time to limit | | | |
| 13 | Interrupt power to the CDU (from its designed source; if UPS-backed, test the transfer) | Behaves as designed: rides through, or restarts automatically on restoration; alarm at monitoring | | | |
| 14 | Lower the reservoir or expansion tank level (vendor method) | Low-level alarm, then pump protection at low-low level, as designed | | | |
| 15 | Wet each leak detection sensor or cable zone in turn with a small amount of water | Correct zone alarm at the CDU and the monitoring system; designed response (alarm only, or isolation) occurs | | | |
| 16 | Raise filter differential pressure (vendor method or by isolation) | Filter alarm at the designed value | | | |
| 17 | Disconnect communications between the CDU and the monitoring system | Communication failure alarm at the monitoring system; CDU continues in automatic | | | |
| 18 | Isolate one manifold branch (as for IT maintenance) and reconnect it | Remaining branches unaffected; differential pressure held; no air introduced on reconnection | | | |
| 19 | Inspect every joint, quick disconnect, and the CDU drip tray after the test | No leaks; drip trays dry | | | |
| 20 | Verify every point and alarm at the monitoring system | Correct names, values, and priorities | | | |
| 21 | Record as-left settings, fluid level, and concentration; compare with step 1 | Unchanged, or changes approved and recorded | | | |

Rules: record what was observed, not "OK". A failed step is logged in the [Cx Issues Log](../templates/issues-log.md) with an ID and is retested in full after the root cause is found and corrected.

## 9. Acceptance criteria
From the sequence of operations, the CDU manufacturer's data, the IT equipment fluid specification, and the design basis. Fluid quality: [VERIFY: ASHRAE Liquid Cooling Guidelines for Datacom Equipment Centers, secondary (TCS) fluid quality section]. Supply temperature class and range: [VERIFY: ASHRAE Liquid Cooling Guidelines for Datacom Equipment Centers, facility and TCS temperature class section].

| Criterion | Value | Source document and section |
|---|---|---|
| Supply temperature setpoint and allowed deviation | | |
| Differential pressure setpoint and allowed deviation | | |
| Pump failover time and allowed flow dip | | |
| Minimum margin of supply temperature above dew point | | |
| Fluid quality parameters | | |

## 10. Backout procedure
**Triggers:** any leak; supply temperature at or below the room dew point; supply temperature above the agreed limit; pump cavitation, dry running, or dead-heading; loss of facility water not called for by the step; unsafe condition.

| If stopped during | Abort steps | End state |
|---|---|---|
| Any step, leak found | Set heat load units to zero; isolate the affected branch or manifold at its valves; contain and clean up the fluid; record the volume lost and the location | Branch isolated; CDU running on the remaining network or stopped |
| Steps 5 to 9 (under load) | Reduce heat load to zero; return setpoints to approved values; confirm the CDU is stable in automatic | CDU in automatic, no heat load |
| Steps 10 to 17 (simulated failures) | Remove simulations using the override log; reopen any closed valves; confirm level, flow, and differential pressure normal | CDU in automatic, no simulations |

Fluid lost during testing is replaced with fluid of the same specification and concentration, and the concentration is rechecked.

## 11. Restoration and sign-off
- [ ] Setpoints at approved values and verified against step 1
- [ ] Fluid level and concentration within the specification; makeup recorded
- [ ] Override log clear; heat load units disconnected and quick disconnects capped
- [ ] Leak detection sensors dried and reset; all zones normal
- [ ] Laboratory fluid results received and filed
- [ ] Issues logged; retests scheduled

| Role | Name | Signature | Date |
|---|---|---|---|
| Cx agent | | | |
| Mechanical contractor | | | |
| CDU vendor | | | |
| Owner's witness | | | |
