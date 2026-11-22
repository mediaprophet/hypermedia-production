//! Fixed educational fixtures for the native WellFair accumulation model.
//! Copyright (c) 2026 Timothy Charles Holborn. CC BY-NC-ND 4.0.

use std::collections::BTreeMap;

use wellfare_core::anatomy::{
    accumulate, interactions, Effect, EvidenceTier, Factor, FactorKind, InteractionKind,
    SystemBurden, BODY_SYSTEMS,
};

use crate::{fields, required, text, Args, AssetHost, Value};

use super::{review_fact, review_graph, review_number};

const PROFILE: &str = "urn:qualia:wellfare:synthetic-burden:v1";
const VOCAB: &str = "https://qualiadb.org/ontology/wellfare-fixture#";

impl AssetHost<'_> {
    pub(crate) fn evaluate_clinical_burden_and_vault(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["mode", "fixture"])?;
        if text(required(args, "mode")?)? != "run_synthetic_burden_fixture" {
            return Err("Select run_synthetic_burden_fixture for the educational model".into());
        }
        let fixture = text(required(args, "fixture")?)?;
        let factors = match fixture {
            "all_systems" => all_systems_fixture(),
            "shared_targets" => shared_targets_fixture(),
            _ => return Err("Select all_systems or shared_targets".into()),
        };
        self.charge(100_000, 256 * 1024)?;
        let mut burdens: BTreeMap<String, SystemBurden> = accumulate(&factors)
            .into_iter()
            .map(|burden| (burden.system_id.clone(), burden))
            .collect();
        let pairs = interactions(&factors);
        let fixture_iri = format!("{PROFILE}:{fixture}");
        let mut receipt = review_graph("EducationalSyntheticBurdenFixture")?;
        review_fact(&mut receipt, "fixture", &fixture_iri)?;
        review_fact(
            &mut receipt,
            "modeledQuantity",
            &format!("{VOCAB}AbstractModelWeightMilli"),
        )?;
        review_number(&mut receipt, "fixtureFactors", factors.len())?;
        review_number(&mut receipt, "nativeSystemRows", burdens.len())?;

        let mut systems = Vec::with_capacity(BODY_SYSTEMS.len());
        for system in BODY_SYSTEMS {
            let (has_targets, burden) = match burdens.remove(system.id) {
                Some(burden) => (true, burden),
                None => (false, SystemBurden::default()),
            };
            let row = format!("{fixture_iri}:system:{}", system.id);
            receipt.fact(&fixture_iri, &format!("{VOCAB}systemRow"), &row)?;
            receipt.fact(
                &row,
                &format!("{VOCAB}system"),
                &format!("urn:qualia:wellfare:body-system:{}", system.id),
            )?;
            receipt.fact(
                &row,
                &format!("{VOCAB}rowBasis"),
                &format!(
                    "{VOCAB}{}",
                    if has_targets {
                        "FixtureContribution"
                    } else {
                        "UnmappedFixtureSystem"
                    }
                ),
            )?;
            for (predicate, value) in [
                ("adverseModelWeightMilli", burden.adverse_milli),
                ("supportiveModelWeightMilli", burden.supportive_milli),
                ("netModelWeightMilli", burden.net_milli),
            ] {
                receipt.fact(
                    &row,
                    &format!("{VOCAB}{predicate}"),
                    &format!("urn:qualia:model-weight-milli:{value}"),
                )?;
            }
            let mut contributors = Vec::new();
            for (direction, factors) in [
                ("adverse", burden.adverse_contributors),
                ("supportive", burden.supportive_contributors),
            ] {
                for factor in factors {
                    receipt.fact(
                        &row,
                        &format!("{VOCAB}{direction}Contributor"),
                        &format!("{fixture_iri}:factor:{factor}"),
                    )?;
                    contributors.push(Value::Record(Args::from([
                        ("direction".into(), Value::String(direction.into())),
                        ("fixture_factor".into(), Value::String(factor)),
                    ])));
                }
            }
            systems.push(Value::Record(Args::from([
                ("system_id".into(), Value::String(system.id.into())),
                ("has_fixture_targets".into(), Value::Bool(has_targets)),
                (
                    "adverse_model_weight_milli".into(),
                    Value::U64(u64::from(burden.adverse_milli)),
                ),
                (
                    "supportive_model_weight_milli".into(),
                    Value::U64(u64::from(burden.supportive_milli)),
                ),
                (
                    "net_model_weight_milli".into(),
                    Value::U64(u64::from(burden.net_milli)),
                ),
                ("contributors".into(), Value::List(contributors)),
            ])));
        }
        if !burdens.is_empty() {
            return Err(
                "The native synthetic fixture returned a system outside the canonical taxonomy"
                    .into(),
            );
        }

        let mut rules = Vec::with_capacity(pairs.len());
        for (index, pair) in pairs.into_iter().enumerate() {
            let rule = match pair.kind {
                InteractionKind::HerbDrug => "synthetic_medication_botanical_overlap",
                InteractionKind::Compounding => "shared_adverse_target",
                InteractionKind::Opposing => "opposing_target_effects",
            };
            let row = format!("{fixture_iri}:rule:{index}");
            receipt.fact(&fixture_iri, &format!("{VOCAB}ruleMatch"), &row)?;
            receipt.fact(&row, &format!("{VOCAB}ruleKind"), &format!("{VOCAB}{rule}"))?;
            receipt.fact(
                &row,
                &format!("{VOCAB}sharedSystem"),
                &format!("{fixture_iri}:system:{}", pair.system_id),
            )?;
            for (predicate, factor) in [
                ("firstFactor", &pair.factor_a),
                ("secondFactor", &pair.factor_b),
            ] {
                receipt.fact(
                    &row,
                    &format!("{VOCAB}{predicate}"),
                    &format!("{fixture_iri}:factor:{factor}"),
                )?;
            }
            rules.push(Value::Record(Args::from([
                ("system_id".into(), Value::String(pair.system_id)),
                ("factor_a".into(), Value::String(pair.factor_a)),
                ("factor_b".into(), Value::String(pair.factor_b)),
                ("fixture_rule".into(), Value::String(rule.into())),
            ])));
        }
        Ok(Value::Record(Args::from([
            (
                "scope".into(),
                Value::String("educational_synthetic_fixture".into()),
            ),
            ("fixture".into(), Value::String(fixture.into())),
            ("systems".into(), Value::List(systems)),
            ("rule_matches".into(), Value::List(rules)),
            ("receipt".into(), self.admit_review(receipt)?),
        ])))
    }
}

