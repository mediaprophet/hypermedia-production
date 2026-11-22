//! Detached, aggregate-only check of a host-private authenticated cohort packet.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.
//!
//! The trusted native operator supplies an absolute local file path via
//! `SYNTH_MPC_COHORT_VERIFICATION_INPUT`, the lowercase SHA-256 pin of the
//! authority's raw Ed25519 public key via `SYNTH_MPC_COHORT_AUTHORITY_SHA256`,
//! and the lowercase release digest via `SYNTH_MPC_COHORT_RELEASE_SHA256`.
//! These settings are outside Vibe and the HMC retained-input archive.
//! Keep the packet and its parent directories under the trusted operator's
//! control, outside `SYNTH_INPUT_ROOT`, `SYNTH_OUTPUT_ROOT`, and every
//! retained or public asset directory. Release approval applies to the
//! native process. Any caller allowed to run an AssetHost invocation in
//! that process can request the pinned aggregate with its expected epoch.
//! Changes to the pinned release across queries need release accounting.
//!
//! V1 wire layout, concatenated with no padding or trailing bytes:
//! `b"Q42-MPC-COHORT-VERIFICATION-V1\0"`, authority public key (32 bytes),
//! epoch digest (32), anonymity floor (u64 LE), commitment salt (32),
//! Ed25519 policy signature (64), party 0, party 1, then each party's
//! detached authority signature (64 bytes, party 0 followed by party 1).
//! Each party is `role:u8 || alpha_share:u64 LE || row_count:u64 LE || rows`.
//! Each row is `tag:[u8;32] || value_share:u64 LE || mac_share:u64 LE`.
//! Rows must be strictly sorted by tag in each table. All field elements
//! use their canonical unsigned representative in `[0, 2^61-1)`.
//!
//! All digests are SHA-256 over the following exact concatenations. Domain
//! strings shown in this list have no terminator. The `party` bytes include
//! the role, alpha share, row count and complete row array.
//!
//! - Commitment = `Q42-MPC-COHORT-MAC-COMMITMENT-V1 || epoch || salt || alpha_0 LE || alpha_1 LE`.
//! - Policy = `Q42-MPC-COHORT-POLICY-V1 || epoch || floor LE || Commitment`.
//! - Party[i] = `Q42-MPC-COHORT-PARTY-V1 || Policy || party[i]`.
//! - Release = `Q42-MPC-COHORT-RELEASE-V1 || Policy || Party[0] || Party[1]`.
//! - Role message[i] = `Q42-MPC-COHORT-ROLE-SIGNATURE-V1 || Release || role_byte || Party[i]`.
//!
//! The host authority signs the 32-byte Policy and both 32-byte role messages
//! with strict Ed25519. It attests that tags were prepared in the required
//! `Q42-COHORT-TAG-V1` epoch domain and authorizes this exact ordered pair.
//! The verifier sees both tables and key shares. It checks a detached local
//! aggregate, with no distributed preprocessing or private PSI protocol.

use crate::{Args, AssetHost, Value};

