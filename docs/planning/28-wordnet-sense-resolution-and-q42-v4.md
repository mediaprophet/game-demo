# WordNet vocabulary and Q42 v4 sense resolution

## Current source

Use the local copy the project owner identified:

- `C:/github/ontology/english-wordnet-2025-plus/english-wordnet-2025-plus.q42`
- `C:/github/ontology/english-wordnet-2025-plus/english-wordnet-2025-plus.ttl`

The Q42 volume header currently declares volume version 3. Its embedded Q42LEX dictionary declares format version 4 and contains 1,732,603 entries. Do not confuse the Q42 volume generation with the Q42LEX format generation. The Princeton WordNet 3.1 copy at `C:/github/qualiaDB/docs/playground/wordnet.q42` is older and its Q42LEX section has no usable entries.

## Game integration already present

`web/fixtures/investment-taxonomy.json` defines seven socioeconomic resource-investment categories and maps game actions to them. `web/game.html` replays accepted actions and records who paid, who benefited, money/time changes, and observed outcomes. The journal identifies the Open English WordNet 2025+ source and Q42LEX v4.

The category descriptions and causal rules are game ontology. WordNet supplies lexical terms, parts of speech, synsets, glosses, and lexical relationships; it does not prescribe policy meaning or assign moral character.

## Remaining generic QualiaDB capability

The native `Q42RangeVolume` API already supports bounded object-index and field-index query plans against Q42 volumes, including FIDX/PIDX pruning. The game WASM shell does not expose this reader together with Q42LEX v4 term resolution. Add a generic, read-only, bounded WASM interface that:

1. Opens a Q42 range source without reading the whole volume into browser memory.
2. Resolves lexical hashes through Q42LEX v4, including namespaced page entries.
3. Runs bounded subject, predicate, object, and joined pattern queries using BIDX/FIDX/PIDX.
4. Returns paged Quins with resolved lexical terms and the volume/lexicon version and source digest.
5. Fails clearly when a requested term or sense cannot be resolved; never silently picks one among multiple senses.

This is general Q42 query and lexicon functionality. It belongs in the QualiaDB engine/WASM profile and should be tested against WordNet and at least one unrelated Q42 volume. The QualiaDB repository remains read-only under the current task constraints; no QDB source was changed here.

## Game-side acceptance once the generic interface is available

- Validate every candidate lemma and part of speech against Q42LEX.
- Resolve all matching senses and retain each stable synset IRI, POS, gloss, and provenance.
- Preserve polysemy as a list. Associate a category with a sense only when the category wording supports that choice; leave ambiguous candidates unresolved for review.
- Keep missing terms visible as unresolved vocabulary entries rather than dropping them or substituting another ontology.
- Verify the result reproducibly from the source Q42 digest and add assertions for representative money, work, cost, benefit, risk, capacity, and governance senses.
