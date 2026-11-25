---
title: EOP - Loss of Utility Power
doc_type: eop
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# EOP: Loss of Utility Power

> Example EOP for a generic facility with UPS and standby generators that start and transfer automatically. Replace with the site's sequence of operations.

## 1. Document control
Use the block from the [EOP Template](eop-template.md).

## 2. Entry conditions
Utility source alarm, generators starting, UPS on battery, or lights transferring to emergency.

## 3. Pre-site plan (preparedness)
- [ ] Sequence of operations for utility loss known by every operator
- [ ] Generators exercised per the SOP; fuel above the site's minimum
- [ ] Fuel supplier emergency delivery arrangement in place and tested
- [ ] Utility emergency contact and account details posted
- [ ] Generator and switchgear vendor emergency contacts current; site access and briefing arrangements agreed in advance
- [ ] Manual start and manual transfer steps posted at the equipment
- [ ] Spares for starting systems (batteries, chargers) in stock

## 4. Immediate actions

| # | Action | Expected response |
|---|---|---|
| 1 | Confirm the loss on two indications (utility metering, UPS status) | Loss confirmed |
| 2 | Verify UPS is carrying the critical load | Load on battery or already on generator |
| 3 | Verify generators started and reached voltage and frequency | Required number running |
| 4 | Verify transfer to generator | Load on generator; UPS back on its rectifier and recharging |
| 5 | Verify cooling restarted | Units and plant running |
| 6 | Notify the shift lead and monitoring center | Acknowledged |

**If generators did not start or transfer did not occur:** follow the posted manual start and transfer steps for the equipment. Note the UPS battery time remaining and tell the shift lead at once.

## 5. Follow-up actions

| # | Action | Expected response | Time |
|---|---|---|---|
| 1 | Record generator loads, fuel levels, and UPS status | Recorded | |
| 2 | Calculate fuel run time at current load; call for delivery if the site's threshold is reached | Delivery confirmed | |
| 3 | Contact the utility for cause and estimated restoration | Logged | |
| 4 | Walk the generators and fuel system at set intervals | No leaks or alarms | |
| 5 | Watch room temperatures until cooling is stable | Within limits | |
| 6 | Suspend all non-essential work and open change windows | Work made safe | |

## 6. Escalation and communication

| When | Who is told | By whom |
|---|---|---|
| At once | Shift lead, facility manager | Operator |
| Generator fault or reduced standby redundancy | Owner, generator vendor | Facility manager |
| Fuel below the site's threshold | Fuel supplier, owner | Facility manager |

## 7. Do not
- Do not transfer back to utility manually on first return of power. Wait for the stable period in the design.
- Do not reset tripped breakers without knowing why they tripped.
- Do not start planned work while on generator.

## 8. Backout procedure (return to normal)
- **Conditions:** utility confirmed stable by the utility and by site metering for the period in the design; no generator or switchgear alarms that affect retransfer.
- **Steps:** allow the automatic retransfer, or perform the manual retransfer by the site procedure, with an operator at the switchgear and at the UPS. Expected: load on utility without interruption, generators run their cool-down and stop, selectors in auto.
- **If retransfer fails:** remain on generator, do not retry without diagnosis, call the switchgear vendor.
- **Verification:** normal lineup on the one-line, UPS on rectifier with batteries recharging, generators in auto, cooling in normal mode.

## 9. After the event
- [ ] Fuel refilled; generator post-run checks done
- [ ] Battery recharge confirmed
- [ ] Event log and trend data saved
- [ ] Post-event review held
