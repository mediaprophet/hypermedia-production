/// Maximum tile dimension admitted by this cold authoring grid.
pub const MAX_TILE_SIZE: usize = 4096;
pub const MAX_TILES: usize = 65_536;
pub const MAX_STROKE_SAMPLES: usize = 65_536;
pub const MAX_HISTORY_ENTRIES: usize = 256;
pub const MAX_LABEL_BYTES: usize = 4096;
pub const MAX_SAMPLE_VERTEX_VISITS: usize = 16_777_216;

impl CowTerrainTileGrid {
    pub fn new(tile_size: usize, cell_size_m: f32, byte_budget: usize) -> Result<Self, GeoStudioError> {
        if tile_size == 0 || tile_size > MAX_TILE_SIZE {
            return Err(GeoStudioError::InvalidParameter("tile_size must be 1..=4096"));
        }
        if !cell_size_m.is_finite() || cell_size_m <= 0.0 || !(cell_size_m * tile_size as f32).is_finite() {
            return Err(GeoStudioError::InvalidParameter("cell size and tile span must be finite and positive"));
        }
        if byte_budget == 0 { return Err(GeoStudioError::InvalidParameter("byte_budget must be positive")); }
        Ok(Self {
            tile_size, cell_size_m, tiles: HashMap::new(), undo_stack: Vec::new(), redo_stack: Vec::new(),
            active_baseline: HashMap::new(), active_strokes: Vec::new(), byte_budget,
            next_commit_id: 1, is_stroke_active: false,
        })
    }

    pub fn tile_size(&self) -> usize { self.tile_size }
    pub fn cell_size_m(&self) -> f32 { self.cell_size_m }
    pub fn byte_budget(&self) -> usize { self.byte_budget }
    pub fn is_stroke_active(&self) -> bool { self.is_stroke_active }
    pub fn tiles(&self) -> &HashMap<(i32, i32), Arc<Vec<f32>>> { &self.tiles }
    pub fn undo_history(&self) -> &[UndoCommitEntry] { &self.undo_stack }
    pub fn redo_history(&self) -> &[UndoCommitEntry] { &self.redo_stack }
    pub fn active_samples(&self) -> &[StrokeSample] { &self.active_strokes }

    fn tile_bytes(&self) -> usize {
        // Construction bounds tile_size to 4096, so these products fit usize on supported targets.
        self.tile_size * self.tile_size * core::mem::size_of::<f32>()
    }

    fn admit(&self, extra: usize) -> Result<(), GeoStudioError> {
        let requested = self.estimate_memory_bytes().checked_add(extra).ok_or(
            GeoStudioError::InvalidParameter("payload byte count overflow"),
        )?;
        if requested > self.byte_budget {
            return Err(GeoStudioError::ByteBudgetExceeded { current_bytes: requested, budget_bytes: self.byte_budget });
        }
        Ok(())
    }

    pub fn ensure_tile(&mut self, tx: i32, ty: i32, initial_height: f32) -> Result<(), GeoStudioError> {
        if !initial_height.is_finite() { return Err(GeoStudioError::InvalidParameter("initial height must be finite")); }
        if self.tiles.contains_key(&(tx, ty)) { return Ok(()); }
        if self.is_stroke_active { return Err(GeoStudioError::StrokeAlreadyActive); }
        if self.tiles.len() >= MAX_TILES { return Err(GeoStudioError::InvalidParameter("tile count limit reached")); }
        self.admit(self.tile_bytes())?;
        self.tiles.try_reserve(1).map_err(|_| GeoStudioError::InvalidParameter("tile table allocation refused"))?;
        let mut data = Vec::new();
        data.try_reserve_exact(self.tile_size * self.tile_size).map_err(|_| GeoStudioError::InvalidParameter("tile allocation refused"))?;
        data.resize(self.tile_size * self.tile_size, initial_height);
        self.tiles.insert((tx, ty), Arc::new(data));
        Ok(())
    }

