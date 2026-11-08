//! Presentation slide show navigation and state machine.

use crate::model::master::Presentation;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShowAction {
    None,
    PlayStep(usize),
    FinishStep(usize),
    StepBack(usize),
    Slide(usize),
    End,
    Exit,
}

pub fn safe_url(url: &str) -> bool {
    let u = url.trim();
    let Some((scheme, rest)) = u.split_once(':') else { return false };
    match scheme.to_ascii_lowercase().as_str() {
        "http" | "https" => rest.starts_with("//") && rest.len() > 2,
        "mailto" => !rest.is_empty(),
        _ => false,
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum LinkJump {
    Show(ShowAction),
    Open(String),
    Exit,
    None,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Default)]
pub struct ShowState {
    pub slide: usize,
    pub step: usize,
    pub playing: bool,
    pub ended: bool,
}

/// Slide indices visited in order, filtering out hidden slides and applying ranges/custom shows.
pub fn show_order(pres: &Presentation) -> Vec<usize> {
    let n = pres.slides.len();
    let base: Vec<usize> = match pres
        .show
        .custom_show
        .as_ref()
        .and_then(|name| pres.custom_shows.iter().find(|c| &c.name == name))
    {
        Some(cs) => cs
            .slides
            .iter()
            .filter_map(|id| pres.slides.iter().position(|s| s.id == *id))
            .collect(),
        None => match pres.show.range {
            Some((a, b)) => {
                let a = (a.max(1) as usize).saturating_sub(1);
                let b = (b as usize).min(n);
                (a..b).collect()
            }
            None => (0..n).collect(),
        },
    };

    base.into_iter().filter(|&i| !pres.slides[i].hidden).collect()
}

impl ShowState {
    pub fn next(&mut self, pres: &Presentation, steps_on_slide: usize) -> ShowAction {
        if self.ended {
            return ShowAction::Exit;
        }

        if self.step < steps_on_slide {
            let next_step = self.step;
            self.step += 1;
            self.playing = true;
            return ShowAction::PlayStep(next_step);
        }

        let order = show_order(pres);
        let cur_pos = order.iter().position(|&idx| idx == self.slide).unwrap_or(0);

        if cur_pos + 1 < order.len() {
            self.slide = order[cur_pos + 1];
            self.step = 0;
            self.playing = false;
            ShowAction::Slide(self.slide)
        } else if pres.show.loop_continuously && !order.is_empty() {
            self.slide = order[0];
            self.step = 0;
            self.playing = false;
            ShowAction::Slide(self.slide)
        } else {
            self.ended = true;
            ShowAction::End
        }
    }

    pub fn prev(&mut self, pres: &Presentation) -> ShowAction {
        if self.ended {
            self.ended = false;
            return ShowAction::Slide(self.slide);
        }

        if self.step > 0 {
            self.step -= 1;
            self.playing = false;
            return ShowAction::StepBack(self.step);
        }

        let order = show_order(pres);
        let cur_pos = order.iter().position(|&idx| idx == self.slide).unwrap_or(0);

        if cur_pos > 0 {
            self.slide = order[cur_pos - 1];
            self.step = 0;
            self.playing = false;
            ShowAction::Slide(self.slide)
        } else {
            ShowAction::None
        }
    }
}
