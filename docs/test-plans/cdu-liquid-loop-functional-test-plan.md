---
title: CDU and Secondary Fluid Network Functional Test Plan
doc_type: cx-script
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# CDU and Secondary Fluid Network Functional Test Plan

> Example plan for a generic coolant distribution unit (CDU) serving a secondary fluid network (often called the technology cooling system) that supplies direct-to-chip cold plates through row or rack manifolds, with heat rejected to a primary facility water system through the CDU's heat exchanger. Tested at Level 4 before IT equipment is connected, using liquid-cooled heat load units at the manifold connections. Liquid cooling practice varies more between vendors than air cooling does: every value this plan depends on is listed in section 4 and must come from the approved sequence of operations, the CDU manufacturer, and the IT equipment manufacturer before the test is scheduled.

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
| Fluid specification confirmed by the IT equipment manufacturer (name / date) | |
| Test date | |

## 2. Purpose and scope
Prove that the secondary fluid network is clean, tight, filled with fluid of the specified quality, and free of air; that the CDU controls supply temperature, flow, and differential pressure as designed across its load range; that redundant pumps, controls, and (where fitted) redundant CDUs fail over without interrupting flow; that leak detection works at every sensor; and that the CDU responds correctly to loss of facility water, loss of power, and loss of communications. Excluded: testing with live IT equipment, which belongs in Level 5, and the facility water system itself, which has its own functional test.

## 3. Risk and impact
Facility not live; no IT equipment connected. Risks: fluid leaks onto electrical equipment or floors; contamination that later fouls cold plates (particulates, biological growth, wrong fluid chemistry); condensation if supply temperature falls below the room dew point; pump damage from running dry or dead-headed. A fluid that is not clean and correct at this stage becomes the IT vendor's warranty problem later.

## 4. Pre-site plan

### Values to obtain before the test
Do not schedule the test until every row has an entry and a source. A blank row is an open issue.

| Value | Source | Entry |
|---|---|---|
| Fluid type, concentration, and inhibitor package | IT equipment and CDU manufacturers (agreed in writing) | |
| Fluid quality parameters and acceptance values for laboratory analysis | Fluid specification; [VERIFY: ASHRAE Liquid Cooling Guidelines for Datacom Equipment Centers, secondary (TCS) fluid quality section] | |
| Filtration rating required at the CDU and during flushing | CDU and IT equipment manufacturers | |
| Wetted materials permitted in the network | IT equipment and CDU manufacturers | |
| Supply temperature setpoint and allowed range | Sequence of operations; [VERIFY: ASHRAE Liquid Cooling Guidelines for Datacom Equipment Centers, facility and TCS temperature class section] | |
| Differential pressure setpoint and control point location | Sequence of operations | |
| Design flow per manifold branch and balancing tolerance | Design documents; balancing report | |
| Pump failover time and allowed flow dip | Sequence of operations; CDU manufacturer | |
| Minimum margin of supply temperature above room dew point | Sequence of operations | |
| Supply temperature limit that ends a test | Agreed with the design engineer and CDU vendor | |
| Facility water design supply temperature and flow at the CDU | Design documents | |
| CDU power source and designed behavior on power loss (ride-through or auto-restart) | Design documents; sequence of operations | |
| Leak detection response per zone (alarm only, or automatic isolation) | Sequence of operations; leak detection layout | |
| CDU redundancy arrangement (single CDU, or several on a common header) | Design documents | |
| Secondary network test pressure and duration | Design documents; [VERIFY: ASME B31.9 leak testing section] | |
| Relief valve set pressure | Design documents | |