    /// Import one already-decoded finite height tile before local history begins.
    /// Source identifiers, nodata masks, datum and canonical decoding stay with the host.
    pub fn load_tile(&mut self, tx: i32, ty: i32, heights: &[f32]) -> Result<(), GeoStudioError> {
        if self.is_stroke_active { return Err(GeoStudioError::StrokeAlreadyActive); }
        if !self.undo_stack.is_empty() || !self.redo_stack.is_empty() {
            return Err(GeoStudioError::InvalidParameter("source tile import requires empty local history"));
        }
        if heights.len() != self.tile_size * self.tile_size || heights.iter().any(|v| !v.is_finite()) {
            return Err(GeoStudioError::InvalidParameter("source tile must contain exactly tile_size squared finite metre heights"));
        }
        if !self.tiles.contains_key(&(tx, ty)) && self.tiles.len() >= MAX_TILES {
            return Err(GeoStudioError::InvalidParameter("tile count limit reached"));
        }
        self.admit(self.tile_bytes())?;
        self.tiles.try_reserve(1).map_err(|_| GeoStudioError::InvalidParameter("tile table allocation refused"))?;
        let mut data = Vec::new();
        data.try_reserve_exact(heights.len()).map_err(|_| GeoStudioError::InvalidParameter("source tile allocation refused"))?;
        data.extend_from_slice(heights);
        self.tiles.insert((tx, ty), Arc::new(data));
        Ok(())
    }

    pub fn begin_stroke(&mut self) -> Result<(), GeoStudioError> {
        if self.is_stroke_active { return Err(GeoStudioError::StrokeAlreadyActive); }
        self.active_baseline.clear();
        self.active_strokes.clear();
        self.is_stroke_active = true;
        Ok(())
    }

    fn validate_sample(sample: &StrokeSample) -> Result<(), GeoStudioError> {
        if [sample.center_x_m, sample.center_y_m, sample.radius_m, sample.strength,
            sample.target_height_m, sample.step_m].iter().any(|v| !v.is_finite())
            || sample.radius_m <= 0.0 || !(sample.radius_m * sample.radius_m).is_finite()
            || sample.strength < 0.0
            || ![(sample.center_x_m - sample.radius_m), (sample.center_x_m + sample.radius_m),
                (sample.center_y_m - sample.radius_m), (sample.center_y_m + sample.radius_m)].iter().all(|v| v.is_finite())
        {
            return Err(GeoStudioError::InvalidParameter("stroke coordinates, radius and strength must be finite and admitted"));
        }
        if sample.op_code > 5 || sample.falloff_code > 3 || sample.layer_index != 0 || sample.flags != 0 || sample.pad != [0; 2] {
            return Err(GeoStudioError::InvalidParameter("unsupported stroke operation, falloff, layer, flags or padding"));
        }
        if sample.op_code == 4 && sample.step_m <= 0.0 {
            return Err(GeoStudioError::InvalidParameter("terrace step must be positive"));
        }
        Ok(())
    }

