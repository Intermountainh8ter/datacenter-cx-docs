---
title: Cx Script - Automatic and Static Transfer Switch Functional Performance Test
doc_type: cx-script
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Cx Script: Automatic and Static Transfer Switch Functional Performance Test

> Example script for generic automatic transfer switches (ATS) and static transfer switches (STS), tested on a load bank or non-critical load before the facility is live. Align with the OEM procedures and the approved sequence of operations.

## 1. Document control

| Field | Entry |
|---|---|
| Script ID and revision | |
| Switches under test (asset tags; ATS or STS) | |
| Controller firmware versions | |
| Approved settings revision | |
| Reference documents (sequence of operations, one-line, submittals, revision) | |
| Author (Cx agent) / date | |
| Reviewed by (design engineer, electrical contractor, switch vendor) | |
| Test date | |

## 2. Purpose and scope
Prove that each switch transfers and retransfers on the right conditions, at the right times, without unacceptable disturbance to the load, and that its bypass, interlocks, and alarms work.

## 3. Risk and impact
Facility not live. Risks: switching hazards, out-of-phase transfer if settings are wrong, load bank loss during failed transfers.

## 4. Pre-site plan
- [ ] Vendor startup reports received; pre-functional checklists closed
- [ ] Approved settings collected for each switch: pickup and dropout voltages and frequencies, time delays, preferred source, retransfer mode, in-phase or synchronization limits
- [ ] Both sources available for the test dates, with the generator vendor present if a generator is one source
- [ ] Method agreed for simulating each source condition (which upstream breaker, or a test set at the sensing inputs)
- [ ] Power quality recorder available at the load side to capture every transfer
- [ ] Load bank or test load arranged at the level in the specification
- [ ] Phase rotation and phase relationship between the two sources already verified
- [ ] Backout reviewed; go/no-go recorded

## 5. Prerequisites (day of test)
- [ ] Switch on its preferred source, in auto, no alarms; both sources healthy
- [ ] Test load connected and stable

## 6. Safety
Electrical switching and work at sensing terminals: [VERIFY: NFPA 70E sections for electrically safe work condition and energized work permit].

## 7. Test equipment

| Instrument | ID | Calibration due |
|---|---|---|
| Power quality recorder (load side) | | |
| Multimeter | | |
| Phase rotation meter | | |
| Timing device | | |
| Test set for sensing inputs (if used) | | |

## 8. Test steps

### Part A: automatic transfer switch

| # | Action or simulated condition | Expected response | Observed | Pass/Fail | Init. |
|---|---|---|---|---|---|
| A1 | Record as-found settings and compare with the approved settings | Match | | | |
| A2 | Remove the normal source | After the set delay: start signal to the alternate source, then transfer when it is ready | | | |
| A3 | Record the time of each stage and the load-side interruption | Within the design values | | | |
| A4 | Restore the normal source | Retransfer after the set stable-source delay, in the set mode | | | |
| A5 | Record engine cool-down signal timing, if a generator is the alternate | Per settings | | | |
| A6 | Simulate undervoltage on one phase only | Transfers per settings | | | |
| A7 | Simulate under- or over-frequency, if sensing is provided | Transfers per settings | | | |
| A8 | Remove the normal source with the alternate unavailable | Stays or opens per the design; alarm | | | |
| A9 | Fail the alternate source while on it, with normal available | Returns to normal per the design | | | |
| A10 | Operate the test switch and any remote transfer command | Per design | | | |
| A11 | Operate the bypass-isolation sequence to each source; attempt a wrong sequence | Load uninterrupted; interlocks prevent the wrong sequence | | | |
| A12 | Verify every status and alarm at the monitoring system | Correct | | | |

### Part B: static transfer switch

| # | Action or simulated condition | Expected response | Observed | Pass/Fail | Init. |
|---|---|---|---|---|---|
| B1 | Record as-found settings; confirm sources are synchronized within the switch's window | Match; in sync | | | |
| B2 | Command a manual transfer to the alternate source and back | Transfer recorded within the specified time; load undisturbed | | | |
| B3 | Remove the preferred source | Automatic transfer within the specified time | | | |
| B4 | Restore the preferred source | Retransfer per the set mode and delay | | | |
| B5 | Take the sources out of synchronization, if the test setup allows, then remove the preferred source | Behaves per settings (inhibit or delayed transfer); alarm | | | |
| B6 | Simulate a downstream fault condition per the OEM method | Transfer inhibited per design, so the fault is not moved to the other source | | | |
| B7 | Operate the maintenance bypass to each source; attempt a wrong sequence | Load uninterrupted; interlocks hold | | | |
| B8 | Remove one control power supply | Switch continues on the redundant supply; alarm | | | |
| B9 | Verify every status and alarm at the monitoring system | Correct | | | |

Rules: record what was observed, not "OK". A failed step is logged in the [Cx Issues Log](../templates/issues-log.md) with an ID and is retested in full after the root cause is found and corrected.

## 9. Acceptance criteria
Approved settings, specification, and OEM data. Electrical acceptance tests: [VERIFY: NETA ATS sections for generators and transfer switches].

| Criterion | Value | Source document and section |
|---|---|---|
| ATS: time at each stage (start signal, transfer, retransfer, cool-down) | | |
| ATS: load-side interruption on transfer | | |
| STS: transfer time, manual and automatic | | |
| STS: synchronization window and out-of-sync behavior | | |
| Load-side disturbance limits | | |

## 10. Backout procedure
**Triggers:** transfer does not occur or occurs out of phase; load-side disturbance beyond the specification; overheating; interlock fails; unsafe condition.

| If stopped during | Abort steps | End state |
|---|---|---|
| Any simulated source condition | Restore the source or remove the test signal; confirm the switch returns to its preferred source | Preferred source, auto |
| B6 and B8 (simulated fault, control power) | Remove the simulation per the override log; restore control power; confirm with the vendor that the switch is healthy on its preferred source and transfer is not inhibited | Preferred source, auto |
| Switch stuck between positions or on the wrong source | Stop. Remove the test load. Vendor recovers the switch by the OEM method; do not force it | Switch safe, load off |
| Bypass tests | Complete or reverse the bypass sequence per the OEM steps so that no bypass is left closed | Bypass open, switch in auto |

Every jumper, test lead, and temporary setting is entered in the override log and cleared before sign-off.

## 11. Restoration and sign-off
- [ ] Settings at approved values, recorded as-left
- [ ] Override log clear; test load removed
- [ ] Issues logged

| Role | Name | Signature | Date |
|---|---|---|---|
| Cx agent | | | |
| Electrical contractor | | | |
| Switch vendor | | | |
| Owner's witness | | | |
