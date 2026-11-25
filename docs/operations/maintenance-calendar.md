---
title: Maintenance Calendar Template
doc_type: template
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Maintenance Calendar Template

A year-long plan of preventive maintenance for critical infrastructure. Intervals are not given here: they come from the equipment manufacturer, the governing standards, the insurer, and the site's own history.

## Sources of intervals

| Source | Use |
|---|---|
| Manufacturer's maintenance schedule for the exact model | The baseline for every asset |
| Electrical equipment maintenance standards | [VERIFY: NETA MTS frequency of maintenance tests guidance] |
| Standby power requirements | [VERIFY: NFPA 110 sections on operational inspection and testing] |
| Battery standards | [VERIFY: IEEE battery maintenance standard for the installed battery type] |
| Fire system requirements | [VERIFY: NFPA 72 sections on inspection, testing, and maintenance] |
| Site history and condition monitoring | Shorten or lengthen intervals with evidence, and with the owner's approval |

## Planning rules

- **One side at a time.** Never schedule maintenance on both paths of a redundant pair in the same window, or the second side before the first is proven back in service.
- **Spread the risk.** Do not stack high-risk tasks in the same week.
- **Avoid bad times.** Keep high-risk work out of freeze periods, extreme weather seasons, and periods of known utility work.
- **Each task maps to a procedure.** Routine checks use an SOP. Anything that changes the state of critical equipment uses a MOP with a pre-site plan and a backout.
- **Book vendors and parts early.** The pre-site plan for each task starts weeks ahead.
- **Overdue is a risk, not a status.** Any deferral is approved, dated, and reviewed monthly.

## Task list

| System | Task | Interval (from source) | Source | Procedure ID | Risk level | Vendor or in-house |
|---|---|---|---|---|---|---|
| Utility and switchgear | Visual inspection and thermal scan | | | | | |
| Utility and switchgear | Breaker and relay maintenance and testing | | | | | |
| Transfer equipment | Inspection and transfer test | | | | | |
| UPS | Preventive maintenance visit | | | | | |
| UPS | Capacitor and fan replacement by age | | | | | |
| Batteries | Inspection and measurements | | | | | |
| Batteries | Capacity or discharge test | | | | | |
| Generators | Exercise run | | | | | |
| Generators | Service: fluids, filters, belts, hoses | | | | | |
| Generators | Load bank test | | | | | |
| Fuel | Sampling and testing; polishing if needed | | | | | |
| Fuel | Tank, piping, and leak detection inspection | | | | | |
| Cooling units | Filters, belts, drains, coils, sensors | | | | | |
| Chilled water plant | Chiller service; pump and drive checks | | | | | |
| Liquid cooling (CDUs) | Filter service; fluid sampling, concentration, and quality testing | | | | | |
| Liquid cooling (CDUs) | Pump rotation and failover check; leak detection test | | | | | |
| Heat rejection | Cleaning and water treatment | | | | | |
| Water systems | Valve exercising; leak detection test | | | | | |
| Monitoring and controls | Backup, sensor calibration, alarm test | | | | | |
| Fire systems | Inspection and testing | | | | | |
| Building | Roof, drains, seals, pest control | | | | | |

## Year view

Mark each task in the month it is due. Use A or B for the side.

| Task | Jan | Feb | Mar | Apr | May | Jun | Jul | Aug | Sep | Oct | Nov | Dec |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| | | | | | | | | | | | | |

## Monthly review

| Month | Planned | Done on time | Deferred (with approval) | Overdue | Oldest overdue (days) | Notes |
|---|---|---|---|---|---|---|
| | | | | | | |

## After each task
- [ ] Report received and reviewed; findings ticketed
- [ ] Readings compared with the previous visit; trends noted
- [ ] Spares used are reordered
- [ ] Calendar updated with the actual date
