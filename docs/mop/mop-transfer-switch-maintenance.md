---
title: MOP - Automatic Transfer Switch Maintenance
doc_type: mop
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# MOP: Automatic Transfer Switch Maintenance Using Bypass-Isolation

> Example MOP for a generic automatic transfer switch (ATS) that has a bypass-isolation feature. If the switch has no bypass, the load must be fed another way or shut down, and this MOP does not apply. Replace every step with the OEM sequence before use.

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
Bypass the ATS to its connected source, isolate the transfer mechanism, maintain and test it, and return it to automatic service.

## 3. Risk and impact

| Question | Answer |
|---|---|
| Systems affected | Loads fed through this ATS |
| Redundancy during work | **While bypassed, there is no automatic transfer.** A loss of the connected source drops the load until someone transfers the bypass by hand, unless a UPS or a second path covers it |
| Worst credible outcome | Source loss while bypassed |
| Response | Manual bypass transfer to the other source by the posted OEM steps; EOP for loss of utility |

## 4. Pre-site plan
- [ ] Change ticket approved; load owners notified
- [ ] OEM bypass-isolation sequence for the exact model obtained and reviewed; interlocks understood
- [ ] One-line reviewed: both sources, downstream loads, and what covers the load during a source loss
- [ ] Manual bypass transfer steps printed and assigned to a named person stationed at the switch for the whole window
- [ ] Generator readiness and fuel confirmed
- [ ] Maintenance scope and acceptance values agreed: [VERIFY: NETA MTS sections for the equipment being maintained]
- [ ] Controller settings recorded and backed up
- [ ] Stable weather; no utility work notices
- [ ] Backout reviewed; latest PNR time calculated; go/no-go recorded

## 5. Prerequisites (day of work)
- [ ] ATS on normal source, in auto, no alarms; both sources healthy or the alternate proven available
- [ ] Monitoring center informed; pre-task briefing held

## 6. Safety
Line and load terminals of the bypass remain energized while the transfer mechanism is isolated. Work only on the isolated, withdrawn, or de-energized section, verified dead: [VERIFY: NFPA 70E sections for electrically safe work condition and energized work permit]. Lock and tag per [VERIFY: OSHA 1910.147 lockout/tagout section].

## 7. Roles and communications

| Role | Name | Contact |
|---|---|---|
| MOP supervisor (stop/go authority) | | |
| Performer | | |
| Verifier | | |
| Standby operator at the bypass handle | | |
| Vendor technician | | |
| Monitoring contact | | |

Communication method: ____. Use three-part communication for every instruction.

## 8. Procedure steps

| # | Action | Expected response | Reading | Init. | Time |
|---|---|---|---|---|---|
| 1 | Record as-found: source connected, load per phase, controller settings and timers, alarms | Record complete | | | |
| 2 | Confirm which source the load is on and that the bypass will be closed to the same source | Same source | | | |
| H1 | **HOLD POINT**: supervisor confirms go | Approval given | | | |
| 3 | Close the bypass to the connected source per the OEM sequence | Bypass closed indication; load uninterrupted | | | |
| 4 | Move the transfer switch to the isolated (or test) position per the OEM sequence | Isolated indication; load on bypass only | | | |
| 5 | Verify load voltage and current | Consistent with step 1 | | | |
| 6 | Verify absence of voltage on the isolated section; apply locks and tags | Safe to work | | | |
| P | **POINT OF NO RETURN**: first disassembly of the transfer mechanism | | | | |
| 7 | Perform maintenance per the OEM task list: clean, inspect contacts, lubricate, check connections, test controller | Tasks complete; results within acceptance values | | | |
| 8 | In the test position, operate the switch electrically in both directions | Transfers and retransfers correctly without affecting the load | | | |
| 9 | Confirm controller settings match step 1 | Match | | | |
| H2 | **HOLD POINT**: supervisor and vendor confirm ready to return | Approval given | | | |
| 10 | Return the transfer switch to the connected position on the same source as the bypass | Connected indication | | | |
| 11 | Open the bypass per the OEM sequence | Load on the ATS; uninterrupted | | | |
| 12 | Confirm ATS in auto; record as-left readings | Consistent with as-found | | | |

## 9. Backout procedure

**Triggers:** interlock prevents an expected operation; indication does not match the expected state; source disturbance; safety concern.

**Latest time to cross the PNR:** ____

| If stopped during steps | Backout steps | Expected end state |
|---|---|---|
| 3 (bypass closed, switch still connected) | B1: Open the bypass per the OEM sequence. | As-found, in auto |
| 4 to 6 (switch isolated, untouched) | B2: Remove locks; return the switch to connected; open the bypass. | As-found, in auto |
| 7 onward (after PNR) | Forward recovery: reassemble and test in the test position, then steps 10 to 12. If the switch cannot be returned, the load stays on bypass with a standby operator or a posted manual transfer procedure; escalate and plan a follow-up MOP. | ATS in auto, or load on bypass with risk communicated |

**Source loss while bypassed:** the standby operator transfers the bypass to the alternate source by the posted OEM steps once that source is confirmed available.

## 10. Restoration and verification
- [ ] ATS in auto on normal source; bypass open; no alarms
- [ ] Controller timers and settings verified
- [ ] An automatic transfer test done under a separate approved step or scheduled

## 11. Closeout
- [ ] Maintenance report attached; change ticket closed; spares used are reordered

| Sign-off | Name | Date |
|---|---|---|
| MOP supervisor | | |
| Facility owner | | |