    /// Prepare all touched tile successors, then install atomically for this sample.
    /// Existing tile/sample payload plus prepared tile payload is admitted before cloning.
    pub fn apply_stroke_sample(&mut self, sample: StrokeSample) -> Result<usize, GeoStudioError> {
        if !self.is_stroke_active { return Err(GeoStudioError::NoActiveStroke); }
        Self::validate_sample(&sample)?;
        if self.active_strokes.len() >= MAX_STROKE_SAMPLES {
            return Err(GeoStudioError::InvalidParameter("stroke sample limit reached"));
        }
        let span = self.tile_size as f32 * self.cell_size_m;
        let min_tx = ((sample.center_x_m as f64 - sample.radius_m as f64) / span as f64).floor();
        let max_tx = ((sample.center_x_m as f64 + sample.radius_m as f64) / span as f64).floor();
        let min_ty = ((sample.center_y_m as f64 - sample.radius_m as f64) / span as f64).floor();
        let max_ty = ((sample.center_y_m as f64 + sample.radius_m as f64) / span as f64).floor();
        // Visit existing tiles only, never an input-sized unpopulated coordinate range.
        let mut coords = Vec::new();
        coords.try_reserve(self.tiles.len()).map_err(|_| GeoStudioError::InvalidParameter("stroke coordinate allocation refused"))?;
        coords.extend(self.tiles.keys().copied().filter(|(tx, ty)|
            (*tx as f64) >= min_tx && (*tx as f64) <= max_tx && (*ty as f64) >= min_ty && (*ty as f64) <= max_ty));
        coords.sort_unstable();
        if coords.len().checked_mul(self.tile_size * self.tile_size).is_none_or(|n| n > MAX_SAMPLE_VERTEX_VISITS) {
            return Err(GeoStudioError::InvalidParameter("sample vertex work limit reached"));
        }
        let extra = coords.len().checked_mul(self.tile_bytes()).and_then(|n| n.checked_add(core::mem::size_of::<StrokeSample>()))
            .ok_or(GeoStudioError::InvalidParameter("stroke byte count overflow"))?;
        self.admit(extra)?;
        self.active_strokes.try_reserve(1).map_err(|_| GeoStudioError::InvalidParameter("journal allocation refused"))?;
        self.active_baseline.try_reserve(coords.len()).map_err(|_| GeoStudioError::InvalidParameter("baseline allocation refused"))?;
        let mut prepared = Vec::new();
        prepared.try_reserve(coords.len()).map_err(|_| GeoStudioError::InvalidParameter("stroke patch allocation refused"))?;
        let op = BrushOperation::from_sample(&sample);
        let falloff = BrushFalloff::from_code(sample.falloff_code);
        for coord @ (tx, ty) in coords {
            let (tile_min_x, tile_min_y) = (tx as f32 * span, ty as f32 * span);
            let (rmin_x, rmax_x) = (sample.center_x_m - sample.radius_m - tile_min_x, sample.center_x_m + sample.radius_m - tile_min_x);
            let (rmin_y, rmax_y) = (sample.center_y_m - sample.radius_m - tile_min_y, sample.center_y_m + sample.radius_m - tile_min_y);
            if [tile_min_x, tile_min_y, rmin_x, rmax_x, rmin_y, rmax_y].iter().any(|v| !v.is_finite()) {
                return Err(GeoStudioError::InvalidParameter("tile coordinates exceed finite brush domain"));
            }
            if rmax_x < 0.0 || rmin_x > span || rmax_y < 0.0 || rmin_y > span { continue; }
            let start_lx = (rmin_x / self.cell_size_m).floor().max(0.0) as usize;
            let end_lx = (rmax_x / self.cell_size_m).ceil().min((self.tile_size - 1) as f32) as usize;
            let start_ly = (rmin_y / self.cell_size_m).floor().max(0.0) as usize;
            let end_ly = (rmax_y / self.cell_size_m).ceil().min((self.tile_size - 1) as f32) as usize;
            if start_lx > end_lx || start_ly > end_ly { continue; }
            let rad_sq = sample.radius_m * sample.radius_m;
            let has_affected = (start_ly..=end_ly).any(|ly| (start_lx..=end_lx).any(|lx| {
                let (vx, vy) = (tile_min_x + lx as f32 * self.cell_size_m, tile_min_y + ly as f32 * self.cell_size_m);
                (vx - sample.center_x_m).powi(2) + (vy - sample.center_y_m).powi(2) <= rad_sq
            }));
            if !has_affected { continue; }
            let before = self.tiles.get(&coord).ok_or(GeoStudioError::TileNotFound(tx, ty))?;
            let mut data = Vec::new();
            data.try_reserve_exact(before.len()).map_err(|_| GeoStudioError::InvalidParameter("stroke tile allocation refused"))?;
            data.extend_from_slice(before);
            for ly in start_ly..=end_ly {
                for lx in start_lx..=end_lx {
                    let (vx, vy) = (tile_min_x + lx as f32 * self.cell_size_m, tile_min_y + ly as f32 * self.cell_size_m);
                    let dist = ((vx - sample.center_x_m).powi(2) + (vy - sample.center_y_m).powi(2)).sqrt();
                    if dist <= sample.radius_m {
                        apply_op_to_vertex(op, sample.strength, falloff.compute_weight(dist, sample.radius_m), self.tile_size, lx, ly, &mut data);
                    }
                }
            }
            if data.iter().any(|v| !v.is_finite()) { return Err(GeoStudioError::InvalidParameter("brush output is not finite")); }
            prepared.push((coord, Arc::clone(before), Arc::new(data)));
        }
        let modified = prepared.len();
        for (coord, before, after) in prepared {
            self.active_baseline.entry(coord).or_insert(before);
            self.tiles.insert(coord, after);
        }
        self.active_strokes.push(sample);
        Ok(modified)
    }

