//! Sprite-sheet, 9-slice, and PBR-set packing. Direct opaque blit; PNG via encode_rgba_png.
use super::graphics::lattice_profile;
use super::*;
use std::collections::BTreeMap;
use vibe::Value;

fn nonnegative(a: &Args, key: &str, default: usize, max: usize) -> Result<usize, String> {
    let n = a.get(key).map(|v| integer(v, key)).unwrap_or(Ok(default))?;
    if n > max {
        Err(format!("{key} outside 0..{max}"))
    } else {
        Ok(n)
    }
}

fn blit_opaque(
    dest: &mut [[u8; 4]],
    dw: u32,
    dh: u32,
    src: &[[u8; 4]],
    sw: u32,
    sh: u32,
    ox: u32,
    oy: u32,
) {
    for sy in 0..sh {
        let dy = oy.saturating_add(sy);
        if dy >= dh {
            break;
        }
        for sx in 0..sw {
            let dx = ox.saturating_add(sx);
            if dx >= dw {
                break;
            }
            dest[(dy * dw + dx) as usize] = src[(sy * sw + sx) as usize];
        }
    }
}

fn blit_rect_nn(
    dest: &mut [[u8; 4]],
    dw: u32,
    dh: u32,
    dx: u32,
    dy: u32,
    dww: u32,
    dhh: u32,
    src: &[[u8; 4]],
    sw: u32,
    sh: u32,
    sx: u32,
    sy: u32,
    sww: u32,
    shh: u32,
) {
    if dww == 0 || dhh == 0 || sww == 0 || shh == 0 {
        return;
    }
    for y in 0..dhh {
        let dst_y = dy.saturating_add(y);
        if dst_y >= dh {
            break;
        }
        let src_y = sy.saturating_add(y * shh / dhh);
        if src_y >= sh {
            continue;
        }
        for x in 0..dww {
            let dst_x = dx.saturating_add(x);
            if dst_x >= dw {
                break;
            }
            let src_x = sx.saturating_add(x * sww / dww);
            if src_x >= sw {
                continue;
            }
            dest[(dst_y * dw + dst_x) as usize] = src[(src_y * sw + src_x) as usize];
        }
    }
}

fn rects_overlap(ax: u32, ay: u32, aw: u32, ah: u32, bx: u32, by: u32, bw: u32, bh: u32) -> bool {
    ax < bx.saturating_add(bw)
        && bx < ax.saturating_add(aw)
        && ay < by.saturating_add(bh)
        && by < ay.saturating_add(ah)
}

/// Copy 1px wrap gutter after an opaque tile blit: right from the tile's left
/// column, bottom from the top row, corner from the top-left pixel. Writes only
/// into padding / trailing sheet pixels so frames stay disjoint.
fn copy_autotile_gutter(
    dest: &mut [[u8; 4]],
    dw: u32,
    dh: u32,
    ox: u32,
    oy: u32,
    tw: u32,
    th: u32,
) {
    if tw == 0 || th == 0 {
        return;
    }
    let right_x = ox.saturating_add(tw);
    let bottom_y = oy.saturating_add(th);
    if right_x < dw {
        for sy in 0..th {
            let dy = oy.saturating_add(sy);
            if dy >= dh {
                break;
            }
            dest[(dy * dw + right_x) as usize] = dest[(dy * dw + ox) as usize];
        }
    }
    if bottom_y < dh {
        for sx in 0..=tw {
            let dx = ox.saturating_add(sx);
            if dx >= dw {
                break;
            }
            dest[(bottom_y * dw + dx) as usize] = dest[(oy * dw + dx) as usize];
        }
    }
}