### Readiness
- [ ] Pre-functional checklists and CDU vendor startup report closed and reviewed
- [ ] Pressure test of the secondary network complete and recorded at the values above
- [ ] Relief valve set pressure verified against the design documents
- [ ] Flushing complete with temporary filtration or bypass around cold plate and manifold connections; temporary strainers inspected and clean on the final pass; cleanliness accepted by the method agreed in the flushing plan
- [ ] Laboratory arranged for fluid samples, with the parameters to be reported
- [ ] Liquid-cooled heat load units sized for the design load, with connections compatible with the manifold quick disconnects
- [ ] Facility water system available at design conditions, or the test limits adjusted and recorded
- [ ] Room dew point monitoring available in the space
- [ ] Leak detection layout drawing (sensor locations and zones) obtained
- [ ] Spill kits and absorbent materials staged at the CDU and every manifold
- [ ] Method agreed with the vendor for simulating each failure (pump trip, sensor failure, controller failure, loss of facility water, leak)
- [ ] Trend logging set up on supply and return temperature, flow, differential pressure, pump speed, facility water valve position, and reservoir or expansion tank level
- [ ] Backout reviewed; go/no-go recorded

## 5. Prerequisites (day of test)
- [ ] Final fluid installed; laboratory results within the specification
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

Hold points (H) require the test director to poll every recorder before continuing.

| # | Action or simulated condition | Expected response (with SOO reference) | Observed | Pass/Fail | Init. |
|---|---|---|---|---|---|
| 1 | Record as-found: setpoints, firmware, fluid level, filter differential pressure, fluid concentration | Recorded; matches approved settings and fluid specification | | | |
| 2 | Take a fluid sample at the CDU and at the most remote manifold; send for analysis | Results within the fluid specification (record when received) | | | |
| 3 | Verify flow, supply temperature, and differential pressure at the CDU display against reference instruments | Within instrument accuracy in the specification | | | |
| 4 | With heat load at zero, confirm flow to each manifold branch against the design flow, and confirm the pump minimum-flow or bypass protection is active | Each branch within the balancing tolerance; pumps not dead-headed | | | |
| 5 | Run at full design flow for the agreed period; watch for entrained air (flow or pressure fluctuation, pump noise, air vent activity); vent and recheck the tank level | Stable flow and pressure; no air noise; level restored | | | |
| 6 | Increase heat load in steps to the design load; record supply and return temperature, flow, differential pressure, and facility water valve position at each step | Supply temperature held at setpoint; differential pressure held at setpoint | | | |
| 7 | Confirm the supply temperature stays above the room dew point by the designed margin, and the dew point protection logic (if fitted) is active | Supply temperature above dew point; protection active | | | |
| 8 | Change the supply temperature setpoint within its allowed range | CDU reaches the new setpoint without instability | | | |
| H1 | **HOLD POINT**: test director polls for ready to start failure tests at design load | All ready | | | |
| 9 | At design load, trip the running pump (vendor method) | Standby pump starts within the designed time; flow maintained; alarm at monitoring; record the flow dip and supply temperature excursion | | | |
| 10 | Restore the tripped pump; test lead/lag rotation | Rotation per sequence without flow interruption | | | |
| 11 | If several CDUs serve a common header: at design load, stop one CDU (vendor method) | Remaining CDUs hold flow and differential pressure; alarm at monitoring; record the supply temperature excursion | | | |
| 12 | Fail the supply temperature sensor (disconnect or simulate) | CDU moves to its designed fail-safe or backup sensor; alarm at monitoring | | | |
| 13 | Fail the CDU controller (or the primary controller, if redundant) | Pumps continue in the designed fallback mode; alarm at monitoring | | | |
| H2 | **HOLD POINT**: heat load operator confirms ready to shed; test director restates the supply temperature limit that ends the test | All ready | | | |
| 14 | Interrupt facility water flow to the CDU (close the isolation valve slowly, vendor and mechanical contractor present) | Loss-of-facility-water alarm and the designed CDU response; record the time from interruption to the test limit as an operations baseline. The test team sheds heat load at the limit | | | |
| 15 | Interrupt power to the CDU from its designed source (if UPS-backed, test the transfer) | Behaves as designed: rides through, or restarts automatically on restoration; alarm at monitoring | | | |
| 16 | Lower the reservoir or expansion tank level (vendor method) | Low-level alarm, then pump protection at low-low level, as designed | | | |
| 17 | Wet each leak detection sensor or cable zone in turn with a small amount of water | Correct zone alarm at the CDU and the monitoring system; the designed response (alarm only, or isolation) occurs | | | |
| 18 | Raise filter differential pressure (vendor method or by isolation) | Filter alarm at the designed value | | | |
| 19 | Disconnect communications between the CDU and the monitoring system | Communication failure alarm at the monitoring system; CDU continues in automatic | | | |
| 20 | Isolate one manifold branch (as for IT maintenance) and reconnect it | Remaining branches unaffected; differential pressure held; no air introduced on reconnection | | | |
| 21 | Disconnect and reconnect one rack position's quick disconnects under pressure, following the vendor procedure | Fluid released within what the vendor states; no air introduced; no leak after reconnection | | | |
| 22 | Inspect every joint, quick disconnect, and the CDU drip tray after the test | No leaks; drip trays dry | | | |
| 23 | Verify every point and alarm at the monitoring system | Correct names, values, and priorities | | | |
| 24 | Record as-left settings, fluid level, and concentration; compare with step 1 | Unchanged, or changes approved and recorded | | | |

