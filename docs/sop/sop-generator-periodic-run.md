---
title: SOP - Generator Periodic Run
doc_type: sop
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# SOP: Generator Periodic Run

> Example SOP for the routine exercise of a standby generator. Frequency, duration, and loading come from the governing requirement and the OEM, not from this document.

## 1. Document control

| Field | Entry |
|---|---|
| SOP ID and revision | |
| Title | |
| Owner | |
| Approver / date | |
| Review cycle | Every ____ months, and after any related incident or equipment change |
| Frequency of task | Frequency and minimum loading: [VERIFY: NFPA 110 section on routine exercise frequency and loading] |

## 2. Purpose and scope
Prove the generator starts and runs, keep the engine in condition, and catch defects early. This SOP covers a run that does **not** transfer the critical load. A run that transfers load needs a MOP.

## 3. Risk
The unit is briefly out of auto in some designs. A utility failure during the run is the main risk; the backout returns the unit to auto.

## 4. Pre-site plan
- [ ] Run scheduled; no conflict with other work on standby power
- [ ] Last run sheet and open defects reviewed
- [ ] Fuel level above the site's minimum for a test run
- [ ] Weather and utility conditions normal
- [ ] Any noise or emissions notification made
- [ ] Vendor attendance confirmed, if the run is combined with a service visit

## 5. Safety
Hearing protection. Keep clear of rotating parts and hot exhaust. Do not open electrical compartments.

## 6. Procedure steps

| # | Action | Expected response or normal range | Reading | Init. |
|---|---|---|---|---|
| 1 | Tell the monitoring center the run is starting | Acknowledged | | |
| 2 | Pre-start checks: oil, coolant, fuel, battery voltage, charger, block heater, leaks, air intake and exhaust clear | All normal | | |
| 3 | Record run hours and fuel level | Recorded | | |
| 4 | Start the unit by the method in the site design (test switch or controller) | Starts within the site's start time; no alarms | | |
| 5 | Record voltage, frequency, oil pressure, coolant temperature after warm-up | Site normal ranges | | |
| 6 | Walk around: leaks, vibration, exhaust color, unusual noise | None | | |
| 7 | Run for the required duration | Stable readings | | |
| 8 | Stop by the normal method; allow the cool-down | Normal shutdown | | |
| 9 | Confirm the selector is in auto and no alarms remain | Ready for automatic start | | |
| 10 | Record run hours and fuel level; tell the monitoring center the run is complete | Acknowledged | | |

## 7. Abnormal conditions
- Fails to start: do not retry repeatedly. Tag the unit, notify the shift lead, and state the remaining standby capacity.
- Alarm or abnormal reading during the run: stop by the normal method unless the OEM instructions say otherwise, then report.
- Leak or smoke: emergency stop, then the matching EOP.

## 8. Backout procedure

| If stopped at | Do this | End state |
|---|---|---|
| Utility disturbance at any point | Return the selector to auto at once; the unit follows its automatic sequence. Go to the EOP for loss of utility. | Unit in auto |
| Steps 4 to 7, other reason | Stop by the normal method, complete steps 8 to 10 | Unit in auto, or tagged out with capacity restated |

## 9. Records
Run sheet filed. Compare readings with previous runs; a drift is a finding even when inside the normal range.
