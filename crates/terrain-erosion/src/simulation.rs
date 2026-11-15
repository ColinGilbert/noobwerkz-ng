use std::thread;

use glam::FloatExt;
use noobwerkz_shared::grid2d::Grid2D;
use rand::distr::Distribution;

const L_X: f32 = 1.0;
const L_Y: f32 = 1.0;
const GRAVITY: f32 = 9.81;

pub struct TerrainErosionSimulation {
    water: Grid2D<f32>,
    terrain: Grid2D<f32>,
    sediment: Grid2D<f32>,
    tmp_sediment: Grid2D<f32>,
    u_vel: Grid2D<f32>,
    v_vel: Grid2D<f32>,
    l_flux: Grid2D<f32>,
    r_flux: Grid2D<f32>,
    t_flux: Grid2D<f32>,
    b_flux: Grid2D<f32>,
    rain_pos: glam::USizeVec2,
    num_cpus: usize,
}

impl TerrainErosionSimulation {
    pub fn new(dimension: usize) -> Self {
        Self {
            water: Grid2D::new(dimension, dimension, 0.0),
            terrain: Grid2D::new(dimension, dimension, 0.0),
            sediment: Grid2D::new(dimension, dimension, 0.0),
            tmp_sediment: Grid2D::new(dimension, dimension, 0.0),
            u_vel: Grid2D::new(dimension, dimension, 0.0),
            v_vel: Grid2D::new(dimension, dimension, 0.0),
            l_flux: Grid2D::new(dimension, dimension, 0.0),
            r_flux: Grid2D::new(dimension, dimension, 0.0),
            t_flux: Grid2D::new(dimension, dimension, 0.0),
            b_flux: Grid2D::new(dimension, dimension, 0.0),
            rain_pos: glam::USizeVec2::new(dimension / 2, dimension / 2),
            num_cpus: num_cpus::get_physical(),
        }
    }

    pub fn update(&mut self, dt: web_time::Duration, rain: bool, flood: bool) {
        // 1. Add water to the system
        if rain {
            self.make_rain();
        }
        if flood {
            self.make_flood(dt);
        }
        // 2. Simulate Flow
        self.simulate_flow(dt);
        // 3. Simulate Errosion-deposition
        self.simulate_erosion();
        // 4. Advection of suspended sediment
        self.simulate_sediment_transportation(dt);
        // 5. Simulate Evaporation
        self.simulate_evaporation(dt);
    }