    /// Commit preparation refuses without changing preview, redo, IDs or history.
    pub fn commit_stroke(&mut self, label: &str) -> Result<u64, GeoStudioError> {
        if !self.is_stroke_active { return Err(GeoStudioError::NoActiveStroke); }
        if label.len() > MAX_LABEL_BYTES || self.undo_stack.len() >= MAX_HISTORY_ENTRIES {
            return Err(GeoStudioError::InvalidParameter("history or label limit reached"));
        }
        let next = self.next_commit_id.checked_add(1).ok_or(GeoStudioError::InvalidParameter("commit identifier exhausted"))?;
        // Moving each active baseline into a two-reference history patch adds
        // one conservatively charged reference; admit that before commit.
        self.admit(self.active_baseline.len().saturating_mul(self.tile_bytes()))?;
        let mut label_copy = String::new();
        label_copy.try_reserve_exact(label.len()).map_err(|_| GeoStudioError::InvalidParameter("label allocation refused"))?;
        label_copy.push_str(label);
        let mut modified_tiles = Vec::new();
        modified_tiles.try_reserve_exact(self.active_baseline.len()).map_err(|_| GeoStudioError::InvalidParameter("commit patch allocation refused"))?;
        self.undo_stack.try_reserve(1).map_err(|_| GeoStudioError::InvalidParameter("undo allocation refused"))?;
        for (coord, before_tile) in &self.active_baseline {
            let after_tile = self.tiles.get(coord).ok_or(GeoStudioError::TileNotFound(coord.0, coord.1))?;
            if !Arc::ptr_eq(before_tile, after_tile) {
                modified_tiles.push(TileDeltaPatch { tile_coord: *coord, before_tile: Arc::clone(before_tile), after_tile: Arc::clone(after_tile) });
            }
        }
        modified_tiles.sort_by_key(|p| p.tile_coord);
        let commit_id = self.next_commit_id;
        self.undo_stack.push(UndoCommitEntry { commit_id, label: label_copy, modified_tiles, stroke_samples: std::mem::take(&mut self.active_strokes) });
        self.active_baseline.clear();
        self.redo_stack.clear();
        self.next_commit_id = next;
        self.is_stroke_active = false;
        Ok(commit_id)
    }

    /// Restore the precise original tile Arcs without committing or changing history.
    pub fn cancel_stroke(&mut self) -> Result<(), GeoStudioError> {
        if !self.is_stroke_active { return Err(GeoStudioError::NoActiveStroke); }
        for (coord, baseline) in self.active_baseline.drain() { self.tiles.insert(coord, baseline); }
        self.active_strokes.clear();
        self.is_stroke_active = false;
        Ok(())
    }