impl AssetHost<'_> {
    pub(crate) fn evaluate_zk_mpc_and_vole_psi(&mut self, args: &Args) -> Result<Value, String> {
        #[cfg(not(target_arch = "wasm32"))]
        {
            native::verify(self, args)
        }
        #[cfg(target_arch = "wasm32")]
        {
            let _ = args;
            Err("cohort_verification_host_unavailable".into())
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
mod native {
    use std::{
        cmp::Ordering,
        env,
        fs::{self, File},
        io::Read,
        path::{Path, PathBuf},
    };

    use qualia_core_db::crypto::network::signing_profile::verify_ed25519_raw_strict;
    use sha2::{Digest, Sha256};

    use crate::{
        fields, qualia_cas_stats_modalities_geospatial_network_ops::ReceiptGraph, required, text,
        Args, AssetHost, Value,
    };

    const MAGIC: &[u8] = b"Q42-MPC-COHORT-VERIFICATION-V1\0";
    const MAC_DOMAIN: &[u8] = b"Q42-MPC-COHORT-MAC-COMMITMENT-V1";
    const POLICY_DOMAIN: &[u8] = b"Q42-MPC-COHORT-POLICY-V1";
    const PARTY_DOMAIN: &[u8] = b"Q42-MPC-COHORT-PARTY-V1";
    const RELEASE_DOMAIN: &[u8] = b"Q42-MPC-COHORT-RELEASE-V1";
    const ROLE_SIGNATURE_DOMAIN: &[u8] = b"Q42-MPC-COHORT-ROLE-SIGNATURE-V1";
    const PRIME: u64 = (1u64 << 61) - 1;
    const MAX_ROWS: usize = 4096;
    const ROW_BYTES: usize = 48;
    const PARTY_HEADER_BYTES: usize = 17;
    const FIXED_BYTES: usize = MAGIC.len() + 32 + 32 + 8 + 32 + 64 + 2 * (PARTY_HEADER_BYTES + 64);
    const MAX_BYTES: usize = FIXED_BYTES + 2 * MAX_ROWS * ROW_BYTES;
    const HOST_UNAVAILABLE: &str = "cohort_verification_host_unavailable";
    const REJECTED: &str = "cohort_authentication_failed";
    const BELOW_FLOOR: &str = "cohort_below_k_anonymity_floor";

    struct Party<'a> {
        role: u8,
        alpha: u64,
        encoded: &'a [u8],
        rows: &'a [u8],
    }

    struct Packet<'a> {
        authority: &'a [u8; 32],
        epoch: &'a [u8; 32],
        floor: u64,
        salt: &'a [u8; 32],
        policy_signature: &'a [u8; 64],
        parties: [Party<'a>; 2],
        role_signatures: [&'a [u8; 64]; 2],
    }

    struct Row<'a> {
        tag: &'a [u8; 32],
        value_share: u64,
        mac_share: u64,
    }

    pub(super) fn verify(host: &mut AssetHost<'_>, args: &Args) -> Result<Value, String> {
        fields(args, &["mode", "expected_epoch_sha256"]).map_err(|_| REJECTED)?;
        if required(args, "mode")
            .and_then(text)
            .map_err(|_| REJECTED)?
            != "verify_local_authenticated_cohort"
        {
            return Err(
                "Expected verify_local_authenticated_cohort on the trusted native host".into(),
            );
        }
        let expected_epoch = decode_digest(
            required(args, "expected_epoch_sha256")
                .and_then(text)
                .map_err(|_| REJECTED)?,
        )
        .filter(|value| *value != [0; 32])
        .ok_or(REJECTED)?;
        let authority_pin = env_digest("SYNTH_MPC_COHORT_AUTHORITY_SHA256")?;
        let release_pin = env_digest("SYNTH_MPC_COHORT_RELEASE_SHA256")?;
        let location =
            env::var_os("SYNTH_MPC_COHORT_VERIFICATION_INPUT").ok_or(HOST_UNAVAILABLE)?;
        let bytes = read_packet(host, Path::new(&location))?;
        let packet = Packet::parse(&bytes).ok_or(REJECTED)?;
        if expected_epoch != *packet.epoch || !packet.authenticate(&authority_pin, &release_pin) {
            return Err(REJECTED.into());
        }
        let (cardinality, sum) = packet.checked_aggregate().map_err(|_| REJECTED)?;
        if cardinality < packet.floor {
            return Err(BELOW_FLOOR.into());
        }

        drop(bytes);
        let mut digest = Sha256::new();
        digest.update(b"Q42-MPC-COHORT-AGGREGATE-V1");
        digest.update(cardinality.to_le_bytes());
        digest.update(sum.to_le_bytes());
        let subject = format!("urn:qualia:cohort-aggregate:sha256:{:x}", digest.finalize());
        let mut graph = ReceiptGraph::new(&subject);
        graph.fact(
            &subject,
            "http://www.w3.org/1999/02/22-rdf-syntax-ns#type",
            "urn:qualia:cohort:verified-local-aggregate",
        )?;
        graph.fact(
            &subject,
            "urn:qualia:cohort:intersectionCardinality",
            &format!("urn:qualia:count:{cardinality}"),
        )?;
        graph.fact(
            &subject,
            "urn:qualia:cohort:verifiedAggregateSum",
            &format!("urn:qualia:field:mersenne61:{sum}"),
        )?;
        graph.fact(
            &subject,
            "urn:qualia:cohort:macCheck",
            "urn:qualia:cohort:verified",
        )?;
        let receipt = graph.seal(host.limits.resource_bytes.min(1024 * 1024))?;
        Ok(Value::Record(Args::from([
            ("intersection_cardinality".into(), Value::U64(cardinality)),
            ("verified_aggregate_sum".into(), Value::U64(sum)),
            ("spdz_mac_verified".into(), Value::Bool(true)),
            ("receipt".into(), host.admit_encoded("q42", receipt)?),
        ])))
    }

    impl<'a> Packet<'a> {
        fn parse(bytes: &'a [u8]) -> Option<Self> {
            let mut rest = bytes.strip_prefix(MAGIC)?;
            let authority = take_array(&mut rest)?;
            let epoch = take_array(&mut rest)?;
            let floor = take_u64(&mut rest)?;
            let salt = take_array(&mut rest)?;
            let policy_signature = take_array(&mut rest)?;
            if !(2..=MAX_ROWS as u64).contains(&floor) || *salt == [0; 32] {
                return None;
            }
            let parties = [Party::parse(&mut rest, 0)?, Party::parse(&mut rest, 1)?];
            let role_signatures = [take_array(&mut rest)?, take_array(&mut rest)?];
            rest.is_empty().then_some(Self {
                authority,
                epoch,
                floor,
                salt,
                policy_signature,
                parties,
                role_signatures,
            })
        }

        fn authenticate(&self, authority_pin: &[u8; 32], release_pin: &[u8; 32]) -> bool {
            let key_digest: [u8; 32] = Sha256::digest(self.authority).into();
            if &key_digest != authority_pin {
                return false;
            }
            let mac_commitment = hash(|hash| {
                hash.update(MAC_DOMAIN);
                hash.update(self.epoch);
                hash.update(self.salt);
                for party in &self.parties {
                    hash.update(party.alpha.to_le_bytes());
                }
            });
            let policy = hash(|hash| {
                hash.update(POLICY_DOMAIN);
                hash.update(self.epoch);
                hash.update(self.floor.to_le_bytes());
                hash.update(mac_commitment);
            });
            if verify_ed25519_raw_strict(self.authority, &policy, self.policy_signature).is_err() {
                return false;
            }
            let party_digests = self.parties.each_ref().map(|party| {
                hash(|hash| {
                    hash.update(PARTY_DOMAIN);
                    hash.update(policy);
                    hash.update(party.encoded);
                })
            });
            let release = hash(|hash| {
                hash.update(RELEASE_DOMAIN);
                hash.update(policy);
                hash.update(party_digests[0]);
                hash.update(party_digests[1]);
            });
            if &release != release_pin {
                return false;
            }
            self.parties
                .iter()
                .zip(party_digests)
                .zip(self.role_signatures)
                .all(|((party, digest), signature)| {
                    let message = hash(|hash| {
                        hash.update(ROLE_SIGNATURE_DOMAIN);
                        hash.update(release);
                        hash.update([party.role]);
                        hash.update(digest);
                    });
                    verify_ed25519_raw_strict(self.authority, &message, signature).is_ok()
                })
        }

        fn checked_aggregate(&self) -> Result<(u64, u64), ()> {
            let [left, right] = &self.parties;
            let alpha = add(left.alpha, right.alpha);
            if alpha == 0 || !left.rows_canonical() || !right.rows_canonical() {
                return Err(());
            }
            let mut left_rows = left.rows.chunks_exact(ROW_BYTES);
            let mut right_rows = right.rows.chunks_exact(ROW_BYTES);
            let mut a = left_rows.next().map(Row::parse).transpose()?;
            let mut b = right_rows.next().map(Row::parse).transpose()?;
            let mut cardinality = 0;
            let mut sum = 0;
            let mut mac = 0;
            while let (Some(left), Some(right)) = (&a, &b) {
                match left.tag.cmp(right.tag) {
                    Ordering::Less => a = left_rows.next().map(Row::parse).transpose()?,
                    Ordering::Greater => b = right_rows.next().map(Row::parse).transpose()?,
                    Ordering::Equal => {
                        let value = add(left.value_share, right.value_share);
                        let row_mac = add(left.mac_share, right.mac_share);
                        if row_mac != multiply(value, alpha) {
                            return Err(());
                        }
                        cardinality += 1;
                        sum = add(sum, value);
                        mac = add(mac, row_mac);
                        a = left_rows.next().map(Row::parse).transpose()?;
                        b = right_rows.next().map(Row::parse).transpose()?;
                    }
                }
            }
            (mac == multiply(sum, alpha))
                .then_some((cardinality, sum))
                .ok_or(())
        }
    }

    impl<'a> Party<'a> {
        fn parse(input: &mut &'a [u8], expected_role: u8) -> Option<Self> {
            let start = *input;
            let role = *take(input, 1)?.first()?;
            let alpha = take_u64(input)?;
            let count = usize::try_from(take_u64(input)?).ok()?;
            if role != expected_role || alpha >= PRIME || count > MAX_ROWS {
                return None;
            }
            let rows = take(input, count * ROW_BYTES)?;
            let encoded = &start[..PARTY_HEADER_BYTES + rows.len()];
            Some(Self {
                role,
                alpha,
                encoded,
                rows,
            })
        }

        fn rows_canonical(&self) -> bool {
            let mut previous = None;
            for encoded in self.rows.chunks_exact(ROW_BYTES) {
                let Ok(row) = Row::parse(encoded) else {
                    return false;
                };
                if previous.is_some_and(|tag| tag >= row.tag) {
                    return false;
                }
                previous = Some(row.tag);
            }
            true
        }
    }

    impl<'a> Row<'a> {
        fn parse(mut input: &'a [u8]) -> Result<Self, ()> {
            let tag = take_array(&mut input).ok_or(())?;
            let value_share = take_u64(&mut input).ok_or(())?;
            let mac_share = take_u64(&mut input).ok_or(())?;
            if *tag == [0; 32] || value_share >= PRIME || mac_share >= PRIME {
                return Err(());
            }
            Ok(Self {
                tag,
                value_share,
                mac_share,
            })
        }
    }

    fn read_packet(host: &mut AssetHost<'_>, path: &Path) -> Result<Vec<u8>, String> {
        let path = host_private_path(path)?;
        let mut file = File::open(path).map_err(|_| HOST_UNAVAILABLE)?;
        let meta = file.metadata().map_err(|_| HOST_UNAVAILABLE)?;
        let size = usize::try_from(meta.len()).map_err(|_| REJECTED)?;
        if !meta.is_file()
            || !(FIXED_BYTES..=MAX_BYTES.min(host.limits.resource_bytes)).contains(&size)
        {
            return Err(REJECTED.into());
        }
        host.charge(300_000 + size * 8, size)?;
        let mut bytes = vec![0; size];
        file.read_exact(&mut bytes).map_err(|_| REJECTED)?;
        if file.read(&mut [0]).map_err(|_| REJECTED)? != 0 {
            return Err(REJECTED.into());
        }
        Ok(bytes)
    }

    fn host_private_path(path: &Path) -> Result<PathBuf, String> {
        if !path.is_absolute() {
            return Err(HOST_UNAVAILABLE.into());
        }
        check_local_path(path)?;
        let private = fs::canonicalize(path).map_err(|_| HOST_UNAVAILABLE)?;
        check_local_path(&private)?;
        if !fs::symlink_metadata(&private)
            .map_err(|_| HOST_UNAVAILABLE)?
            .is_file()
        {
            return Err(HOST_UNAVAILABLE.into());
        }
        for name in ["SYNTH_INPUT_ROOT", "SYNTH_OUTPUT_ROOT"] {
            if let Some(root) = env::var_os(name) {
                let root = fs::canonicalize(root).map_err(|_| HOST_UNAVAILABLE)?;
                if path_is_within(&private, &root)? {
                    return Err(HOST_UNAVAILABLE.into());
                }
            }
        }
        Ok(private)
    }

    fn check_local_path(_path: &Path) -> Result<(), String> {
        #[cfg(windows)]
        {
            use std::path::{Component, Prefix};
            if !matches!(_path.components().next(), Some(Component::Prefix(prefix))
                if matches!(prefix.kind(), Prefix::Disk(_) | Prefix::VerbatimDisk(_)))
            {
                return Err(HOST_UNAVAILABLE.into());
            }
        }
        Ok(())
    }

    #[cfg(windows)]
    fn path_is_within(path: &Path, root: &Path) -> Result<bool, String> {
        let path = path.to_str().ok_or(HOST_UNAVAILABLE)?.to_uppercase();
        let root = root.to_str().ok_or(HOST_UNAVAILABLE)?.to_uppercase();
        Ok(Path::new(&path).starts_with(root))
    }

    #[cfg(not(windows))]
    fn path_is_within(path: &Path, root: &Path) -> Result<bool, String> {
        Ok(path.starts_with(root))
    }

    fn take<'a>(input: &mut &'a [u8], count: usize) -> Option<&'a [u8]> {
        let (value, rest) = input.split_at_checked(count)?;
        *input = rest;
        Some(value)
    }

    fn take_array<'a, const N: usize>(input: &mut &'a [u8]) -> Option<&'a [u8; N]> {
        take(input, N)?.try_into().ok()
    }

    fn take_u64(input: &mut &[u8]) -> Option<u64> {
        Some(u64::from_le_bytes(*take_array(input)?))
    }

    fn env_digest(name: &str) -> Result<[u8; 32], String> {
        env::var(name)
            .ok()
            .as_deref()
            .and_then(decode_digest)
            .filter(|value| *value != [0; 32])
            .ok_or_else(|| HOST_UNAVAILABLE.into())
    }

    fn decode_digest(text: &str) -> Option<[u8; 32]> {
        if text.len() != 64 {
            return None;
        }
        fn digit(byte: u8) -> Option<u8> {
            match byte {
                b'0'..=b'9' => Some(byte - b'0'),
                b'a'..=b'f' => Some(byte - b'a' + 10),
                _ => None,
            }
        }
        let mut bytes = [0; 32];
        for (dest, src) in bytes.iter_mut().zip(text.as_bytes().chunks_exact(2)) {
            *dest = (digit(src[0])? << 4) | digit(src[1])?;
        }
        Some(bytes)
    }

    fn hash(update: impl FnOnce(&mut Sha256)) -> [u8; 32] {
        let mut digest = Sha256::new();
        update(&mut digest);
        digest.finalize().into()
    }

    fn add(a: u64, b: u64) -> u64 {
        (a + b) % PRIME
    }

    fn multiply(a: u64, b: u64) -> u64 {
        ((u128::from(a) * u128::from(b)) % u128::from(PRIME)) as u64
    }
}
