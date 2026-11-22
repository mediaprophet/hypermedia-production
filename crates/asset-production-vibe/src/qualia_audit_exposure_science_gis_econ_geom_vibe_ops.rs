//! WP-29 production integrations identified by the exposure audit.

mod acoustics;
mod economics;
mod geometry;
mod gis;
mod safeguards;
mod science;
mod solvers;
mod spatial;
mod vibe_tools;
mod vision;

use crate::{fields, record, required, text, Args, AssetHost, Value};

impl AssetHost<'_> {
    pub(crate) fn enact_safeguards_dead_mans_and_accountability(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        safeguards::evaluate(
            self,
            text(required(args, "op")?)?,
            record(required(args, "parameters")?)?,
        )
    }

    pub(crate) fn decompile_merge_and_legislate_cof(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        vibe_tools::evaluate(
            self,
            text(required(args, "op")?)?,
            record(required(args, "parameters")?)?,
        )
    }

    pub(crate) fn super_resolve_and_compile_10d_observations(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        vision::evaluate(
            self,
            text(required(args, "op")?)?,
            record(required(args, "parameters")?)?,
        )
    }

    pub(crate) fn evaluate_gis_gp_terrain_and_datacubes(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        let op = text(required(args, "op")?)?;
        let parameters = record(required(args, "parameters")?)?;
        if op == "decode_zarr_v3_zip" {
            return self.decode_zarr_v3_zip(parameters);
        }
        gis::evaluate(self, op, parameters)
    }

    pub(crate) fn bridge_qac_ultrasound_haptic_and_bioacoustics(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        acoustics::evaluate(
            self,
            text(required(args, "op")?)?,
            record(required(args, "parameters")?)?,
        )
    }

    pub(crate) fn solve_macro_market_portfolio_and_engineering(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        economics::evaluate(
            self,
            text(required(args, "op")?)?,
            record(required(args, "parameters")?)?,
        )
    }

    pub(crate) fn encode_spatial_predicates_and_situated_place_time(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        spatial::evaluate(
            self,
            text(required(args, "op")?)?,
            record(required(args, "parameters")?)?,
        )
    }

    pub(crate) fn evaluate_graph_opt_active_learning_and_special_fns(
        &mut self,
        args: &Args,
    ) -> Result<Value, String> {
        fields(args, &["op", "parameters"])?;
        solvers::evaluate(
            self,
            text(required(args, "op")?)?,
            record(required(args, "parameters")?)?,
        )
    }
}
