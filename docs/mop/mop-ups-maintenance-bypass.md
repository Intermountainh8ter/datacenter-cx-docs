---
title: MOP - Transfer UPS to Maintenance Bypass and Return
doc_type: mop
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# MOP: Transfer a UPS to Maintenance Bypass and Return

> Example MOP for a generic single UPS system with an internal static bypass and an external maintenance bypass. Breaker names, interlocks, and the exact sequence differ by manufacturer and design. Replace every step with the OEM sequence for the installed model before use.

## 1. Document control
Use the block from the [MOP Template](mop-template.md).

## 2. Purpose and scope
Place the UPS in maintenance bypass so the UPS can be isolated for service, then return the load to conditioned power. Scope ends at the UPS input, output, and bypass breakers.

## 3. Risk and impact

| Question | Answer |
|---|---|
| Systems affected | Loads fed by this UPS |
| Redundancy during work | **While on bypass the load is on unconditioned source power with no battery backup.** In a 2N design the other side remains protected; in a single-path design a source disturbance drops the load. |
| Worst credible outcome | Loss of the bypass source while on bypass |
| Response | EOP for UPS failure; EOP for loss of utility |

## 4. Pre-site plan
- [ ] Change ticket approved; load owners notified of reduced protection
- [ ] OEM transfer sequence and key-interlock scheme for the installed model obtained and reviewed
- [ ] One-line reviewed: bypass source, upstream breakers, downstream loads confirmed
- [ ] Vendor technician confirmed; firmware and parts for the service confirmed
- [ ] Generator readiness and fuel confirmed for the window
- [ ] In a 2N design: no work planned on the other side, and its capacity for full load confirmed
- [ ] Stable weather and no utility work notices for the window
- [ ] Backout reviewed; latest PNR time calculated
- [ ] Go/no-go recorded

## 5. Prerequisites (day of work)
- [ ] UPS in normal mode, no active alarms, batteries healthy
- [ ] Bypass source available and within the UPS acceptance window; UPS reports synchronized to bypass
- [ ] Monitoring center informed; pre-task briefing held

## 6. Safety
Electrical work per the site electrical safety program. Boundaries and PPE: [VERIFY: NFPA 70E sections for electrically safe work condition and energized work permit]. Lockout/tagout of the isolated UPS: [VERIFY: OSHA 1910.147 lockout/tagout section]. Stored energy: batteries and DC capacitors remain hazardous after isolation; follow the OEM discharge time.

## 7. Roles and communications
Supervisor, performer, verifier, vendor technician, monitoring contact. Three-part communication.

## 8. Procedure steps

### Part A: transfer to maintenance bypass

| # | Action | Expected response | Reading | Init. | Time |
|---|---|---|---|---|---|
| 1 | Record as-found: UPS mode, output V and A per phase, load %, battery state, alarms, breaker positions | Record complete | | | |
| 2 | Confirm bypass source voltage and frequency are within the UPS acceptance window | UPS displays bypass available and synchronized | | | |
| H1 | **HOLD POINT**: supervisor confirms go | Approval given | | | |
| 3 | At the UPS control panel, transfer the load to static bypass | Load on static bypass; no interruption; expected "on bypass" alarm only | | | |
| 4 | Verify output voltage and current on all phases | Readings consistent with step 1 | | | |
| H2 | **HOLD POINT**: supervisor confirms closing the maintenance bypass | Approval given | | | |
| 5 | Close the maintenance bypass breaker using the interlock sequence | Breaker closed; maintenance bypass in parallel with static bypass | | | |
| 6 | Open the UPS output breaker | Load fed only through maintenance bypass; load current unchanged | | | |
| 7 | Shut down the UPS per the OEM sequence (inverter, rectifier, battery breaker, input breaker) | UPS de-energized | | | |
| 8 | Apply locks and tags; verify absence of voltage; wait the OEM discharge time | Safe to work | | | |
| 9 | Perform the service work (separate vendor work order) | Work complete, vendor report signed | | | |

### Part B: return to normal

| # | Action | Expected response | Reading | Init. | Time |
|---|---|---|---|---|---|
| 10 | Remove locks and tags; confirm tools and covers accounted for | Area clear | | | |
| 11 | Start the UPS per the OEM sequence up to "inverter ready, on static bypass" with the output breaker open | UPS healthy, synchronized to bypass | | | |
| H3 | **HOLD POINT**: supervisor and vendor confirm UPS is ready to accept load | Approval given | | | |
| 12 | Close the UPS output breaker | Static bypass in parallel with maintenance bypass | | | |
| 13 | Open the maintenance bypass breaker using the interlock sequence | Load on static bypass only | | | |
| 14 | Transfer the load from static bypass to inverter | Load on inverter; no interruption; alarms clear | | | |
| 15 | Record as-left readings and compare with step 1 | Consistent with as-found | | | |

## 9. Backout procedure

**Triggers:** UPS does not report synchronized; any unexpected alarm; a transfer does not produce the expected response; bypass source becomes unstable; safety concern.

There is no single PNR in Part A up to step 6: the UPS can be returned to inverter. After service work begins (step 9), the PNR is the first irreversible service action (for example, removal of a component). **Latest time to start step 9:** ____

| If stopped during steps | Backout steps | Expected end state |
|---|---|---|
| 2 (before any transfer) | B1: Do not transfer. Leave UPS in normal mode. Investigate. | As-found |
| 3 to 4 (on static bypass) | B2: Transfer load back to inverter. Verify readings against step 1. | As-found |
| 5 to 6 (maintenance bypass closed) | B3: Close UPS output breaker if opened; open maintenance bypass breaker per interlock; transfer to inverter. | As-found |
| 7 to 8 (UPS shut down, not yet worked on) | B4: Perform Part B steps 10 to 15. | As-found |
| 9 onward (after PNR) | Forward recovery: complete the service or refit removed parts, then Part B. If the UPS cannot be restored, the load stays on maintenance bypass; escalate, keep generators ready, and plan a follow-up MOP. | Load on bypass, risk communicated |

If the bypass source is lost while on bypass, go to the EOP for loss of utility.

## 10. Restoration and verification
- [ ] UPS in normal mode on inverter, batteries charging, no alarms
- [ ] All breakers in as-found positions; interlock keys returned
- [ ] Monitoring center informed that the window is closed

## 11. Closeout
- [ ] Vendor service report attached
- [ ] Deviations and lessons recorded; change ticket closed
- [ ] Spares used are reordered
