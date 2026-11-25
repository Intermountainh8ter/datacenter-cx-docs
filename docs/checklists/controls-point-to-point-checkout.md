---
title: Controls Point-to-Point Checkout Sheet
doc_type: checklist
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Controls Point-to-Point Checkout Sheet

> **Gate:** a completed and accepted point-to-point checkout for a system is a prerequisite for **entry to Level 4** (functional performance testing) of that system. No Level 4 script that depends on a controls or monitoring point starts until every point it uses is checked here and accepted, or is listed as an exception accepted in writing by the commissioning authority. See the [Commissioning Levels One-Pager](../guides/commissioning-levels.md).

Point-to-point checkout proves that each physical point (sensor, status, command, or alarm) is wired, addressed, scaled, named, and displayed correctly, end to end, from the field device to the operator screen and any alarm notification. It is done by the controls integrator and witnessed or sampled by the Cx agent. It is not a functional test: it proves the points are right, so that the functional test can trust them.

Copy one sheet per controller, panel, or equipment item.

| Field | Entry |
|---|---|
| System / equipment and asset tag | |
| Controller or panel ID | |
| Points list reference and revision | |
| Graphics revision | |
| Checked by (controls integrator) / date | |
| Witnessed or sampled by (Cx agent) / date | |
| Sample size if sampled (as agreed in the Cx plan) | |

## A. Before starting

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| A1 | [ ] | Approved points list at the current revision is on hand and matches the installed controller program. | Points list revision; controller program version | Stop; obtain the current list and program before checking |
| A2 | [ ] | Field wiring terminated and labeled; pre-functional checklist for the equipment is complete. | Pre-functional checklist ([template](pre-functional-checklist.md)) | Defer the checkout for this equipment |
| A3 | [ ] | Calibrated reference instruments available for each analog type (temperature, pressure, flow, current, voltage). | Instrument IDs and calibration certificates | Do not check analog points without a reference |
| A4 | [ ] | Method agreed for safely simulating each alarm and status, including any that would trip or start equipment. | Simulation method noted per point type; override log ready | Agree methods with the equipment vendor before continuing |
| A5 | [ ] | Owner's naming convention for points and alarms is available. | Naming standard reference | Note naming deviations as issues rather than stopping |

## B. Point record

Record every point. "Observed" is the value or state actually seen, not "OK".

| # | Point name (as displayed) | Type (AI, AO, DI, DO, network) | Field device / terminal | Test method | Reference value | Value at controller | Value at operator screen | Alarm at screen and notification (where applicable) | Pass/Fail | Init. |
|---|---|---|---|---|---|---|---|---|---|---|
| 1 | | | | | | | | | | |
| 2 | | | | | | | | | | |
| 3 | | | | | | | | | | |

Acceptance for analog points: within the accuracy stated in the project controls specification. [VERIFY: project controls specification sensor accuracy section]

## C. Checks by point type

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| C1 | [ ] | **Analog inputs:** reading compared with the reference instrument at the device; units and scaling correct; reading at the screen matches the controller. | Section B entries with reference values | Log the point; correct scaling or the device, then recheck |
| C2 | [ ] | **Analog outputs:** commanded at several points across the range; the device moves to the commanded position and feedback (if fitted) agrees. | Section B entries | Log the point; check stroke, wiring, and configuration |
| C3 | [ ] | **Digital inputs:** each state created at the field device (not forced in software) and seen correctly at the controller and screen. | Section B entries; method recorded | Log; correct wiring or the normally-open/normally-closed configuration |
| C4 | [ ] | **Digital outputs:** each command issued from the operator screen and the field device observed responding. | Section B entries | Log; check the interposing relay, wiring, and hand-off-auto position |
| C5 | [ ] | **Network points:** each integrated point read from the equipment's own display and compared with the value at the screen. | Section B entries | Log; check mapping, register, and scaling with the equipment vendor |
| C6 | [ ] | **Alarms:** each alarm created at its source; it appears at the screen with the correct name, priority, and time, and reaches the notification path (pager, email, or alarm console) where one is specified. | Section B alarm column; notification records | Log; an alarm that does not notify is a failed point |
| C7 | [ ] | **Loss of communication:** the controller or device is disconnected and the communication-failure alarm appears. | Recorded result | Log; operators must know when they have lost visibility |

## D. Close-out

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| D1 | [ ] | Every point on the points list has a record in Section B. Points not checked are listed with a reason. | Point count in Section B compared with the points list | Complete the missing points or list them as exceptions |
| D2 | [ ] | All overrides, forces, and simulations removed and the override log signed. | Override log | Do not sign off until clear |
| D3 | [ ] | Failed points logged in the [Cx Issues Log](../templates/issues-log.md) and rechecked after correction. | Issue IDs; recheck entries | Keep the sheet open until rechecks pass |
| D4 | [ ] | Graphics, point names, and alarm text match the approved submittal. | Screen captures or a sample review | Log as issues; correct before Level 4 |

## Level 4 entry decision

| Decision | Name | Role | Date |
|---|---|---|---|
| Points accepted; system may enter Level 4 / Not accepted | | | |

| Exception (point # or item #) | Reason | Accepted by | Date |
|---|---|---|---|
| | | | |
