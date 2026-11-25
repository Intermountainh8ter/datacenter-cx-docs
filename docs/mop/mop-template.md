---
title: MOP Template
doc_type: mop
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Method of Procedure (MOP) Template

> Template. Adapt to the site, the manufacturer's instructions, and the approved design. A MOP is valid only after site-specific review and approval.

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
| Risk level | Low / Medium / High |

## 2. Purpose and scope
- What will be done, on which equipment (asset tags), and why.
- What is out of scope.

## 3. Risk and impact

| Question | Answer |
|---|---|
| Systems affected | |
| Redundancy before, during, and after the work | |
| Worst credible outcome | |
| Response to the worst credible outcome (EOP reference) | |
| Customer or load impact expected | None / describe |

## 4. Pre-site plan
Complete the [Pre-Site Plan](../guides/pre-site-plan.md) and attach it. Minimum for this MOP:
- [ ] Change ticket approved; stakeholders notified
- [ ] Drawings and OEM instructions for the exact model reviewed
- [ ] Personnel named and qualified
- [ ] Tools, calibrated test equipment, and parts confirmed
- [ ] Spares for the worst credible failure confirmed on site
- [ ] Backout procedure reviewed and time-budgeted
- [ ] Go/no-go decision recorded

## 5. Prerequisites (day of work)
- [ ] No active alarms on affected or redundant systems
- [ ] Redundant path verified healthy and carrying or ready to carry load
- [ ] Weather and utility conditions acceptable
- [ ] Monitoring center informed that the window is open
- [ ] Pre-task briefing held and signed
- [ ] As-found readings recorded (section 8, step 1)

## 6. Safety
- Hazards: (electrical, mechanical, stored energy, noise, heat, chemicals, working at height)
- Controls and PPE: per the site safety program. Electrical PPE and boundaries: [VERIFY: NFPA 70E sections for electrically safe work condition and energized work permit]
- Lockout/tagout points: list each isolation point by tag. [VERIFY: OSHA 1910.147 lockout/tagout section]
- Emergency contacts and nearest first aid, eyewash, and exits

## 7. Roles and communications

| Role | Name | Contact |
|---|---|---|
| MOP supervisor (stop/go authority) | | |
| Performer | | |
| Verifier | | |
| Safety lead | | |
| Vendor technician | | |
| Operations / monitoring contact | | |

Communication method: ____. Use three-part communication for every instruction.

## 8. Procedure steps

| # | Action | Expected response | Reading | Init. | Time |
|---|---|---|---|---|---|
| 1 | Record as-found state: positions, modes, loads, alarms | Record complete | | | |
| 2 | | | | | |
| H | **HOLD POINT**: supervisor confirms go | Approval given | | | |
| 3 | | | | | |
| P | **POINT OF NO RETURN** | | | | |
| 4 | | | | | |

## 9. Backout procedure

**Triggers:** expected response not observed; unexpected alarm; safety concern; latest PNR time passed.

**Latest time to cross the PNR:** ____

| If stopped during steps | Backout steps | Expected end state |
|---|---|---|
| 2 to 3 (before PNR) | B1 ... | As-found state restored |
| After PNR | Forward recovery: ... | Work completed or system made safe |

Verify backout by comparing against the as-found record from step 1.

## 10. Restoration and verification
- [ ] System returned to normal configuration and automatic modes
- [ ] As-left readings recorded and compared with as-found
- [ ] No unexpected alarms; redundancy restored
- [ ] Locks and tags removed and accounted for
- [ ] Monitoring center informed that the window is closed

## 11. Closeout
- [ ] Deviations and lessons recorded
- [ ] Change ticket closed with actual times
- [ ] Spares used are reordered
- [ ] Drawings or records updated if anything changed

| Sign-off | Name | Date |
|---|---|---|
| MOP supervisor | | |
| Facility owner | | |
