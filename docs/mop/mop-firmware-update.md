---
title: MOP - Firmware or Controller Software Update
doc_type: mop
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# MOP: Firmware or Controller Software Update on Critical Equipment

> Example MOP for updating firmware or control software on equipment such as UPS units, cooling units, power meters, or plant controllers. The vendor's update procedure for the exact model governs.

## 1. Document control
Use the block from the [MOP Template](mop-template.md).

## 2. Purpose and scope
Update one unit, prove it, then update the remaining units in a controlled order. State the current and target versions and the reason for the update.

## 3. Risk and impact

| Question | Answer |
|---|---|
| Systems affected | The unit being updated, and anything that talks to it |
| Redundancy during work | The unit may restart, lose control, or drop off monitoring during the update. Treat it as out of service |
| Worst credible outcome | Update fails and the unit will not restart; or the new version behaves differently and the fault appears on every updated unit |
| Response | Backout below; the EOP for the affected system |

## 4. Pre-site plan
- [ ] Change ticket approved
- [ ] Vendor release notes read: fixes, known issues, changed defaults, required intermediate versions
- [ ] Compatibility confirmed with hardware revision, other units in the group, and the monitoring system's drivers and point maps
- [ ] Update file obtained from the vendor's official source and its integrity check verified
- [ ] Previous firmware file on hand, and the vendor confirms that downgrade is supported
- [ ] Configuration backup method tested: a backup has been taken and proven restorable
- [ ] Update order planned: one unit first, on one side only, with a soak period before the next: ____ hours or days
- [ ] Never both sides of a redundant pair in the same window
- [ ] Vendor technician attending or on call, as the risk level requires
- [ ] Spare controller or board with a known-good version available for the worst case
- [ ] Laptop, cables, and software checked; laptop on mains power
- [ ] Backout reviewed; go/no-go recorded

## 5. Prerequisites (day of work)
- [ ] All units in the group healthy, no alarms; redundant units ready to carry the duty
- [ ] Unit to be updated placed in the safe state the vendor requires (standby, bypass, or offline) under its own steps
- [ ] Monitoring center informed; pre-task briefing held

## 6. Safety
Electrical and mechanical hazards of the equipment still apply when connecting to service ports. Do not open enclosures beyond what the vendor procedure requires.

## 7. Roles and communications
Supervisor, performer (vendor or trained technician), verifier, monitoring contact.

## 8. Procedure steps

| # | Action | Expected response | Reading | Init. | Time |
|---|---|---|---|---|---|
| 1 | Record as-found: firmware version, hardware revision, serial, all settings, alarms, operating mode | Record complete | | | |
| 2 | Export and save the configuration to two locations | Files saved and openable | | | |
| 3 | Confirm redundant units are carrying the duty | Confirmed | | | |
| H1 | **HOLD POINT**: supervisor confirms go | Approval given | | | |
| P | **POINT OF NO RETURN**: update started. Do not interrupt power or communications | | | | |
| 4 | Start the update per the vendor procedure | Progress as described by the vendor | | | |
| 5 | Wait for completion and restart | Unit restarts; reports the target version | | | |
| 6 | Compare every setting with step 1; restore the configuration if the vendor procedure calls for it | Settings match, or changes are explained by release notes and recorded | | | |
| 7 | Run the vendor's post-update checks and a functional check of the unit | Pass | | | |
| 8 | Confirm the monitoring system reads the unit correctly: values, alarms, point names | Correct | | | |
| 9 | Return the unit to service under its own steps | Normal operation | | | |
| 10 | Begin the soak period; record the start time | Unit watched for the agreed period | | | |
| H2 | **HOLD POINT**: after the soak, supervisor approves the next unit | Approval given | | | |
| 11 | Repeat steps 1 to 10 for each remaining unit, one at a time | All units on the target version | | | |

## 9. Backout procedure

**Triggers:** update fails or stalls beyond the vendor's stated time; unit will not restart; settings lost and cannot be restored; functional check fails; monitoring cannot read the unit; unexpected behavior during soak.

| If stopped during steps | Backout steps | Expected end state |
|---|---|---|
| 1 to 3 (before the update) | B1: Do nothing further. Leave the unit on its current version. | As-found |
| 4 to 5 (update in progress or failed) | B2: Do not power-cycle unless the vendor instructs it. Follow the vendor recovery procedure. If recovery fails, fit the spare controller with the previous version and restore the saved configuration. | Unit on the previous version, or out of service with redundancy restated |
| 6 to 10 (updated, but faulty) | B3: Downgrade to the previous version per the vendor procedure; restore the saved configuration; repeat the functional check. | Previous version, settings as step 1 |
| During rollout (some units updated) | B4: Stop the rollout. Decide with the vendor whether mixed versions may run together. If not, downgrade the updated units one at a time. | One consistent, supported set of versions |

## 10. Restoration and verification
- [ ] All units in normal operation; versions recorded per unit
- [ ] Settings verified against the as-found record
- [ ] Monitoring reads all units correctly

## 11. Closeout
- [ ] Asset records updated with the new versions
- [ ] Spare boards and controllers checked for version compatibility; spares register updated
- [ ] New configuration backups stored; change ticket closed
