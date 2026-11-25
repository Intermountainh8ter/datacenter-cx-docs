---
title: Critical Spare Parts Program Guide
doc_type: guide
status: "DRAFT — author review pending"
license: CC BY 4.0
---

# Critical Spare Parts Program Guide

A critical spare is a part whose absence turns a recoverable failure into a long loss of redundancy or an outage. This guide shows how to decide what to stock, how many, where, and how to keep the stock trustworthy.

## Step 1: Build the asset list
Start from the commissioned equipment list. For each asset record: tag, system, manufacturer, model, serial, firmware or hardware revision, redundancy level, and service vendor. Parts are matched to model **and revision**, so capture both.

## Step 2: List candidate parts per asset
Sources, in order of value:
1. The manufacturer's recommended spares list for the exact model
2. The service vendor's failure history for that model
3. Site history: every failure and every part used since commissioning
4. Failure-mode thinking: for each component, ask "if this fails, what stops, and can we restore it without the part?"

## Step 3: Score criticality
Use the [Criticality Scoring Worksheet](criticality-scoring-worksheet.md). The factors:

| Factor | Question |
|---|---|
| Impact | If this part fails, what is lost: nothing, redundancy, or load? |
| Redundancy cover | Does a redundant unit cover the failure, and for how long is running without redundancy acceptable? |
| Lead time | How long to get the part to site, door to door, on the worst day (weekend, holiday, end of production)? |
| Failure likelihood | Is it a wear item, a known weak point, or rarely failing? |
| Fit on site | Can site staff fit it, or is a vendor needed, and how quickly can the vendor arrive? |
| Obsolescence | Is the part still made? Is there a last-buy date? |

## Step 4: Choose a stocking strategy per part

| Strategy | Use when |
|---|---|
| Stock on site | High criticality and lead time longer than the acceptable time at reduced redundancy |
| Vendor-held stock under contract | Vendor must fit the part anyway and guarantees delivery with them inside the acceptable time. Get the guarantee in writing and audit it |
| Pooled across sites | Several sites share the same model and transport between them is inside the acceptable time |
| Order on failure | Low criticality or short lead time |
| Whole-unit spare | Repair on site is impractical and swapping the unit is faster (for example, a spare breaker, fan assembly, or controller) |

## Step 5: Set quantities
- Minimum = enough to recover from one failure of each kind, plus the number likely to fail together (parts of the same age and batch often fail close together).
- Wear items: use the maintenance plan consumption plus a margin for the lead time.
- Identical parts shared by many assets: stock for the population, not per asset.
- Record a minimum and a reorder point for every stocked part. Using a part triggers a reorder the same day.

## Step 6: Typical candidates by system
Generic starting points only. The manufacturer's list for the installed model governs.

| System | Typical candidates |
|---|---|
| UPS | Fans, power modules, control and communication boards, fuses, capacitors, display, static switch parts |
| Batteries | Spare cells, jars, or modules of the same type and age class; monitoring sensors; connectors |
| Generators | Starting batteries and charger, filters (fuel, oil, air), belts, hoses, coolant heater, controller, sensors, governor and regulator parts |
| Fuel system | Transfer pump, filters, level sensors, leak sensors |
| Switchgear and distribution | Spare breaker of each common frame, trip units, control power fuses, relays, charging motors, coils |
| Transfer equipment | Controller, coils, auxiliary contacts |
| Cooling units | Fan assemblies or motors, speed drives, valve actuators, sensors, controller, belts, filters, condensate pump |
| Chilled water and heat rejection | Pump seals and bearings, drive, valve actuators, sensors, starters |
| Monitoring and controls | Controllers, input and output modules, power supplies, network switches, programmed spare with current configuration |
| Fire and leak detection | Detectors, modules, power supplies, per the vendor and code requirements |

## Step 7: Store parts so they work when needed
- Clean, dry, temperature-controlled storage; static protection for electronics.
- Label every part: part number, revision, compatible asset tags, date received, shelf-life date.
- Shelf-life items (batteries, capacitors, belts, hoses, seals, filters with gaskets, lubricants) have a replace-by date. Rotate stock: fit the oldest spare at planned maintenance and store the new one.
- Electronics with firmware: record the version on the label. Keep configuration files and the tools needed to load them with the spare.
- Spare batteries need periodic charging per the manufacturer.
- Large rotating spares may need shaft rotation and heaters in storage.

## Step 8: Control and audit
- One inventory register (see the [Inventory Template](critical-spares-inventory-template.md)).
- Parts leave the store only against a ticket. The ticket triggers the reorder.
- Audit on a fixed cycle using the [Spares Storage and Audit SOP](sop-spares-storage-and-audit.md).
- Review the whole list yearly, and after any failure, equipment change, firmware change, or end-of-life notice.

## Step 9: Tie spares to procedures
- Every MOP pre-site plan confirms the spares for its worst credible failure.
- Every EOP preparedness section lists the spares its event needs.
- IST and commissioning often reveal the first real failures. Start the register then, not after turnover.

## Common mistakes
- Stocking the manufacturer's full list without scoring (money on shelves, gaps where it matters).
- Trusting "vendor holds stock" without a written commitment and an audit.
- Spares that do not match the installed revision after an upgrade.
- Using a spare and not reordering.
- No one owns the register.
