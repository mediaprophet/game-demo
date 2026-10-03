# Maslows Challenge: Trading Posts, Parts, and Logistics

## Purpose

The **Trading Post** makes repair, travel, and community exchange feel lived-in.
A player might find a second-hand solar blanket on a local noticeboard, compare
it with a new panel, discover that a compatible battery is listed in another
town, or arrange freight for a caravan component to a community ground with a
safe receiving point. The result is a story and a logistical decision, not a
generic shop menu.

Trading is local-first and extensible. v0.1 uses curated scenario listings and
NPC/community-board interactions; it does not connect players to real sellers,
payments, shipping, or public marketplaces.

## Trading-post experience

```text
arrive at a local/community Trading Post
  → browse New, Second-hand, Community Exchange, and Services sections
  → inspect listing provenance, condition, compatibility, location, and terms
  → reserve / enquire / offer swap / accept donation / arrange fulfilment
  → travel to collect, or organise simulated freight to a receiving point
  → inspect/accept item and add it to inventory or a maker project
  → repair, install, inspect, share, resell, donate, or document the item history
```

In a storybook/RPG view this can be a noticeboard, workshop counter, market
table, or radio/data-terminal. In an illustrated-adventure view it becomes a
scene with pinned notices, seller/mentor dialogue, maps, and delivery choices.
All views use the same semantic listings and command paths.

## Listing and catalogue model

The catalogue is a vocabulary plus pack-defined specialisations, rather than a
fixed list of item names.

| Entity | Required concepts |
|---|---|
| `TradeBoard` | Ground/location, visibility, available sections, steward, refresh/version |
| `Listing` | Offering type, status, price/value terms, condition, quantity, location, availability, source/provenance |
| `TradeItem` | Category, model/specification abstraction, dimensions/units where needed, condition, compatible targets, visual asset |
| `VehicleOrDwelling` | Vehicle, camper, van, bus, caravan, 4WD, demountable home, or equipment target |
| `PartOrEquipment` | Component, tool, camping equipment, solar blanket/panel, battery, storage/water module, accessory |
| `Fulfilment` | Local collection, community transfer, freight, overseas shipping, delivery, receiving point, expected scenario delay/cost |
| `ReceivingPoint` | Party/dwelling, workshop, ground node, partner venue, or nominated secure community address/node |
| `TransactionRecord` | Reservation, exchange, donation, loan, purchase, sale, inspection, acceptance, return/dispute outcome |

Listings support **new**, **second-hand**, **community surplus**, **swap**,
**donation**, **loan**, and **service/repair help**. Each category has different
price, condition, provenance, availability, trust, and logistics implications.
Second-hand does not mean inferior: repairability, documentation, reuse value,
compatibility, and condition can make it the better choice.

Listings may include a scenario maintenance/service history, stated fault,
inspection result, repairability, and condition uncertainty. These facts flow
into a player asset's maintenance record after acceptance, so buying used items
creates informed repair opportunities rather than hidden penalties.

## Location and fulfilment

Listings are anchored to different community grounds, towns, partner nodes, or
pack-defined overseas origins. A player must select an appropriate simulated
fulfilment path:

- collect locally or travel to another node;
- transfer through a participating community ground;
- use domestic freight to a receiving point; or
- use an abstract international shipping route to a receiving point with
  sufficient access, storage, agreement, and scenario capacity.

International shipping is an economic/logistics abstraction: the game can model
cost, time, availability, uncertainty, route capacity, and a pack-defined
administrative/receiving requirement. It does not advise on customs, import law,
payments, shipping contracts, dangerous goods, or real-world delivery.

## Compatibility, condition, and trust

The simulation evaluates listings against semantic requirements, not a hidden
“recommended item” score. A part may require a compatible target, a qualified
inspection/supervised learning path, a tool, a project stage, and a receiving
point. Condition is represented as scenario evidence such as unknown, usable,
needs inspection, repairable, or unsuitable for a declared project.

