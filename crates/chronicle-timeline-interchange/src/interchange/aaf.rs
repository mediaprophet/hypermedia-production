//! Advanced Authoring Format (AAF) interchange model.
//!
//! Grounded in AMWA AAF Object Specification v1.1 and AAF Edit Protocol v1.0.

use serde::{Deserialize, Serialize};

use crate::model::{TimelineClip, TimelineSequence, TimelineTrack, TrackKind};
use crate::time::{FrameRate, Tick};

/// AAF Unique Material Identifier (UMID) or Identification.
#[derive(Clone, Copy, PartialEq, Eq, Hash, Default, Serialize, Deserialize)]
pub struct MobId(pub [u8; 32]);

impl std::fmt::Debug for MobId {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "urn:smpte:umid:")?;
        for b in &self.0 {
            write!(f, "{b:02x}")?;
        }
        Ok(())
    }
}

/// An AAF Mob (Metadata Object).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Mob {
    Composition(CompositionMob),
    Master(MasterMob),
    Source(SourceMob),
}

/// A top-level or nested timeline composition mob.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct CompositionMob {
    pub id: MobId,
    pub name: String,
    pub slots: Vec<TimelineMobSlot>,
}

/// Master mob connecting editorial clips to physical media.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct MasterMob {
    pub id: MobId,
    pub name: String,
    pub slots: Vec<TimelineMobSlot>,
}

/// Source mob describing essence format and storage path.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct SourceMob {
    pub id: MobId,
    pub name: String,
    pub locator_path: String,
}

/// A track slot on an AAF Mob.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct TimelineMobSlot {
    pub slot_id: u32,
    pub slot_name: String,
    pub edit_rate: FrameRate,
    pub origin: i64,
    pub segment: Segment,
}

/// A segment inside an AAF track slot.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub enum Segment {
    Sequence(AafSequence),
    SourceClip(AafSourceClip),
    Filler(AafFiller),
    Transition(AafTransition),
}

impl Default for Segment {
    fn default() -> Self {
        Segment::Filler(AafFiller { length: 0 })
    }
}

/// An ordered sequence of components.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AafSequence {
    pub length: i64,
    pub components: Vec<Segment>,
}

/// Reference to source mob material.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AafSourceClip {
    pub length: i64,
    pub start_time: i64,
    pub source_mob_id: MobId,
    pub source_slot_id: u32,
}

/// A timeline gap or empty space.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AafFiller {
    pub length: i64,
}

/// Overlapping transition between components.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AafTransition {
    pub length: i64,
    pub cut_point: i64,
    pub effect_name: String,
}

/// An AAF interchange document container.
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct AafDocument {
    pub compositions: Vec<CompositionMob>,
    pub master_mobs: Vec<MasterMob>,
    pub source_mobs: Vec<SourceMob>,
}

impl AafDocument {
    /// Convert a TimelineSequence into an AAF Document model.
    pub fn from_sequence(seq: &TimelineSequence) -> Self {
        let mut doc = Self::default();
        let mut comp = CompositionMob {
            id: MobId([1u8; 32]),
            name: seq.name.clone(),
            slots: Vec::new(),
        };

        let mut slot_id = 1u32;
        for track in &seq.video_tracks {
            let mut seq_seg = AafSequence::default();
            for clip in &track.clips {
                let dur_frames = seq.frame_rate.frame_at(clip.timeline_duration);
                let in_frames = seq.frame_rate.frame_at(clip.source_in);
                seq_seg.length += dur_frames;
                seq_seg.components.push(Segment::SourceClip(AafSourceClip {
                    length: dur_frames,
                    start_time: in_frames,
                    source_mob_id: MobId([2u8; 32]),
                    source_slot_id: 1,
                }));
            }
            comp.slots.push(TimelineMobSlot {
                slot_id,
                slot_name: track.name.clone(),
                edit_rate: seq.frame_rate,
                origin: 0,
                segment: Segment::Sequence(seq_seg),
            });
            slot_id += 1;
        }

        doc.compositions.push(comp);
        doc
    }

    /// Convert this AAF Document model into a TimelineSequence.
    pub fn to_sequence(&self) -> Option<TimelineSequence> {
        let comp = self.compositions.first()?;
        let mut seq = TimelineSequence::default();
        seq.name = comp.name.clone();

        for slot in &comp.slots {
            let mut track = TimelineTrack::new(&slot.slot_name, TrackKind::Video);
            let mut cur_time = Tick::ZERO;

            if let Segment::Sequence(seq_seg) = &slot.segment {
                for comp_item in &seq_seg.components {
                    match comp_item {
                        Segment::SourceClip(sc) => {
                            let dur_tick = slot.edit_rate.tick_of(sc.length);
                            track.clips.push(TimelineClip {
                                id: format!("clip_{}", track.clips.len() + 1),
                                name: "Clip".into(),
                                reel: String::new(),
                                source_path: String::new(),
                                timeline_start: cur_time,
                                timeline_duration: dur_tick,
                                source_in: slot.edit_rate.tick_of(sc.start_time),
                                source_out: slot.edit_rate.tick_of(sc.start_time + sc.length),
                                speed: 1.0,
                            });
                            cur_time = cur_time + dur_tick;
                        }
                        Segment::Filler(f) => {
                            cur_time = cur_time + slot.edit_rate.tick_of(f.length);
                        }
                        _ => {}
                    }
                }
            }
            seq.video_tracks.push(track);
        }

        Some(seq)
    }
}
