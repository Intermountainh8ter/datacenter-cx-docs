---
title: MOP - Isolate One Distribution Path for Maintenance
doc_type: mop
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# MOP: Isolate One Distribution Path (A Side) for Maintenance

> Example MOP for a generic dual-path (A/B) electrical distribution in which every critical load has two feeds. Adapt to the site one-line and the OEM instructions before use.

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
De-energize the A-side switchboard for maintenance while the B side carries the full critical load, then restore.

## 3. Risk and impact

| Question | Answer |
|---|---|
| Systems affected | A-side switchboard and everything fed only from it |
| Redundancy during work | **Critical load is on a single path (B).** Any B-side fault drops load. |
| Worst credible outcome | B-side failure during the window |
| Response | Stop work, restore A if it is in a restorable state (backout); EOPs for utility loss and UPS failure |

## 4. Pre-site plan
- [ ] Change ticket approved; load owners notified of single-path operation
- [ ] One-line and panel schedules reviewed; **every load on A checked for a working B feed**
- [ ] Single-corded loads identified and a plan agreed for each (transfer device, temporary feed, or accepted outage)
- [ ] B-side capacity confirmed for the combined load, including UPS, cooling, and upstream breakers
- [ ] B-side maintenance records reviewed; no open defects
- [ ] Mechanical loads fed from A (pumps, cooling units, controls power) identified with their response to loss of A
- [ ] Maintenance scope, test equipment, and acceptance values agreed: [VERIFY: NETA MTS sections for the equipment being maintained]
- [ ] Protective device settings file backed up
- [ ] Backout reviewed; latest PNR time calculated; go/no-go recorded

## 5. Prerequisites (day of work)
- [ ] No alarms on A or B; generators in auto; utility stable
- [ ] Dual-fed loads verified drawing from both feeds (spot check with power monitoring)
- [ ] Monitoring center informed; pre-task briefing held

## 6. Safety
Establish an electrically safe work condition on the A-side switchboard before maintenance: [VERIFY: NFPA 70E sections for electrically safe work condition and energized work permit]. Lockout/tagout at every source of supply including backfeeds and control power: [VERIFY: OSHA 1910.147 lockout/tagout section].

## 7. Roles and communications

| Role | Name | Contact |
|---|---|---|
| MOP supervisor (stop/go authority) | | |
| Switching performer | | |
| Verifier | | |
| Load monitor (watching B-side load and temperatures) | | |
| Safety lead | | |
| Monitoring contact | | |

Communication method: ____. Use three-part communication for every instruction.

## 8. Procedure steps

| # | Action | Expected response | Reading | Init. | Time |
|---|---|---|---|---|---|
| 1 | Record as-found: A and B loads per phase, breaker positions, alarms | Record complete | | | |
| 2 | Calculate expected B load after transfer and compare with B capacity | Within the site's allowed loading | | | |
| H1 | **HOLD POINT**: supervisor confirms go | Approval given | | | |
| 3 | Open A-side feeder breakers one at a time, starting with the lowest-risk load | After each: load remains on, B load rises as expected, no alarms beyond "loss of A feed" | | | |
| 4 | After each feeder, load monitor confirms affected loads are running on B | Confirmed per feeder | | | |
| 5 | Open the A-side main breaker | Switchboard de-energized | | | |
| 6 | Apply locks and tags at all sources; verify absence of voltage | Electrically safe work condition established | | | |
| P | **POINT OF NO RETURN**: first removal or disassembly | | | | |
| 7 | Perform maintenance and testing per the scope | Results within acceptance values | | | |
| 8 | Inspect: tools out, covers on, connections torqued and marked | Second-person check signed | | | |
| H2 | **HOLD POINT**: supervisor confirms ready to re-energize | Approval given | | | |
| 9 | Remove locks and tags; close the A-side main with feeders open | Switchboard energized, voltages normal | | | |
| 10 | Close A-side feeder breakers one at a time | Loads share between A and B again | | | |
| 11 | Record as-left readings and compare with step 1 | Consistent with as-found | | | |

## 9. Backout procedure

**Triggers:** a load does not continue on B when its A feeder opens; B load or temperature higher than calculated; any B-side alarm; safety concern; latest PNR time passed.

**Latest time to cross the PNR:** ____

| If stopped during steps | Backout steps | Expected end state |
|---|---|---|
| 3 to 4 (feeders being opened) | B1: Re-close the last feeder opened. If stable, re-close the others in reverse order. | As-found |
| 5 to 6 (board de-energized, untouched) | B2: Remove locks; close the A main, then feeders one at a time. | As-found |
| 7 onward (after PNR) | Forward recovery: make the switchboard safe to energize (refit parts, complete inspections), then steps 8 to 11. If it cannot be energized, remain on B, escalate, restrict all other work, and plan a follow-up MOP. | A restored, or single-path risk communicated |

If the B side fails while A is restorable, the supervisor may order an expedited B2 after confirming no person or tool is inside the equipment.

## 10. Restoration and verification
- [ ] All breakers in as-found positions; dual-fed loads drawing from both feeds
- [ ] No alarms; protective settings confirmed against the backed-up file
- [ ] Monitoring center informed that the window is closed

## 11. Closeout
- [ ] Test reports attached; deviations and lessons recorded; change ticket closed
- [ ] Spares used are reordered

| Sign-off | Name | Date |
|---|---|---|
| MOP supervisor | | |
| Facility owner | | |