Every listing and transaction carries provenance: source board, author/seller
role, pack version, listing status, stated condition, evidence/inspection result,
and subsequent repair/install history. The game can say *why* an item cannot
currently be installed and offer alternatives: travel, delivery, inspection,
repair, a substitute, mentoring, or another listing.

## Vehicle/camper readiness and camper yards

An acquired car, camper, van, bus, caravan, or 4WD has a separate simulated
mobility state: `transported`, `stored`, `needsInspection`, `needsRepair`,
`yardWork`, `readyForScenarioRoadUse`, `restricted`, or `retired`. It is not
assumed usable on ordinary public roads or standard community-ground access just
because a player owns it. A region/scenario pack defines game-policy requirements
for road use and site access; this is not a statement of real registration or
roadworthiness law.

The player can move an unready vehicle only through allowed simulated paths,
such as transported delivery, secure storage, or a **camper yard**. A camper yard
is a specialised community-ground node inspired by vessel yards: it can provide
storage, repair bays, sheds, tools, parts receiving, shared services, inspection
appointments, and professional/mentor assistance. Its use requires capacity,
booking/access agreement, a project plan, and sometimes a scenario fee.

```text
acquire vehicle → delivery/collection → store or place in camper yard
  → inspect condition → create repair/readiness project
  → source parts + learn/collaborate or book paid professional help
  → validated simulated inspection/readiness result
  → ordinary road/site access becomes available under the pack's rules
```

Professionals are role-based service listings with declared scope, availability,
fee/value terms, and project evidence. They can complete or supervise game tasks;
they never represent a real authority, certification, inspection, or legal
roadworthiness decision.

## Economics and community benefit

Trading Post actions settle from a person's/consented party's wallet or, when a
committee-approved shared purchase is selected, a committee-owned treasury.
They flow into person/party, project, and community-ground ledgers. They can
model value/cost, travel/freight, time, storage, community
exchange credit, contribution/donation, repair labour, warranty-like scenario
terms, and reuse/material benefit. Values are scenario parameters, not current
market prices or financial advice.

A purchase from a treasury must carry the treasury fund, purpose, and approval
receipt required by the committee policy; a player contribution does not let
them spend the treasury. Completed simulated transactions create balanced,
provenance-linked postings rather than directly editing a displayed balance.

A community can operate a surplus shelf, parts library, tool loan pool, repair
day, or exchange board. These reduce waste and enable repairs, but require
stewardship, storage, inspection, agreements, and recordkeeping.

## Logic and safety boundaries

Core predicates include `listedOn`, `hasListingType`, `hasCondition`,
`compatibleWith`, `locatedAt`, `hasFulfilmentOption`, `requiresReceivingPoint`,
`hasReservation`, `requiresInspection`, `mayTransferTo`, and `acceptedBy`.

Rules validate a simulated listing/reservation/fulfilment path and can hold it
for missing capacity, consent, storage, route, compatibility, inspection, or
project requirements. They cannot assert real product safety, suitability,
seller trustworthiness, payment validity, shipping legality, or electrical/
vehicle compliance.

Mobility/camper-yard predicates include `hasMobilityState`, `requiresInspection`,
`requiresRepair`, `mayStoreAt`, `mayUseYard`, `hasYardCapacity`,
`hasServiceListing`, `requiresBooking`, and `permitsScenarioRoadUse`. Rules can
gate simulated travel/site access, reserve a yard bay, price a service, or explain
a missing repair pathway. They cannot certify an actual vehicle or establish
real registration, roadworthiness, insurance, or legal site access.

## v0.1 proof

v0.1 ships one local community board and one remote board. It includes new and
second-hand sections, a donated/swap item, a service listing, a vehicle/camper
part, camping equipment, a solar item, and a battery item. The player must
complete one local collection and one simulated remote fulfilment to a nominated
community-ground receiving point, then use an accepted item in a validated maker
project. It also includes one unready camper/vehicle moved into a camper yard,
then made ready through inspection, repair, and a paid/supervised service path.
Later packs may add further boards, regions, and abstract international routes
under the same catalogue/fulfilment contract.
