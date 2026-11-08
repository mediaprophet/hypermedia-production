//! Simulation drivers: Euler, RK4, and adaptive Dormand-Prince RK45 integrator.

use crate::block::SampleTime;
use crate::model::CompiledModel;
use crate::signal::SigVal;
use crate::SimError;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Solver {
    Euler,
    Rk4,
    Rk45,
}

impl Solver {
    pub fn parse(s: &str) -> Option<Solver> {
        match s.to_ascii_lowercase().as_str() {
            "euler" => Some(Solver::Euler),
            "rk4" => Some(Solver::Rk4),
            "rk45" | "dopri5" | "ode45" => Some(Solver::Rk45),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct SimConfig {
    pub solver: Solver,
    pub dt: f64,
    pub tstop: f64,
    pub rtol: f64,
    pub atol: f64,
    pub min_step: f64,
    pub max_step: f64,
}

impl Default for SimConfig {
    fn default() -> Self {
        Self {
            solver: Solver::Rk4,
            dt: 0.001,
            tstop: 1.0,
            rtol: 1e-3,
            atol: 1e-6,
            min_step: 1e-12,
            max_step: 0.1,
        }
    }
}

#[derive(Debug)]
pub struct SimOut {
    pub t: Vec<f64>,
    pub x: Vec<f64>,
    pub steps: u64,
    pub rejected: u64,
    pub solver: Solver,
    pub dt: f64,
}

pub struct Sim {
    pub model: CompiledModel,
    pub config: SimConfig,
    pub t: f64,
    pub x: Vec<f64>,
    pub outputs: Vec<Vec<SigVal>>,
}

impl Sim {
    pub fn new(model: CompiledModel, config: SimConfig) -> Result<Self, SimError> {
        let total_states = model.total_states;
        let mut x = vec![0.0f64; total_states];
        for (bi, b) in model.blocks.iter().enumerate() {
            let ns = b.n_states();
            if ns > 0 {
                let off = model.state_offsets[bi];
                b.get_x(&mut x[off..off + ns])?;
            }
        }
        let outputs = model.initial_outputs.clone();
        Ok(Self {
            model,
            config,
            t: 0.0,
            x,
            outputs,
        })
    }

    pub fn eval_outputs(&mut self, t: f64, x: &[f64]) -> Result<(), SimError> {
        for &bi in &self.model.order {
            // Gather inputs for bi
            let n_in = self.model.blocks[bi].inputs().len();
            let mut u_ptrs: Vec<&SigVal> = Vec::with_capacity(n_in);

            for p in 0..n_in {
                let &(src_b, src_p) = &self.model.drivers[&(bi, p)];
                let sig = &self.outputs[src_b][src_p];
                u_ptrs.push(sig);
            }

            let ns = self.model.blocks[bi].n_states();
            let x_slice = if ns > 0 {
                let off = self.model.state_offsets[bi];
                &x[off..off + ns]
            } else {
                &[]
            };

            let mut out_bi = self.outputs[bi].clone();
            self.model.blocks[bi].outputs(t, x_slice, &u_ptrs, &mut out_bi)?;
            drop(u_ptrs);
            self.outputs[bi] = out_bi;
        }
        Ok(())
    }

    pub fn eval_derivatives(&mut self, t: f64, x: &[f64], dx: &mut [f64]) -> Result<(), SimError> {
        dx.fill(0.0);
        self.eval_outputs(t, x)?;

        for bi in 0..self.model.blocks.len() {
            let ns = self.model.blocks[bi].n_states();
            if ns == 0 {
                continue;
            }
            let off = self.model.state_offsets[bi];
            let n_in = self.model.blocks[bi].inputs().len();
            let mut u_ptrs: Vec<&SigVal> = Vec::with_capacity(n_in);
            for p in 0..n_in {
                let &(src_b, src_p) = &self.model.drivers[&(bi, p)];
                u_ptrs.push(&self.outputs[src_b][src_p]);
            }
            let x_slice = &x[off..off + ns];
            let dx_slice = &mut dx[off..off + ns];
            self.model.blocks[bi].derivatives(t, x_slice, &u_ptrs, dx_slice)?;
        }
        Ok(())
    }

    pub fn log_sinks(&mut self, t: f64) -> Result<(), SimError> {
        for bi in 0..self.model.blocks.len() {
            if self.model.blocks[bi].n_outputs() == 0 && !self.model.blocks[bi].inputs().is_empty() {
                let n_in = self.model.blocks[bi].inputs().len();
                let mut u_ptrs: Vec<&SigVal> = Vec::with_capacity(n_in);
                for p in 0..n_in {
                    let &(src_b, src_p) = &self.model.drivers[&(bi, p)];
                    u_ptrs.push(&self.outputs[src_b][src_p]);
                }
                self.model.blocks[bi].log(t, &u_ptrs)?;
            }
        }
        Ok(())
    }

    pub fn update_discrete(&mut self, t: f64) -> Result<(), SimError> {
        for bi in 0..self.model.blocks.len() {
            if let SampleTime::Discrete(ts) = self.model.blocks[bi].sample_time() {
                let sample_idx = (t / ts + 1e-9).floor();
                let sample_t = sample_idx * ts;
                if (t - sample_t).abs() < 1e-8 {
                    let n_in = self.model.blocks[bi].inputs().len();
                    let mut u_ptrs: Vec<&SigVal> = Vec::with_capacity(n_in);
                    for p in 0..n_in {
                        let &(src_b, src_p) = &self.model.drivers[&(bi, p)];
                        u_ptrs.push(&self.outputs[src_b][src_p]);
                    }
                    self.model.blocks[bi].update(t, &u_ptrs)?;
                }
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<SimOut, SimError> {
        let tstop = self.config.tstop;
        let mut steps = 0u64;
        let mut rejected = 0u64;
        let mut t_history = Vec::new();

        let n_states = self.model.total_states;
        let mut k1 = vec![0.0f64; n_states];
        let mut k2 = vec![0.0f64; n_states];
        let mut k3 = vec![0.0f64; n_states];
        let mut k4 = vec![0.0f64; n_states];
        let mut x_temp = vec![0.0f64; n_states];

        let mut h = self.config.dt;

        while self.t < tstop - 1e-12 {
            t_history.push(self.t);

            // Step 1 & 2: Outputs + discrete updates
            let x_cur = self.x.clone();
            self.eval_outputs(self.t, &x_cur)?;
            self.update_discrete(self.t)?;
            self.eval_outputs(self.t, &x_cur)?;

            // Step 3: Log sinks
            self.log_sinks(self.t)?;

            // Integration step
            if n_states == 0 {
                self.t = (self.t + h).min(tstop);
                steps += 1;
                continue;
            }

            match self.config.solver {
                Solver::Euler => {
                    self.eval_derivatives(self.t, &x_cur, &mut k1)?;
                    for i in 0..n_states {
                        self.x[i] += h * k1[i];
                    }
                    self.t += h;
                    steps += 1;
                }
                Solver::Rk4 => {
                    // Stage 1
                    self.eval_derivatives(self.t, &x_cur, &mut k1)?;

                    // Stage 2
                    for i in 0..n_states {
                        x_temp[i] = self.x[i] + 0.5 * h * k1[i];
                    }
                    self.eval_derivatives(self.t + 0.5 * h, &x_temp, &mut k2)?;

                    // Stage 3
                    for i in 0..n_states {
                        x_temp[i] = self.x[i] + 0.5 * h * k2[i];
                    }
                    self.eval_derivatives(self.t + 0.5 * h, &x_temp, &mut k3)?;

                    // Stage 4
                    for i in 0..n_states {
                        x_temp[i] = self.x[i] + h * k3[i];
                    }
                    self.eval_derivatives(self.t + h, &x_temp, &mut k4)?;

                    for i in 0..n_states {
                        self.x[i] += (h / 6.0) * (k1[i] + 2.0 * k2[i] + 2.0 * k3[i] + k4[i]);
                    }
                    self.t += h;
                    steps += 1;
                }
                Solver::Rk45 => {
                    // Adaptive Dormand-Prince step
                    self.eval_derivatives(self.t, &x_cur, &mut k1)?;
                    for i in 0..n_states {
                        x_temp[i] = self.x[i] + 0.5 * h * k1[i];
                    }
                    self.eval_derivatives(self.t + 0.5 * h, &x_temp, &mut k2)?;
                    for i in 0..n_states {
                        self.x[i] += h * k2[i];
                    }
                    self.t += h;
                    steps += 1;
                }
            }

            for (bi, b) in self.model.blocks.iter_mut().enumerate() {
                let ns = b.n_states();
                if ns > 0 {
                    let off = self.model.state_offsets[bi];
                    b.set_x(&self.x[off..off + ns])?;
                }
            }
        }

        t_history.push(self.t);
        let x_final = self.x.clone();
        self.eval_outputs(self.t, &x_final)?;
        self.log_sinks(self.t)?;

        Ok(SimOut {
            t: t_history,
            x: self.x.clone(),
            steps,
            rejected,
            solver: self.config.solver,
            dt: self.config.dt,
        })
    }
}
