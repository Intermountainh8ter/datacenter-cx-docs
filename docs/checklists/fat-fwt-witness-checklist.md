---
title: Factory Acceptance / Factory Witness Test Checklist
doc_type: checklist
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Factory Acceptance / Factory Witness Test (FAT / FWT) Checklist

For the owner's representative or Cx agent attending a factory test (Level 1) of major equipment: switchgear, UPS modules, generators and paralleling controls, transfer switches, cooling units, or coolant distribution units. A factory acceptance test (FAT) is the manufacturer's test against the purchase specification; a factory witness test (FWT) is that test, or an agreed part of it, observed by the buyer. The aim is to find problems where they are cheapest to fix, and to come home with evidence, not a brochure.

Test procedures, acceptance values, and test lists come from the purchase specification, the approved submittals, the manufacturer's standard test procedure, and the product standard for the equipment. Product standards to check, where they apply: [VERIFY: IEEE C37.20.2 production test section] for metal-clad switchgear, [VERIFY: IEC 62040-3 routine and type test sections] for UPS, [VERIFY: ISO 8528 factory test section] for generating sets.

| Field | Entry |
|---|---|
| Equipment type, model, and serial numbers | |
| Purchase order and specification reference (generic identifier only) | |
| Approved submittal revision | |
| Factory test procedure reference and revision | |
| Test dates | |
| Witnesses (name, company, role) | |

## A. Before travelling

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| A1 | [ ] | The manufacturer's test procedure was received in advance, reviewed against the specification, and comments resolved in writing. | Procedure revision with comment log | Do not travel until the procedure is agreed; a test against an unreviewed procedure proves only what the factory chose |
| A2 | [ ] | Which tests are witnessed, and which are reported only, is agreed in writing. | Witness point list | Agree before travel |
| A3 | [ ] | Approved submittals, the sequence of operations, settings files, and drawings are available for reference at the factory. | Document list | Bring copies; note gaps |
| A4 | [ ] | The factory has confirmed the equipment will be fully assembled and ready, with the test equipment available and calibrated. | Written readiness confirmation | Reschedule rather than witness an incomplete unit |
| A5 | [ ] | Factory safety rules, PPE, and site access requirements are known. | Factory safety briefing details | Arrange PPE and inductions before travel |

## B. On arrival

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| B1 | [ ] | Nameplates and serial numbers match the purchase order and submittals. | Photographs of nameplates | Record the discrepancy; raise with the manufacturer before testing |
| B2 | [ ] | Test instrument calibration certificates are current. | Certificate list with due dates | Instruments out of calibration cannot produce acceptance data; stop and replace |
| B3 | [ ] | The configuration under test (firmware, settings, options) matches the approved submittal. | Firmware versions and settings printout | Record; agree whether testing proceeds on this configuration |
| B4 | [ ] | Factory pre-test (internal) records exist and show the unit already passed the manufacturer's own checks. | Internal test sheets | Ask why not; a witness test should not be the first time the unit is run |

## C. Construction and inspection

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| C1 | [ ] | Physical arrangement, dimensions, access, and clearances match the approved drawings. | Marked-up drawings; photographs | Log deviations in the punch list |
| C2 | [ ] | Wiring, terminations, labeling, and component ratings match the submittals. | Sample inspection record | Log in the punch list |
| C3 | [ ] | Workmanship: no damage, debris, loose hardware, missing covers, or temporary jumpers. | Inspection notes; photographs | Log; require correction before shipment |
| C4 | [ ] | Spare parts, special tools, and documentation that ship with the unit are identified. | Shipping list | Record what is missing |

## D. Witnessed tests

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| D1 | [ ] | Each agreed witness test was observed in full, with readings taken at the time, not copied later from the factory's sheet. | Witness's own readings or signed factory sheets | A test not observed is recorded as "reported only" |
| D2 | [ ] | Insulation and dielectric tests performed per the procedure and the product standard. | Recorded values | Record as failed; require cause and retest |
| D3 | [ ] | Functional and sequence tests: every mode in the sequence of operations available at the factory is exercised (normal, alarm, failure, manual, maintenance). | Step-by-step results | Log failures; decide which must be retested before shipment |
| D4 | [ ] | Load or performance tests at the loads stated in the specification, with the duration recorded. | Load and duration records | Record any reduced load or shortened duration as a deviation |
| D5 | [ ] | Protective functions tested by injection or simulation, and their settings recorded. | Settings printout; test results | Log |
| D6 | [ ] | Communications to the monitoring system tested with the agreed protocol and points list, where a simulator is available. | Point map and test results | Record as deferred to site, with an owner |
| D7 | [ ] | Any failed test has a root cause stated by the manufacturer before it is repeated, and the repeat is witnessed. | Failure record, cause, retest result | A pass on repeat with no cause is recorded as an open issue, not a pass |

## E. Close-out at the factory

| # | Done | Check | Evidence | Fail action |
|---|---|---|---|---|
| E1 | [ ] | Punch list written, agreed, and signed by the manufacturer before leaving, with each item marked "before shipment" or "on site". | Signed punch list | Do not leave without a signed list |
| E2 | [ ] | Tests deferred to site are listed, with the reason and the site test that will cover them. | Deferral list | Add to the Cx issues log |
| E3 | [ ] | Release to ship is withheld until "before shipment" items are closed and evidenced. | Written release, or written hold | Notify the project team of the hold |
| E4 | [ ] | Final test report, with all recorded values and calibration certificates, is required as a condition of release. | Report due date agreed | Track as an open item |
| E5 | [ ] | Shipping protection, lifting points, and storage requirements confirmed. | Shipping and storage instructions | Record; tell the receiving party |

## Witness record

| Role | Name | Company | Signature | Date |
|---|---|---|---|---|
| Owner's witness | | | | |
| Cx agent | | | | |
| Manufacturer's test lead | | | | |

| Punch item | Before shipment / On site | Owner | Closed (date and evidence) |
|---|---|---|---|
| | | | |