    pub fn simulate_flow(&mut self, dt: web_time::Duration) {
        // Outflux computation settings
        let l = 1.0;
        const A: f32 = 0.00005;
        const DX: f32 = L_X;
        const DY: f32 = L_Y;
        let dt = dt.as_secs_f32();
        let flux_factor = dt * A * GRAVITY / l;

        // Outflow Flux Computation with boundary conditions
        // TODO: Parallelize
        for y in 0..self.u_vel.height() {
            for x in 0..self.u_vel.width() {
                let h0 = self.terrain.get(x, y) + self.water.get(x, y); // water height at current cell
                let mut dh; // height difference
                let mut new_flux;

                // left outflow
                if x > 0 {
                    dh = h0 - (self.terrain.get(x - 1, y) + self.water.get(x - 1, y));
                    new_flux = self.l_flux.get(x, y) + flux_factor * dh;
                    self.l_flux.set(x, y, 0.0f32.max(new_flux));
                } else {
                    self.l_flux.set(x, y, 0.0); // boundary
                }

                // right outflow
                if x < self.water.width() - 1 {
                    dh = h0 - (self.terrain.get(x + 1, y) + self.water.get(x + 1, y));
                    new_flux = self.r_flux.get(x, y) + flux_factor * dh;
                    self.r_flux.set(x, y, 0.0f32.max(new_flux));
                } else {
                    self.r_flux.set(x, y, 0.0); // boundary
                }

                // bottom outflow
                if y > 0 {
                    dh = h0 - (self.terrain.get(x, y - 1) + self.water.get(x, y - 1));
                    new_flux = self.b_flux.get(x, y) + flux_factor * dh;
                    self.b_flux.set(x, y, 0.0f32.max(new_flux));
                } else {
                    self.b_flux.set(x, y, 0.0); // boundary
                }

                // top outflow
                if y < self.water.height() - 1 {
                    dh = h0 - (self.terrain.get(x, y + 1) + self.water.get(x, y + 1));
                    new_flux = self.t_flux.get(x, y) + flux_factor * dh;
                    self.t_flux.set(x, y, 0.0f32.max(new_flux));
                } else {
                    self.t_flux.set(x, y, 0.0); // boundary
                }

                // scaling
                let sum_flux = self.l_flux.get(x, y)
                    + self.r_flux.get(x, y)
                    + self.b_flux.get(x, y)
                    + self.t_flux.get(x, y);
                let k = 1.0f32.min((self.water.get(x, y) * DX * DY) / (sum_flux * dt));
                let val = self.r_flux.get(x, y) * k;
                self.r_flux.set(x, y, val);
                let val = self.l_flux.get(x, y) * k;
                self.l_flux.set(x, y, val);
                let val = self.t_flux.get(x, y) * k;

                self.t_flux.set(x, y, val);
                let val = self.b_flux.get(x, y) * k;
                self.b_flux.set(x, y, val);
            }
        }

        // Update water surface and velocity field
        // TODO: Parallelize
        for y in 0..self.u_vel.height() {
            for x in 0..self.u_vel.width() {
                let in_flow = self.get_r_flux(x - 1, y)
                    + self.get_l_flux(x + 1, y)
                    + self.get_t_flux(x, y - 1)
                    + self.get_b_flux(x, y + 1);
                let out_flow = self.get_r_flux(x, y)
                    + self.get_l_flux(x, y)
                    + self.get_t_flux(x, y)
                    + self.get_b_flux(x, y);
                let d_v = dt * (in_flow - out_flow);
                let old_water = self.water.get(x, y);
                let val = (self.water.get(x, y) + (d_v / (DX * DY))).max(0.0);

                let mean_water = 0.5 * (old_water + val);
                self.water.set(x, y, val);

                if mean_water == 0.0 {
                    self.u_vel.set(x, y, 0.0);
                    self.v_vel.set(x, y, 0.0);
                } else {
                    self.u_vel.set(
                        x,
                        y,
                        0.5 * (self.get_r_flux(x - 1, y)
                            - self.get_l_flux(x, y)
                            - self.get_l_flux(x + 1, y)
                            + self.get_r_flux(x, y))
                            / (DY * mean_water),
                    );
                    self.v_vel.set(
                        x,
                        y,
                        0.5 * (self.get_t_flux(x, y - 1)
                            - self.get_b_flux(x, y)
                            - self.get_b_flux(x, y + 1)
                            + self.get_t_flux(x, y))
                            / (DX * mean_water),
                    );
                }
            }
        }
    }

