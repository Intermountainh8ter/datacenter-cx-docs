---
title: Turnover and Handover Checklist
doc_type: checklist
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Turnover and Handover Checklist

For the owner, the operations lead, and the commissioning authority at the handover of a commissioned facility (or phase) from the construction team to operations. It follows acceptance of the IST report (see the [Level 5 Verification Checklist](level-5-verification-checklist.md)) and is the "5 to turnover" gate in the [Commissioning Levels One-Pager](../guides/commissioning-levels.md).

Commissioning proves the facility works. Turnover makes sure the people who will run it have what they need to keep it working: the documents, the baseline, the training, the spares, and an honest list of what is still open. An unchecked item needs a written disposition, an owner, and a date.

| Field | Entry |
|---|---|
| Facility / phase (generic identifier only) | |
| Handover date | |
| Handed over by (construction / commissioning) | |
| Accepted by (owner / operations) | |

## A. Documents

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| A1 | [ ] | Operation and maintenance manuals received for every system, for the models and revisions actually installed. | Document register against the asset list | List missing manuals as open items with the responsible contractor |
| A2 | [ ] | As-built drawings received, including one-lines, P&IDs, controls drawings, and fire alarm drawings. | Drawing register with revision and date | Record which drawings are still marked "as designed" |
| A3 | [ ] | Final sequence of operations and the as-left settings record (protective devices, controls setpoints, timers, alarm limits) received and matching the installed equipment. | Settings record compared with the as-left values from the last tests | Raise any mismatch as an issue; the record, not memory, is the baseline |
| A4 | [ ] | Approved protective device coordination study and arc-flash labels match the installed settings. | Study revision; label spot check | Log as an issue before energized maintenance is planned |
| A5 | [ ] | One-lines, valve charts, and leak detection zone maps posted where operators need them. | Walk-down record | Post before handover |

## B. Commissioning records

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| B1 | [ ] | Final commissioning report received, with every completed FPT and IST script, and the accepted Level 5 verification checklist. | Report and script index | Hold acceptance until complete |
| B2 | [ ] | Issues log closed or each open item dispositioned (owner, date, written acceptance). | Final [Cx Issues Log](../templates/issues-log.md) | List open items in section F |
| B3 | [ ] | Deferred and seasonal tests scheduled with an owner, a date, and who pays. | Deferral log | Add to the maintenance calendar and the issues log |
| B4 | [ ] | Operations baseline handed over: start times, transfer times, restart times, pump failover results, and time-to-limit from the heat-rise and liquid cooling tests. | Baseline sheet signed by the commissioning authority | Request it; EOPs and rounds depend on it |

## C. Operations readiness

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| C1 | [ ] | Operations staff trained on each system by the vendor or installer, with attendance recorded. | Training records | Schedule the missing sessions before handover |
| C2 | [ ] | [Training and Qualification Matrix](../operations/training-and-qualification-matrix.md) shows the minimum qualified people per shift for each critical task, including nights and weekends. | Matrix with the coverage check completed | Record the gap and the interim cover (for example, vendor standby) |
| C3 | [ ] | EOPs written for this facility from the commissioning results, and each critical EOP walked through at least once with operations staff. | EOP register; drill or walk-through records | Do not hand over without the loss of utility, loss of cooling, and fire EOPs in place |
| C4 | [ ] | Rounds sheet issued with normal ranges taken from the commissioning baseline, not from a template. | Rounds sheet | Issue an interim sheet and set a date for the final one |
| C5 | [ ] | [Vendor Emergency Contact Sheet](../operations/vendor-emergency-contact-sheet.md) completed and every number called. | Sheet with the verification date | Complete before handover |
| C6 | [ ] | Critical spares register started and initial stock delivered, labeled, and stored. | Register; spares audit | List shortages with order dates |
| C7 | [ ] | Asset data loaded into the maintenance management system, and the [Maintenance Calendar](../operations/maintenance-calendar.md) populated from the manufacturers' schedules. | System export or calendar | Record the date it will be complete; no asset leaves warranty without a plan |
| C8 | [ ] | Change and work authorization process in force from the handover date. | Process issued; first tickets raised under it | Construction teams still on site work under it too |

## D. Warranties and contracts

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| D1 | [ ] | Warranty start and end dates recorded for each system and major asset. | Warranty register | Agree the start dates in writing; disputes are easier now than later |
| D2 | [ ] | Service contracts in place, with emergency response times that match what the EOPs assume. | Contracts; response time table | Raise the mismatch with the owner |
| D3 | [ ] | Software licenses, support agreements, and firmware support status recorded for controls and monitoring. | License register | List expiring items with renewal owners |

## E. State of the facility at handover

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| E1 | [ ] | No overrides, forced points, jumpers, or temporary settings remain. | Monitoring system "points in override" report; signed override logs | Clear and recheck before handover |
| E2 | [ ] | All systems in normal automatic operation with full redundancy, and no unexplained alarms. | Walk-down and alarm summary | Record each abnormal item with its reason |
| E3 | [ ] | Monitoring alarms routed to operations recipients, not to construction or commissioning staff. | Alarm recipient list; test alarm | Change before handover |
| E4 | [ ] | Keys, access cards, tools, and system credentials handed over through the owner's secure process, and construction access removed. | Handover register | Complete before construction access ends |
| E5 | [ ] | Configuration backups of controllers, monitoring, and programmable equipment taken at the as-left state and stored off the equipment. | Backup register with dates | Take them before handover |
| E6 | [ ] | Temporary equipment (load banks, temporary power and cooling, test instruments) removed, or listed with an owner and removal date. | Walk-down | List what remains |

## F. Open items at handover

| Item | Severity | Owner | Due | Accepted by |
|---|---|---|---|---|
| | | | | |

## Acceptance

| Decision | Name | Role | Date |
|---|---|---|---|
| Accepted / Accepted with the open items above / Not accepted | | Owner's representative | |
| | | Operations lead | |
| | | Commissioning authority | |
