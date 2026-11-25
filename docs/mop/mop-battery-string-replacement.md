---
title: MOP - Replace One UPS Battery String
doc_type: mop
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# MOP: Replace One UPS Battery String

> Example MOP for a generic UPS with more than one battery string, each with its own disconnect, where one string is replaced while the UPS stays in service. Battery chemistry, cabinet design, and the manufacturer's sequence govern. Replace every step with the OEM procedure before use.

## 1. Document control
Use the block from the [MOP Template](mop-template.md).

## 2. Purpose and scope
Isolate one battery string, remove and replace its cells or modules, commission the new string, and return it to service.

## 3. Risk and impact

| Question | Answer |
|---|---|
| Systems affected | UPS battery run time |
| Redundancy during work | **Battery run time is reduced** while the string is out. State the run time on the remaining strings at current load: ____ |
| Worst credible outcome | Utility loss with generators slow or failed while run time is reduced; or a DC short circuit during handling |
| Response | EOP for loss of utility; EOP for UPS failure |

## 4. Pre-site plan
- [ ] Change ticket approved; load owners told of reduced run time
- [ ] New batteries match the type, rating, and manufacturer approved for this UPS; date codes and delivery inspection recorded
- [ ] New batteries stored and charged per the manufacturer since delivery
- [ ] OEM replacement procedure and torque values obtained for the exact cabinet and battery model
- [ ] Remaining run time at current load calculated and accepted by the owner
- [ ] Generators proven ready (recent successful run); fuel confirmed
- [ ] Insulated tools, DC-rated PPE, lifting equipment, and spill kit suited to the battery type confirmed
- [ ] Disposal or recycling of old batteries arranged with a licensed carrier
- [ ] Battery monitoring system changes planned (sensor moves, baseline reset)
- [ ] Acceptance tests defined: [VERIFY: IEEE battery installation and acceptance testing standard for the installed battery type]
- [ ] Backout reviewed; latest PNR time calculated; go/no-go recorded

## 5. Prerequisites (day of work)
- [ ] UPS in normal mode, no alarms; other strings healthy on the monitoring system
- [ ] Utility stable; no storms forecast; generators in auto
- [ ] Monitoring center informed; pre-task briefing held

## 6. Safety
Batteries cannot be de-energized. Each cell and each partial string is a live source. Hazards: DC shock, arc flash, short circuit from tools, weight, electrolyte or thermal event depending on chemistry. DC work rules and PPE: [VERIFY: NFPA 70E sections for electrically safe work condition and energized work permit]. No metal jewelry. One connection worked at a time. Insulating covers over exposed terminals.

## 7. Roles and communications
Supervisor, battery technicians (two minimum), verifier, safety lead, monitoring contact.

## 8. Procedure steps

| # | Action | Expected response | Reading | Init. | Time |
|---|---|---|---|---|---|
| 1 | Record as-found: UPS mode, load, DC bus voltage, per-string current, string voltages, alarms | Record complete | | | |
| 2 | Confirm which string is to be replaced by label and by the monitoring system | Same string identified by both | | | |
| H1 | **HOLD POINT**: supervisor confirms go | Approval given | | | |
| 3 | Open the disconnect for the string being replaced | String isolated; UPS alarm for that string only; other strings on float | | | |
| 4 | Lock and tag the disconnect; verify zero current on the string | Isolated from the UPS (string itself remains live) | | | |
| 5 | Break the string into safe sections by removing inter-tier or inter-cell links per the OEM sequence | Section voltages reduced | | | |
| P | **POINT OF NO RETURN**: first cell or module removed | | | | |
| 6 | Remove old batteries; inspect and clean the cabinet or rack | Cabinet clean, undamaged | | | |
| 7 | Install new batteries; check polarity of every unit before connecting | Polarity correct | | | |
| 8 | Connect links per the OEM sequence; torque and mark each connection | Torque recorded | | | |
| 9 | Measure open-circuit string voltage and compare with the expected value and the DC bus | Within the OEM's allowed difference for closing | | | |
| 10 | Refit monitoring sensors; confirm readings | All cells or modules reporting | | | |
| H2 | **HOLD POINT**: supervisor and vendor confirm ready to connect | Approval given | | | |
| 11 | Remove lock; close the string disconnect | String on charge; current within OEM limits; alarm clears | | | |
| 12 | Monitor charge current, voltages, and temperatures for the agreed period | Stable | | | |
| 13 | Record as-left readings; reset monitoring baselines | Complete | | | |

## 9. Backout procedure

**Triggers:** utility disturbance; alarm on a remaining string; wrong string identified; damaged or wrong new batteries; safety concern.

**Latest time to cross the PNR:** ____

| If stopped during steps | Backout steps | Expected end state |
|---|---|---|
| 3 to 4 (string isolated, intact) | B1: Remove lock; confirm string voltage matches the DC bus within the OEM limit; close the disconnect. | As-found |
| 5 (links removed, no cells removed) | B2: Refit links per the OEM sequence, torque, then B1. | As-found |
| 6 onward (after PNR) | Forward recovery: complete the installation. If new batteries are unusable, leave the string disconnected, locked, and labeled; restate reduced run time to the owner; plan a follow-up MOP. Old batteries removed are not refitted. | New string in service, or string out with risk communicated |

If utility is lost during the work, stop, make connections safe, clear the area, and follow the EOP for loss of utility.

## 10. Restoration and verification
- [ ] All strings connected and on float; no alarms
- [ ] Monitoring shows every cell or module; baselines reset
- [ ] Capacity or acceptance test scheduled if not done in this window

## 11. Closeout
- [ ] Old batteries removed from site with disposal records
- [ ] Battery records updated: install date, date codes, torque sheet
- [ ] Spare stock and register updated; change ticket closed