    pub fn simulate_erosion(&mut self) {
        const KC: f32 = 25.0; // sediment capacity constant
        const KS: f32 = 0.0001 * 12.0 * 10.0; // dissolving constant
        const KD: f32 = 0.0001 * 12.0 * 10.0; // deposition constant

        let chunk_size = self.sediment.height() / self.num_cpus;
        let end = self.sediment.height();
        let chunked_ranges = (0..end)
            .step_by(chunk_size)
            .map(|i| i..(i + chunk_size).min(end));
        for sub_range in chunked_ranges {
            thread::scope(|s| {
                s.spawn(|| {
                    for y in sub_range {
                        for x in 0..self.sediment.width() {
                            // local velocity
                            let u_v = self.u_vel.get(x, y);
                            let v_v = self.v_vel.get(x, y);

                            // local terrain normal
                            let normal = glam::Vec3::new(
                                self.get_terrain(x + 1, y) - self.get_terrain(x - 1, y),
                                self.get_terrain(x, y + 1) - self.get_terrain(x, y - 1),
                                2.0,
                            )
                            .normalize();
                            const UP: glam::Vec3 = glam::Vec3::Z;
                            let cos_a = normal.dot(UP);
                            let sin_alpha = cos_a.acos().sin().max(0.1); //acos(cosa));

                            // local sediment capacity of the flow
                            let capacity = KC
                                * (u_v * u_v + v_v * v_v).sqrt()
                                * sin_alpha
                                * (self.water.get(x, y).min(0.01) / 0.01);
                            // (std::min(water(y,x), 0.01f)/0.01f) ;
                            let delta = capacity - self.sediment.get(x, y);

                            // Found to be unnecessary.
                            // let v = (u_v*u_v+v_v*v_v).sqrt();
                            // let fctr = self.water.get(x, y).min(0.01) / 0.01;

                            if delta > 0.0 {
                                let d = KS * delta;
                                let mut val = self.terrain.get(x, y) - d;
                                self.terrain.set(x, y, val);
                                val = self.water.get(x, y) + d;
                                self.water.set(x, y, val);
                                val = self.sediment.get(x, y) + d;
                                self.sediment.set(x, y, val);
                            }
                            // deposit onto ground
                            else if delta < 0.0 {
                                let d = KD * delta;
                                let mut val = self.terrain.get(x, y) - d;
                                self.terrain.set(x, y, val);
                                val = self.water.get(x, y) + d;
                                self.water.set(x, y, val);
                                val = self.sediment.get(x, y) + d;
                                self.sediment.set(x, y, val);
                            }
                        }
                    }
                });
            });
        }
    }

    // TODO: Parallelize
    pub fn simulate_sediment_transportation(&mut self, dt: web_time::Duration) {
        let dt = dt.as_secs_f32();
        for y in 0..self.sediment.height() {
            for x in 0..self.sediment.width() {
                // local velocity
                let u_v = self.u_vel.get(x, y);
                let v_v = self.v_vel.get(x, y);
                // position where flow comes from
                let from_pos_x = x as f32 - u_v * dt;
                let from_pos_y = y as f32 - v_v * dt;
                // integer coordinates
                let x0 = from_pos_x.floor() as usize;
                let y0 = from_pos_y.floor() as usize;
                let x1 = x0 + 1;
                let y1 = y0 + 1;
                // interpolation factors
                let f_x = from_pos_x - x0 as f32;
                let f_y = from_pos_y - y0 as f32;
                // clamp to grid borders
                let x0 = x0.clamp(0, self.sediment.width() - 1);
                let x1 = x1.clamp(0, self.sediment.width() - 1);
                let y0 = y0.clamp(0, self.sediment.height() - 1);
                let y1 = y1.clamp(0, self.sediment.height() - 1);
                let term1 = self
                    .sediment
                    .get(x0, y0)
                    .lerp(*self.sediment.get(x1, y0), f_x);
                let term2 = self
                    .sediment
                    .get(x0, y1)
                    .lerp(*self.sediment.get(x1, y1), f_x);
                let val = term1.lerp(term2, f_y);
                self.tmp_sediment.set(x, y, val);
            }
        }

        for i in 0..self.sediment.size() {
            self.sediment
                .set_at_index(i, *self.tmp_sediment.get_at_index(i));
        }
    }

    // TODO: Parallelize
    pub fn simulate_evaporation(&mut self, dt: web_time::Duration) {
        const KE: f32 = 0.00011 * 0.5; // evaporation constant
        let dt = dt.as_secs_f32();

        for y in 0..self.water.height() {
            for x in 0..self.water.width() {
                let val = (self.water.get(x, y) * (1.0 - KE * dt)).max(0.0);
                self.water.set(x, y, val);

                if *self.water.get(x, y) < 0.005f32 {
                    self.water.set(x, y, 0.0);
                }
            }
        }
    }

