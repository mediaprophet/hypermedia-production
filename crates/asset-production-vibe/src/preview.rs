//! Portable review output, without a server, persistent host or external fetches.
use super::*;
use base64::{engine::general_purpose::STANDARD, Engine};
fn escape(s: &str) -> String {
    s.replace('&', "&amp;")
        .replace('<', "&lt;")
        .replace('>', "&gt;")
        .replace('"', "&quot;")
        .replace('\'', "&#39;")
}
impl AssetHost<'_> {
    pub(super) fn gallery(&mut self, a: &Args) -> Result<Value, String> {
        fields(a, &["title", "files"])?;
        let title = a
            .get("title")
            .map(text)
            .transpose()?
            .unwrap_or("Asset preview");
        if title.len() > 1024 {
            return Err("Preview title exceeds 1024 bytes".into());
        }
        let files = list(required(a, "files")?)?;
        if files.len() > 128 {
            return Err("At most 128 preview resources".into());
        }
        let mut estimate = 8192 + title.len() * 6;
        for f in files {
            let f = record(f)?;
            fields(f, &["name", "resource"])?;
            let name = text(required(f, "name")?)?;
            if name.len() > 1024 {
                return Err("Preview name exceeds 1024 bytes".into());
            }
            let resource = self.workspace.encoded(required(f, "resource")?)?;
            estimate = estimate
                .checked_add(
                    resource
                        .bytes
                        .len()
                        .checked_mul(2)
                        .ok_or("Preview size overflow")?
                        + name.len() * 6
                        + 1024,
                )
                .ok_or("Preview size overflow")?;
        }
        if estimate > self.limits.resource_bytes {
            return Err("Preview exceeds resource byte budget".into());
        }
        self.charge(estimate, estimate)?;
        let mut html=String::from("<!doctype html><html lang=\"en\"><meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width,initial-scale=1\"><meta http-equiv=\"Content-Security-Policy\" content=\"default-src 'none'; img-src data:; media-src data:; style-src 'unsafe-inline'; base-uri 'none'; form-action 'none'\"><title>");
        html.push_str(&escape(title));
        html.push_str("</title><style>body{font:16px system-ui;margin:32px;background:#101923;color:#eaf1f8}h1{font-size:28px}main{display:grid;grid-template-columns:repeat(auto-fit,minmax(280px,1fr));gap:20px}article{min-width:0;padding:20px;background:#1c2937;border-radius:12px}h2{font-size:16px;overflow-wrap:anywhere}img{display:block;max-width:100%;height:auto;background:repeating-conic-gradient(#ccd1d6 0% 25%,#fff 0% 50%) 0/20px 20px;border-radius:6px}audio{max-width:100%}p{color:#b6c7d8}</style><h1>");
        html.push_str(&escape(title));
        html.push_str("</h1><main>");
        for f in files {
            let f = record(f)?;
            let name = escape(text(required(f, "name")?)?);
            let r = self.workspace.encoded(required(f, "resource")?)?;
            html.push_str(&format!("<article><h2>{name}</h2>"));
            let mime = match r.kind.as_str() {
                "png" => Some("image/png"),
                "svg" => Some("image/svg+xml"),
                "jpeg" | "jpg" => Some("image/jpeg"),
                "webp" => Some("image/webp"),
                "wav" => Some("audio/wav"),
                _ => None,
            };
            if let Some(mime) = mime {
                let data = STANDARD.encode(&r.bytes);
                if mime.starts_with("image/") {
                    html.push_str(&format!(
                        "<img alt=\"{name}\" src=\"data:{mime};base64,{data}\">"
                    ));
                } else {
                    html.push_str(&format!("<audio controls preload=\"none\" src=\"data:{mime};base64,{data}\"></audio>"));
                }
            } else {
                html.push_str("<p>Use the format's native preview tool for this resource.</p>");
            }
            html.push_str(&format!(
                "<p>{} · {} bytes</p></article>",
                escape(&r.kind),
                r.bytes.len()
            ));
        }
        html.push_str("</main></html>");
        self.encode("html", html.into_bytes())
    }
}