Rules: record what was observed, not "OK". A failed step is logged in the [Cx Issues Log](../templates/issues-log.md) with an ID and is retested in full after the root cause is found and corrected.

## 9. Acceptance criteria
Each criterion is the value recorded in section 4, from its stated source. A criterion with no recorded source is not an acceptance criterion.

| Criterion | Value (from section 4) | Result |
|---|---|---|
| Fluid quality parameters | | |
| Branch flows within balancing tolerance | | |
| Supply temperature held at setpoint across the load range | | |
| Differential pressure held at setpoint across the load range | | |
| Pump failover time and flow dip | | |
| Supply temperature margin above dew point | | |
| Leak detection response per zone | | |

## 10. Backout procedure
**Triggers:** any leak; supply temperature at or below the room dew point; supply temperature at the agreed limit; pump cavitation, dry running, or dead-heading; loss of facility water not called for by the step; unsafe condition.

| If stopped during | Abort steps | End state |
|---|---|---|
| Any step, leak found | Set heat load units to zero; isolate the affected branch or manifold at its valves; contain and clean up the fluid; record the volume lost and the location | Branch isolated; CDU running on the remaining network or stopped |
| Steps 6 to 11 (under load) | Reduce heat load to zero; restart any stopped pump or CDU; return setpoints to approved values; confirm the CDU is stable in automatic | CDU in automatic, no heat load |
| Steps 12 to 19 (simulated failures) | Remove simulations using the override log; reopen any closed valves; restore power and communications; confirm level, flow, and differential pressure normal | CDU in automatic, no simulations |
| Steps 20 to 21 (branch and quick disconnect work) | Close the branch or rack isolation valves; reconnect and leak-check under the vendor's direction; vent the branch | Branch isolated or restored and vented |

Fluid lost during testing is replaced with fluid of the same specification and concentration, and the concentration is rechecked.

## 11. Restoration and sign-off
- [ ] Setpoints at approved values and verified against step 1
- [ ] Fluid level and concentration within the specification; makeup recorded
- [ ] Override log clear; heat load units disconnected and quick disconnects capped
- [ ] Leak detection sensors dried and reset; all zones normal
- [ ] Laboratory fluid results received and filed
- [ ] Operations baseline recorded: pump failover dip, CDU loss excursion (if tested), and time to limit on loss of facility water; given to operations for the [Loss of Liquid Cooling EOP](../eop/eop-loss-of-liquid-cooling.md)
- [ ] Issues logged; retests scheduled

| Role | Name | Signature | Date |
|---|---|---|---|
| Cx agent | | | |
| Mechanical contractor | | | |
| CDU vendor | | | |
| Owner's witness | | | |