    pub fn make_rain(&mut self) {
        let mut rng = rand::rng();
        let die = rand::distr::Uniform::new_inclusive(1, self.water.width() - 2).unwrap();
        for _ in 0..100 {
            let x = die.sample(&mut rng);
            let y = die.sample(&mut rng);

            let val = self.water.get(x - 1, y - 1) + 1.0 / 16.0;
            self.water.set(x - 1, y - 1, val);
            let val = self.water.get(x, y - 1) + 1.0 / 16.0;
            self.water.set(x, y - 1, val);
            let val = self.water.get(x + 1, y - 1) + 1.0 / 16.0;
            self.water.set(x + 1, y - 1, val);
            let val = self.water.get(x - 1, y) + 1.0 / 16.0;
            self.water.set(x - 1, y, val);
            let val = self.water.get(x, y) + 1.0 / 16.0;
            self.water.set(x, y, val);
            let val = self.water.get(x + 1, y) + 1.0 / 16.0;
            self.water.set(x + 1, y, val);
            let val = self.water.get(x - 1, y + 1) + 1.0 / 16.0;
            self.water.set(x - 1, y + 1, val);
            let val = self.water.get(x, y + 1) + 1.0 / 16.0;
            self.water.set(x, y + 1, val);
            let val = self.water.get(x + 1, y + 1) + 1.0 / 16.0;
            self.water.set(x + 1, y + 1, val);
        }
    }

    pub fn make_flood(&mut self, dt: web_time::Duration) {
        let rain_pos = self.rain_pos;
        self.add_raindrop(&rain_pos, 10, dt.as_secs_f32() * 0.01);
    }

    fn add_raindrop(&mut self, pos: &glam::USizeVec2, rad: usize, amount: f32) {
        let rad = rad as isize;
        let rad2 = rad * rad;
        for y in -rad..rad {
            {
                let pos_y = pos.y as isize + y;
                for x in -rad..rad {
                    let pos_x = pos.x as isize + x;
                    if pos_x >= 0
                        && pos_y >= 0
                        && pos_x < self.terrain.width() as isize
                        && pos_y < self.terrain.height() as isize
                    {
                        let d = x * x + y * y;
                        if d <= rad2 {
                            let a = amount * (rad2 as f32 - d as f32);
                            let val = self.water.get(pos_x as usize, pos_y as usize) + a;
                            self.water.set(pos_x as usize, pos_y as usize, val);
                        }
                    }
                }
            }
        }
    }

    #[inline(always)]
    pub fn get_r_flux(&self, x: usize, y: usize) -> f32 {
        if x > self.r_flux.width() - 1 {
            return 0.0;
        } else {
            return *self.r_flux.get(x, y);
        }
    }
    #[inline(always)]
    pub fn get_l_flux(&self, x: usize, y: usize) -> f32 {
        if x > self.l_flux.width() - 1 {
            return 0.0;
        } else {
            return *self.l_flux.get(x, y);
        }
    }
    #[inline(always)]
    pub fn get_b_flux(&self, x: usize, y: usize) -> f32 {
        if y > self.b_flux.height() - 1 {
            return 0.0;
        } else {
            return *self.b_flux.get(x, y);
        }
    }
    #[inline(always)]
    pub fn get_t_flux(&self, x: usize, y: usize) -> f32 {
        if y > self.t_flux.height() - 1 {
            return 0.0;
        } else {
            return *self.t_flux.get(x, y);
        }
    }
    #[inline(always)]
    pub fn get_terrain(&self, x: usize, y: usize) -> f32 {
        return *self.terrain.get(
            x.clamp(0, self.terrain.width() - 1),
            y.clamp(0, self.terrain.height() - 1),
        );
    }
    #[inline(always)]
    pub fn get_water(&self, x: usize, y: usize) -> f32 {
        return *self.water.get(
            x.clamp(0, self.water.width() - 1),
            y.clamp(0, self.water.height() - 1),
        );
    }
}