fn all_systems_fixture() -> Vec<Factor> {
    let mut adverse = fixture_factor("abstract-load-a", FactorKind::Other("fixture".into()));
    let mut supportive = fixture_factor("abstract-offset-b", FactorKind::Other("fixture".into()));
    for (index, system) in BODY_SYSTEMS.iter().enumerate() {
        adverse = adverse.targeting(
            system.id,
            Effect::Adverse,
            EvidenceTier::CommunityAnecdotal,
            100 + 10 * index as u32,
        );
        supportive = supportive.targeting(
            system.id,
            Effect::Supportive,
            EvidenceTier::CommunityAnecdotal,
            [50, 175, 350][index % 3],
        );
    }
    vec![adverse, supportive]
}

fn shared_targets_fixture() -> Vec<Factor> {
    let tier = EvidenceTier::CommunityAnecdotal;
    vec![
        fixture_factor("fictional-medication-a", FactorKind::Medication).targeting(
            "digestive",
            Effect::Adverse,
            tier,
            120,
        ),
        fixture_factor("fictional-botanical-b", FactorKind::Herb)
            .targeting("digestive", Effect::Modulating, tier, 80)
            .targeting("endocrine", Effect::Supportive, tier, 170),
        fixture_factor("abstract-load-c", FactorKind::Other("fixture".into()))
            .targeting("digestive", Effect::Adverse, tier, 180)
            .targeting("circulatory", Effect::Adverse, tier, 90),
        fixture_factor("abstract-offset-d", FactorKind::Other("fixture".into())).targeting(
            "circulatory",
            Effect::Supportive,
            tier,
            120,
        ),
    ]
}

fn fixture_factor(id: &str, kind: FactorKind) -> Factor {
    Factor::new(id, kind, id).from_source(PROFILE)
}