    pub fn undo(&mut self) -> Result<Option<u64>, GeoStudioError> {
        if self.is_stroke_active { return Err(GeoStudioError::StrokeAlreadyActive); }
        if self.undo_stack.is_empty() { return Ok(None); }
        self.redo_stack.try_reserve(1).map_err(|_| GeoStudioError::InvalidParameter("redo allocation refused"))?;
        let Some(entry) = self.undo_stack.pop() else { return Ok(None); };
        for patch in &entry.modified_tiles { self.tiles.insert(patch.tile_coord, Arc::clone(&patch.before_tile)); }
        let id = entry.commit_id;
        self.redo_stack.push(entry);
        Ok(Some(id))
    }

    pub fn redo(&mut self) -> Result<Option<u64>, GeoStudioError> {
        if self.is_stroke_active { return Err(GeoStudioError::StrokeAlreadyActive); }
        if self.redo_stack.is_empty() { return Ok(None); }
        self.undo_stack.try_reserve(1).map_err(|_| GeoStudioError::InvalidParameter("undo allocation refused"))?;
        let Some(entry) = self.redo_stack.pop() else { return Ok(None); };
        for patch in &entry.modified_tiles { self.tiles.insert(patch.tile_coord, Arc::clone(&patch.after_tile)); }
        let id = entry.commit_id;
        self.undo_stack.push(entry);
        Ok(Some(id))
    }

    pub fn replay_stroke_journal(&mut self, journal: &[StrokeSample], label: &str) -> Result<u64, GeoStudioError> {
        if journal.len() > MAX_STROKE_SAMPLES { return Err(GeoStudioError::InvalidParameter("journal sample limit reached")); }
        self.begin_stroke()?;
        for sample in journal {
            if let Err(error) = self.apply_stroke_sample(*sample) {
                self.cancel_stroke()?;
                return Err(error);
            }
        }
        match self.commit_stroke(label) {
            Ok(id) => Ok(id),
            Err(error) => { self.cancel_stroke()?; Err(error) }
        }
    }

    pub fn sample_height(&self, tx: i32, ty: i32, lx: usize, ly: usize) -> Option<f32> {
        if lx >= self.tile_size || ly >= self.tile_size { return None; }
        self.tiles.get(&(tx, ty)).and_then(|t| t.get(ly * self.tile_size + lx)).copied()
    }

    pub fn tile_arc(&self, tx: i32, ty: i32) -> Option<&Arc<Vec<f32>>> { self.tiles.get(&(tx, ty)) }

    /// Deliberately discard local undo/redo history, retaining current tile data and Arcs.
    /// Commit identifiers continue monotonically; this is not a native-project rebase.
    pub fn discard_local_history(&mut self) -> Result<usize, GeoStudioError> {
        if self.is_stroke_active { return Err(GeoStudioError::StrokeAlreadyActive); }
        let discarded = self.undo_stack.len().saturating_add(self.redo_stack.len());
        self.undo_stack.clear();
        self.redo_stack.clear();
        Ok(discarded)
    }

    /// Conservative tile-reference/sample payload estimate, not distinct allocation or heap bytes.
    pub fn payload_memory_bytes(&self) -> usize { self.estimate_memory_bytes() }

    fn estimate_memory_bytes(&self) -> usize {
        // No temporary allocation and no iteration over tile/patch/sample payloads.
        // Private history invariants cap this scan at MAX_HISTORY_ENTRIES (256).
        // Duplicate Arc references deliberately overestimate retained payload.
        if self.undo_stack.len().saturating_add(self.redo_stack.len()) > MAX_HISTORY_ENTRIES {
            return usize::MAX;
        }
        let mut tile_references = self.tiles.len().saturating_add(self.active_baseline.len());
        let mut samples = self.active_strokes.len();
        for entry in self.undo_stack.iter().chain(self.redo_stack.iter()) {
            samples = samples.saturating_add(entry.stroke_samples.len());
            tile_references = tile_references.saturating_add(entry.modified_tiles.len().saturating_mul(2));
        }
        tile_references.saturating_mul(self.tile_bytes()).saturating_add(samples.saturating_mul(core::mem::size_of::<StrokeSample>()))
    }
}
