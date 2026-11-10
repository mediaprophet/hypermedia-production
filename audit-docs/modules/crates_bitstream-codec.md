# `crates/bitstream-codec`

> **GENERATED — do not edit.** Regenerate with `audit docs`.
>
> **Agent verification protocol:** before implementing, renaming, or claiming
> a function/module does not exist, grep `FUNCTIONS.md` (and the relevant
> `modules/*.md`) for the name or related terms. If it appears here, it exists
> at the listed `path:line` — read the source before writing anything new.
> Absence from this index means absence from the codebase as of the
> generation timestamp, not a guess.


_Bit-level reading and writing shared by the chronicle media codec crates.

- [`BitReader`]: MSB-first reader with Exp-Golomb (`ue(v)`, `se(v)`), as used by H.264/HEVC/AV1 headers.
- [`BitWriter`]: the matching writer (for encoders and header rewriting).
- [`unescape_rbsp`] / [`escape_rbsp`]: remove/insert H.264/HEVC emulation-prevention bytes.
- [`annexb_nals`] / [`length_prefixed_nals`]: split Annex-B byte streams and AVCC/HVCC samples into NAL units._

| function | arity | location | domains | doc |
|---|---|---|---|---|
| `impl fmt::Display for BitError::fmt` | 2 | `crates/bitstream-codec/src/lib.rs:21` | - |  |
| `impl BitReader<'a>::new` | 1 | `crates/bitstream-codec/src/lib.rs:41` | - |  |
| `impl BitReader<'a>::data` | 1 | `crates/bitstream-codec/src/lib.rs:44` | - |  |
| `impl BitReader<'a>::position` | 1 | `crates/bitstream-codec/src/lib.rs:48` | - | Bit position from the start. |
| `impl BitReader<'a>::bits_left` | 1 | `crates/bitstream-codec/src/lib.rs:51` | - |  |
| `impl BitReader<'a>::is_byte_aligned` | 1 | `crates/bitstream-codec/src/lib.rs:54` | - |  |
| `impl BitReader<'a>::byte_align` | 1 | `crates/bitstream-codec/src/lib.rs:57` | - |  |
| `impl BitReader<'a>::byte_pos` | 1 | `crates/bitstream-codec/src/lib.rs:61` | - | Byte offset of the current position (rounded down). |
| `impl BitReader<'a>::seek_bits` | 2 | `crates/bitstream-codec/src/lib.rs:64` | - |  |
| `impl BitReader<'a>::peek` | 2 | `crates/bitstream-codec/src/lib.rs:70` | - | Peek up to 32 bits without consuming (zero-padded past the end). |
| `impl BitReader<'a>::read_bits` | 2 | `crates/bitstream-codec/src/lib.rs:86` | - |  |
| `impl BitReader<'a>::read_bits_u64` | 2 | `crates/bitstream-codec/src/lib.rs:95` | - |  |
| `impl BitReader<'a>::read_bit` | 1 | `crates/bitstream-codec/src/lib.rs:105` | - |  |
| `impl BitReader<'a>::read_flag` | 1 | `crates/bitstream-codec/src/lib.rs:108` | - |  |
| `impl BitReader<'a>::skip` | 2 | `crates/bitstream-codec/src/lib.rs:112` | - |  |
| `impl BitReader<'a>::read_ue` | 1 | `crates/bitstream-codec/src/lib.rs:121` | - | Unsigned Exp-Golomb `ue(v)`. |
| `impl BitReader<'a>::read_se` | 1 | `crates/bitstream-codec/src/lib.rs:138` | - | Signed Exp-Golomb `se(v)`. |
| `impl BitReader<'a>::read_te` | 2 | `crates/bitstream-codec/src/lib.rs:144` | - | Truncated Exp-Golomb `te(v)` with range `max`. |
| `impl BitReader<'a>::more_rbsp_data` | 1 | `crates/bitstream-codec/src/lib.rs:149` | - | H.264/HEVC `more_rbsp_data()`: true if there is data before the rbsp trailing bits. |
| `impl BitWriter::new` | 0 | `crates/bitstream-codec/src/lib.rs:178` | - |  |
| `impl BitWriter::write_bits` | 3 | `crates/bitstream-codec/src/lib.rs:181` | - |  |
| `impl BitWriter::write_bit` | 2 | `crates/bitstream-codec/src/lib.rs:195` | - |  |
| `impl BitWriter::write_ue` | 2 | `crates/bitstream-codec/src/lib.rs:198` | - |  |
| `impl BitWriter::write_se` | 2 | `crates/bitstream-codec/src/lib.rs:212` | - |  |
| `impl BitWriter::bit_len` | 1 | `crates/bitstream-codec/src/lib.rs:216` | - |  |
| `impl BitWriter::is_byte_aligned` | 1 | `crates/bitstream-codec/src/lib.rs:219` | - |  |
| `impl BitWriter::align_zero` | 1 | `crates/bitstream-codec/src/lib.rs:223` | - | Pad with zero bits to a byte boundary. |
| `impl BitWriter::rbsp_trailing` | 1 | `crates/bitstream-codec/src/lib.rs:229` | - | `rbsp_trailing_bits()`: a one bit then zeros to alignment. |
| `impl BitWriter::write_bytes` | 2 | `crates/bitstream-codec/src/lib.rs:233` | - |  |
| `impl BitWriter::finish` | 1 | `crates/bitstream-codec/src/lib.rs:242` | - |  |
| `impl BitWriter::bytes` | 1 | `crates/bitstream-codec/src/lib.rs:246` | - |  |
| `unescape_rbsp` | 1 | `crates/bitstream-codec/src/lib.rs:252` | - | Remove emulation-prevention bytes (`00 00 03` → `00 00`) from a NAL payload. |
| `escape_rbsp` | 1 | `crates/bitstream-codec/src/lib.rs:267` | - | Insert emulation-prevention bytes so no `00 00 0x` (x ≤ 3) sequence appears. |
| `annexb_nals` | 1 | `crates/bitstream-codec/src/lib.rs:282` | - | Split an Annex-B byte stream (start codes `00 00 01` / `00 00 00 01`) into NAL units (escaped payloads). |
| `length_prefixed_nals` | 2 | `crates/bitstream-codec/src/lib.rs:308` | - | Split a length-prefixed sample (AVCC/HVCC, `length_size` 1/2/4 bytes) into NAL units. |
| `crc32` | 1 | `crates/bitstream-codec/src/lib.rs:330` | - | CRC-32 (IEEE, reflected) — used by containers (e.g. Matroska CRC-32 elements, PNG). |
| `reads_bits` | 0 | `crates/bitstream-codec/src/lib.rs:347` | - |  |
| `exp_golomb_known` | 0 | `crates/bitstream-codec/src/lib.rs:358` | - |  |
| `emulation_prevention` | 0 | `crates/bitstream-codec/src/lib.rs:381` | - |  |
| `annexb_split` | 0 | `crates/bitstream-codec/src/lib.rs:389` | - |  |
| `more_rbsp` | 0 | `crates/bitstream-codec/src/lib.rs:398` | - |  |
| `crc` | 0 | `crates/bitstream-codec/src/lib.rs:407` | - |  |
