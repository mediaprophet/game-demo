use qualia_core_db::lexicon::generate_60bit_token;
use qualia_core_db::q42::journal::{
    JournalApplicationProfile, JournalHeader, MutableQ42Session, Q42Journal,
};
use qualia_core_db::{NQuin, PermissiveRoutingLane};
use std::io::Cursor;
use wasm_bindgen::prelude::*;

/// Browser-resident offline-first `.q42j` session journal.
#[wasm_bindgen]
pub struct GameSessionJournal {
    session: MutableQ42Session<Cursor<Vec<u8>>>,
    world_did_hash: u64,
    author_did_hash: u64,
    lamport_clock: u64,
    frame_count: u32,
}

#[wasm_bindgen]
impl GameSessionJournal {
    /// Create a new in-memory session journal.
    #[wasm_bindgen(constructor)]
    pub fn new(world_did_hash: f64, author_did_hash: f64) -> Result<GameSessionJournal, JsValue> {
        let w_hash = if world_did_hash.is_finite() && world_did_hash > 0.0 {
            world_did_hash as u64
        } else {
            generate_60bit_token(b"urn:world:kestrel_flats")
        };

        let a_hash = if author_did_hash.is_finite() && author_did_hash > 0.0 {
            author_did_hash as u64
        } else {
            generate_60bit_token(b"did:key:anonymous_surveyor")
        };

        let header = JournalHeader::new(
            JournalApplicationProfile::SimulationWorld,
            w_hash,
            [0u8; 32],
            1,
        );
        let storage = Cursor::new(Vec::with_capacity(4096));
        let journal = Q42Journal::create(storage, header)
            .map_err(|e| JsValue::from_str(&format!("create journal error: {e:?}")))?;

        let session = MutableQ42Session::open(journal, &[])
            .map_err(|e| JsValue::from_str(&format!("open journal session error: {e:?}")))?;

        Ok(Self {
            session,
            world_did_hash: w_hash,
            author_did_hash: a_hash,
            lamport_clock: 1,
            frame_count: 0,
        })
    }

    /// Record a geospatial waypoint / trajectory point as a committed transaction frame.
    pub fn record_waypoint(
        &mut self,
        x: f64,
        _y: f64,
        z: f64,
        _speed: f64,
    ) -> Result<u32, JsValue> {
        self.lamport_clock = self.lamport_clock.saturating_add(1);

        let pred_hash = generate_60bit_token(b"geo:hasWaypoint");
        let x_int = ((x * 100.0) as i32) as u32 as u64;
        let z_int = ((z * 100.0) as i32) as u32 as u64;
        let packed_obj = (x_int << 32) | z_int;

        let lane = PermissiveRoutingLane::SpatiotemporalAmbiguous as u64;
        let meta = (lane << 61) | (self.lamport_clock & 0x1FFF_FFFF);

        let mut quin = NQuin {
            subject: self.author_did_hash,
            predicate: pred_hash,
            object: packed_obj,
            context: self.world_did_hash,
            metadata: meta,
            parity: 0,
        };
        quin.parity = quin.subject ^ quin.predicate ^ quin.object ^ quin.context ^ quin.metadata;

        self.session.advance_tick(self.lamport_clock);
        self.session.stage_add(quin);
        self.session
            .commit_transaction(pred_hash, self.author_did_hash)
            .map_err(|e| JsValue::from_str(&format!("journal commit error: {e:?}")))?;

        self.frame_count = self.frame_count.saturating_add(1);
        Ok(self.frame_count)
    }

    /// Record a field biodiversity observation as a committed transaction frame.
    pub fn record_observation(
        &mut self,
        taxon_iri: &str,
        confidence_permill: u16,
    ) -> Result<u32, JsValue> {
        self.lamport_clock = self.lamport_clock.saturating_add(1);

        let pred_hash = generate_60bit_token(b"dwc:scientificName");
        let taxon_hash = generate_60bit_token(taxon_iri.as_bytes());

        let lane = PermissiveRoutingLane::EnforcePermissiveCommons as u64;
        let meta =
            (lane << 61) | ((confidence_permill as u64) << 32) | (self.lamport_clock & 0x1FFF_FFFF);

        let mut quin = NQuin {
            subject: self.author_did_hash,
            predicate: pred_hash,
            object: taxon_hash,
            context: self.world_did_hash,
            metadata: meta,
            parity: 0,
        };
        quin.parity = quin.subject ^ quin.predicate ^ quin.object ^ quin.context ^ quin.metadata;

        self.session.advance_tick(self.lamport_clock);
        self.session.stage_add(quin);
        self.session
            .commit_transaction(pred_hash, self.author_did_hash)
            .map_err(|e| JsValue::from_str(&format!("journal commit error: {e:?}")))?;

        self.frame_count = self.frame_count.saturating_add(1);
        Ok(self.frame_count)
    }

    /// Export the journal bytes formatted as a standard `.q42j` binary payload.
    pub fn to_bytes(&self) -> Vec<u8> {
        self.session.journal.storage.get_ref().clone()
    }

    /// Returns the total number of committed transaction frames.
    pub fn frame_count(&self) -> u32 {
        self.frame_count
    }
}
