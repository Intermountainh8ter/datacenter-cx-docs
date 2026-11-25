---
title: Level 5 Verification Checklist (Owner)
doc_type: checklist
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Level 5 Verification Checklist (Owner)

For the owner or owner's representative, after integrated systems testing (Level 5) is reported complete and before the IST report is accepted. Its purpose is to confirm that Level 5 was actually delivered as planned, not just reported. Work through it with the IST plan, the completed scripts, the trend data, and the issues log open side by side.

An unchecked item is not automatically a failure, but each one needs a written disposition: retest, accept with stated risk, or reject the report. "Level" here follows the project's commissioning plan; see the [Commissioning Levels One-Pager](../guides/commissioning-levels.md).

| Field | Entry |
|---|---|
| Facility / phase (generic identifier only) | |
| IST plan reference and revision | |
| IST report reference and revision | |
| Reviewed by / date | |

## A. Scope delivered

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| A1 | [ ] | Every scenario in the approved IST plan has a completed script. No scenario was dropped, merged, or renamed without a written change approved by the owner. | IST plan scenario list compared line by line with the completed scripts | List each missing scenario; the report is not accepted until it is run or formally removed by the owner |
| A2 | [ ] | Each script that was run is the approved revision, or a later revision whose changes were approved before the test. | Script revision blocks and approval records | Treat red-lined, unapproved changes as a deviation; review whether the change weakened the test |
| A3 | [ ] | Deferred scenarios (for example, seasonal or utility-dependent tests) have an owner, a date, and a written acceptance of the deferral. | Deferral log entries | Add the deferral to the issues log with an owner and date before acceptance |

## B. Load: was it a real Level 5?

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| B1 | [ ] | **Partial-load IST.** Each scenario ran at the load stated in the IST plan (normally design load). Any scenario run at reduced load is identified in the report, with the load used and the reason. | Load bank logs and electrical trends for the period of each scenario, compared with the plan | Treat any undisclosed partial-load scenario as not run. Decide in writing: rerun at the planned load, or accept with the specific risk stated (for example, transfer and cooling restart not proven at design heat load) |
| B2 | [ ] | Load was distributed as planned (by zone, row, or cabinet position), not concentrated where it was easiest to connect. | Load bank placement record or drawing; thermal trends by zone | Identify the zones that were not loaded; record what was not proven there |
| B3 | [ ] | Steady state at the planned load was held for the planned period before each failure scenario began. | Trends showing stable temperatures and loads before the event timestamp | Results from a scenario started before steady state may not reflect real behavior; decide whether to rerun |
| B4 | [ ] | Mechanical systems saw a real heat load. Cooling results were not taken with electrical load banks rejecting heat outside the space under test. | Load bank type and location; supply and return temperature trends | Record cooling results as unproven at design heat load |

## C. Witnessing and evidence

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| C1 | [ ] | **Unwitnessed tests.** Every scenario was witnessed by the owner's representative or a named delegate, as required by the commissioning plan. Unwitnessed runs are listed in the report, not mixed in with witnessed results. | Signed witness block on each script; attendance records | Do not accept an unwitnessed scenario on the contractor's report alone. Rerun witnessed, or accept in writing with the reason recorded |
| C2 | [ ] | Observed values are recorded as numbers or observations, not "OK", "pass", or a tick. | Completed script tables | Request the underlying data; if none exists, treat the step as unrecorded |
| C3 | [ ] | Trend and alarm data from the monitoring system covers each scenario, is time-synchronized, and agrees with the script times. | Exported trends and event logs with timestamps | Investigate any gap or time offset; a gap during a failure event means the event is not evidenced |
| C4 | [ ] | Power quality recordings exist for the transfer events in the plan. | Recorder files referenced in the report | Record transfer performance as unproven where no recording exists |
| C5 | [ ] | The witness for each scenario was not the party whose work was under test. | Names and companies on the witness blocks | Raise with the commissioning authority; consider a rerun with an independent witness |

## D. Failures and retests

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| D1 | [ ] | Every failed step or aborted scenario appears in the issues log with an ID. | Script fail entries matched to issues log IDs | Add the missing issues before acceptance |
| D2 | [ ] | **Retests without root cause.** Each retest that followed a failure has a recorded root cause and correction before the retest date. A pass on retest with no recorded cause ("could not reproduce", "reset and retried") is flagged. | Issues log root cause, verification, and retest date fields (see the [Cx Issues Log Template](../templates/issues-log.md)) | Keep the issue open. Require the cause to be found, or a written acceptance of the residual risk by the owner's representative. An unexplained failure that passed on retry can recur in operation |
| D3 | [ ] | Retests repeated the full scenario, not just the step that failed. | Retest script showing all steps re-recorded | Record which steps were not repeated and decide whether the partial retest is acceptable |
| D4 | [ ] | Where a correction changed a setting, sequence, or component, the affected Level 4 functional test was repeated before the Level 5 retest. | Level 4 retest records dated before the Level 5 retest | Request the Level 4 retest; the Level 5 result may rest on an unverified change |
| D5 | [ ] | No more than the planned number of attempts was made at any scenario without a stop and review. | Script attempt counts and dates | Repeated attempts until one passes suggest an intermittent fault; treat as D2 |

## E. Final state and handover

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| E1 | [ ] | Every override, forced point, jumper, and temporary setting used during IST is recorded as removed. | Signed override log | Do not accept until the log is clear and spot-checked on site |
| E2 | [ ] | As-left settings match the approved settings, and any setting changed during IST is reflected in the final sequence of operations and settings record. | As-left record compared with the approved settings | Update the records or restore the settings; log as an issue |
| E3 | [ ] | The report gives the operations baseline: start times, transfer times, restart times, and time-to-limit from the heat-rise test. | Baseline section of the IST report | Request the baseline; operations staff need it for their emergency procedures |
| E4 | [ ] | Open items at handover are listed with owner, severity, and date, and each is accepted in writing. | Final issues log and acceptance record | Hold acceptance until each open item has a disposition |
| E5 | [ ] | Operations staff attended IST and the attendance is recorded. | Attendance sheets | Schedule a walk-through of the IST results with operations before handover |

## Decision

| Decision | Name | Role | Date |
|---|---|---|---|
| Accept IST report / Accept with listed exceptions / Reject | | | |

| Exception (item #) | Disposition | Risk accepted by | Date |
|---|---|---|---|
| | | | |