impl AssetHost<'_> {
    fn encode_rgba_png_handle(
        &mut self,
        image: Value,
        profile: Option<&Value>,
    ) -> Result<Value, String> {
        let mut enc = BTreeMap::new();
        enc.insert("image".into(), image);
        if let Some(p) = profile {
            enc.insert("profile".into(), p.clone());
        }
        self.graphics("hypermedia.radiance.encode_rgba_png", &enc)
    }

    fn png_or_encode(&mut self, handle: &Value, profile: Option<&Value>) -> Result<Value, String> {
        match self.workspace.get(handle)? {
            Artifact::Rgba(_) => self.encode_rgba_png_handle(handle.clone(), profile),
            Artifact::Encoded(r) if r.kind == "png" => Ok(handle.clone()),
            _ => Err("Expected rgba or png handle".into()),
        }
    }

    pub(crate) fn pack_sprite_sheet(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "tiles",
                "columns",
                "padding",
                "names",
                "profile",
                "mode",
                "wrap",
            ],
        )?;
        let profile = lattice_profile(a)?;
        let tiles_v = list(required(a, "tiles")?)?;
        if tiles_v.is_empty() || tiles_v.len() > 64 {
            return Err("tiles needs 1..64".into());
        }
        let pack_mode = match a.get("mode") {
            Some(v) => {
                let m = text(v)?;
                match m {
                    "grid" | "autotile" => m,
                    _ => {
                        return Err(
                            "Unknown pack mode: expected 'grid' or 'autotile'".into(),
                        )
                    }
                }
            }
            None => "grid",
        };
        let wrap = match a.get("wrap") {
            Some(v) => {
                let w = text(v)?;
                match w {
                    "clamp" | "repeat" | "mirror" => w,
                    _ => {
                        return Err(
                            "Unknown wrap: expected 'clamp', 'repeat', or 'mirror'".into(),
                        )
                    }
                }
            }
            None => {
                if pack_mode == "autotile" {
                    "repeat"
                } else {
                    "clamp"
                }
            }
        };
        let names = match a.get("names") {
            None => None,
            Some(v) => {
                let ns = list(v)?;
                if ns.len() != tiles_v.len() {
                    return Err("names length must match tiles".into());
                }
                let mut out = Vec::with_capacity(ns.len());
                for n in ns {
                    out.push(text(n)?.to_string());
                }
                Some(out)
            }
        };
        let n_tiles = tiles_v.len();
        let mut default_cols = 1usize;
        while default_cols * default_cols < n_tiles {
            default_cols += 1;
        }
        default_cols = default_cols.clamp(1, 64);
        let columns = count(a, "columns", default_cols, 64)?;
        let padding = if a.get("padding").is_some() {
            let p = nonnegative(a, "padding", 0, 64)? as u32;
            if pack_mode == "autotile" && p == 0 {
                return Err("autotile needs padding>=1".into());
            }
            p
        } else if pack_mode == "autotile" {
            1
        } else {
            0
        };

        let mut dims = Vec::with_capacity(n_tiles);
        for t in tiles_v {
            let r = self.rgba(t)?;
            if r.width == 0 || r.height == 0 || r.width > profile.max_dim || r.height > profile.max_dim
            {
                return Err(format!("tile dimensions outside 1..{}", profile.max_dim));
            }
            dims.push((r.width, r.height));
        }

        struct Frame {
            name: String,
            x: u32,
            y: u32,
            w: u32,
            h: u32,
        }
        let mut frames = Vec::with_capacity(n_tiles);
        let mut x: u32 = 0;
        let mut y: u32 = 0;
        let mut col: usize = 0;
        let mut row_h: u32 = 0;
        let mut sheet_w: u32 = 0;
        for (i, &(tw, th)) in dims.iter().enumerate() {
            if col == columns {
                x = 0;
                y = y
                    .checked_add(row_h)
                    .and_then(|v| v.checked_add(padding))
                    .ok_or("sprite sheet overflow")?;
                row_h = 0;
                col = 0;
            }
            let name = names
                .as_ref()
                .map(|n| n[i].clone())
                .unwrap_or_else(|| i.to_string());
            frames.push(Frame {
                name,
                x,
                y,
                w: tw,
                h: th,
            });
            sheet_w = sheet_w.max(x.checked_add(tw).ok_or("sprite sheet overflow")?);
            row_h = row_h.max(th);
            x = x
                .checked_add(tw)
                .and_then(|v| v.checked_add(padding))
                .ok_or("sprite sheet overflow")?;
            col += 1;
        }
        let mut sheet_h = y.checked_add(row_h).ok_or("sprite sheet overflow")?;
        if pack_mode == "autotile" {
            sheet_w = sheet_w
                .checked_add(1)
                .ok_or("sprite sheet overflow")?;
            sheet_h = sheet_h
                .checked_add(1)
                .ok_or("sprite sheet overflow")?;
        }
        if sheet_w == 0
            || sheet_h == 0
            || sheet_w > profile.max_dim
            || sheet_h > profile.max_dim
        {
            return Err(format!(
                "sprite sheet dimensions outside 1..{}",
                profile.max_dim
            ));
        }
        for i in 0..frames.len() {
            for j in (i + 1)..frames.len() {
                if rects_overlap(
                    frames[i].x,
                    frames[i].y,
                    frames[i].w,
                    frames[i].h,
                    frames[j].x,
                    frames[j].y,
                    frames[j].w,
                    frames[j].h,
                ) {
                    return Err("sprite sheet frames overlap".into());
                }
            }
        }
        let n = (sheet_w as usize)
            .checked_mul(sheet_h as usize)
            .ok_or("sprite sheet overflow")?;
        self.charge(n, n.saturating_mul(4))?;
        let mut dest = vec![[0u8; 4]; n];
        for (i, t) in tiles_v.iter().enumerate() {
            let r = self.rgba(t)?;
            blit_opaque(
                &mut dest,
                sheet_w,
                sheet_h,
                &r.pixels,
                r.width,
                r.height,
                frames[i].x,
                frames[i].y,
            );
            if pack_mode == "autotile" {
                copy_autotile_gutter(
                    &mut dest,
                    sheet_w,
                    sheet_h,
                    frames[i].x,
                    frames[i].y,
                    frames[i].w,
                    frames[i].h,
                );
            }
        }
        let image = self.store_rgba(sheet_w, sheet_h, dest)?;
        let png = self.encode_rgba_png_handle(image.clone(), a.get("profile"))?;
        let mut frame_list = Vec::with_capacity(frames.len());
        for f in frames {
            let mut rec = BTreeMap::new();
            rec.insert("name".into(), Value::String(f.name));
            rec.insert("x".into(), Value::U64(f.x as u64));
            rec.insert("y".into(), Value::U64(f.y as u64));
            rec.insert("w".into(), Value::U64(f.w as u64));
            rec.insert("h".into(), Value::U64(f.h as u64));
            if pack_mode == "autotile" {
                rec.insert("wrap".into(), Value::String(wrap.to_string()));
                rec.insert("mode".into(), Value::String("autotile".into()));
            }
            frame_list.push(Value::Record(rec));
        }
        let mut out = BTreeMap::new();
        out.insert("image".into(), image);
        out.insert("png".into(), png);
        out.insert("width".into(), Value::U64(sheet_w as u64));
        out.insert("height".into(), Value::U64(sheet_h as u64));
        out.insert("frames".into(), Value::List(frame_list));
        out.insert("mode".into(), Value::String(pack_mode.to_string()));
        out.insert("wrap".into(), Value::String(wrap.to_string()));
        Ok(Value::Record(out))
    }

    pub(crate) fn pack_nine_slice(&mut self, a: &Args) -> Result<Value, String> {
        fields(
            a,
            &[
                "image",
                "left",
                "right",
                "top",
                "bottom",
                "dest_width",
                "dest_height",
                "profile",
            ],
        )?;
        let profile = lattice_profile(a)?;
        let (src_w, src_h, src_pixels) = {
            let src = self.rgba(required(a, "image")?)?;
            (src.width, src.height, src.pixels.clone())
        };
        if src_w == 0 || src_h == 0 || src_w > profile.max_dim || src_h > profile.max_dim {
            return Err(format!("nine-slice source outside 1..{}", profile.max_dim));
        }
        let left = nonnegative(a, "left", 0, src_w as usize)? as u32;
        let right = nonnegative(a, "right", 0, src_w as usize)? as u32;
        let top = nonnegative(a, "top", 0, src_h as usize)? as u32;
        let bottom = nonnegative(a, "bottom", 0, src_h as usize)? as u32;
        if left.saturating_add(right) > src_w || top.saturating_add(bottom) > src_h {
            return Err("nine-slice insets exceed source".into());
        }
        let dest_width = integer(required(a, "dest_width")?, "dest_width")?;
        let dest_height = integer(required(a, "dest_height")?, "dest_height")?;
        if dest_width == 0
            || dest_height == 0
            || dest_width > profile.max_dim as usize
            || dest_height > profile.max_dim as usize
        {
            return Err(format!(
                "dest dimensions outside 1..{}",
                profile.max_dim
            ));
        }
        let dest_w = dest_width as u32;
        let dest_h = dest_height as u32;
        if dest_w < left.saturating_add(right) || dest_h < top.saturating_add(bottom) {
            return Err("dest smaller than nine-slice insets".into());
        }
        let n = dest_width
            .checked_mul(dest_height)
            .ok_or("nine-slice dest overflow")?;
        self.charge(n, n.saturating_mul(4))?;
        let mut dest = vec![[0u8; 4]; n];
        let src_cw = src_w - left - right;
        let src_ch = src_h - top - bottom;
        let dst_cw = dest_w - left - right;
        let dst_ch = dest_h - top - bottom;
        blit_rect_nn(
            &mut dest, dest_w, dest_h, 0, 0, left, top, &src_pixels, src_w, src_h, 0, 0, left, top,
        );
        blit_rect_nn(
            &mut dest, dest_w, dest_h, dest_w - right, 0, right, top, &src_pixels, src_w, src_h,
            src_w - right, 0, right, top,
        );
        blit_rect_nn(
            &mut dest, dest_w, dest_h, 0, dest_h - bottom, left, bottom, &src_pixels, src_w, src_h,
            0, src_h - bottom, left, bottom,
        );
        blit_rect_nn(
            &mut dest, dest_w, dest_h, dest_w - right, dest_h - bottom, right, bottom, &src_pixels,
            src_w, src_h, src_w - right, src_h - bottom, right, bottom,
        );
        blit_rect_nn(
            &mut dest, dest_w, dest_h, left, 0, dst_cw, top, &src_pixels, src_w, src_h, left, 0,
            src_cw, top,
        );
        blit_rect_nn(
            &mut dest, dest_w, dest_h, left, dest_h - bottom, dst_cw, bottom, &src_pixels, src_w,
            src_h, left, src_h - bottom, src_cw, bottom,
        );
        blit_rect_nn(
            &mut dest, dest_w, dest_h, 0, top, left, dst_ch, &src_pixels, src_w, src_h, 0, top, left,
            src_ch,
        );
        blit_rect_nn(
            &mut dest, dest_w, dest_h, dest_w - right, top, right, dst_ch, &src_pixels, src_w, src_h,
            src_w - right, top, right, src_ch,
        );
        blit_rect_nn(
            &mut dest, dest_w, dest_h, left, top, dst_cw, dst_ch, &src_pixels, src_w, src_h, left,
            top, src_cw, src_ch,
        );
        let image = self.store_rgba(dest_w, dest_h, dest)?;
        let png = self.encode_rgba_png_handle(image.clone(), a.get("profile"))?;
        let mut insets = BTreeMap::new();
        insets.insert("left".into(), Value::U64(left as u64));
        insets.insert("right".into(), Value::U64(right as u64));
        insets.insert("top".into(), Value::U64(top as u64));
        insets.insert("bottom".into(), Value::U64(bottom as u64));
        let mut out = BTreeMap::new();
        out.insert("image".into(), image);
        out.insert("png".into(), png);
        out.insert("insets".into(), Value::Record(insets));
        out.insert("dest_width".into(), Value::U64(dest_w as u64));
        out.insert("dest_height".into(), Value::U64(dest_h as u64));
        Ok(Value::Record(out))
    }

    pub(crate) fn pack_pbr_set(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["colour", "normal", "orm", "profile"])?;
        let colour_h = required(a, "colour")?;
        let normal_h = required(a, "normal")?;
        let orm_h = required(a, "orm")?;
        let mut work = 0usize;
        for h in [colour_h, normal_h, orm_h] {
            work = work.saturating_add(match self.workspace.get(h)? {
                Artifact::Rgba(r) => r.pixels.len(),
                Artifact::Encoded(r) if r.kind == "png" => r.bytes.len(),
                _ => return Err("Expected rgba or png handle".into()),
            });
        }
        self.charge(work.max(1), work.saturating_mul(4))?;
        let profile = a.get("profile");
        let colour = self.png_or_encode(colour_h, profile)?;
        let normal = self.png_or_encode(normal_h, profile)?;
        let orm = self.png_or_encode(orm_h, profile)?;
        let mut out = BTreeMap::new();
        out.insert("colour".into(), colour);
        out.insert("normal".into(), normal);
        out.insert("orm".into(), orm);
        Ok(Value::Record(out))
    }
}
