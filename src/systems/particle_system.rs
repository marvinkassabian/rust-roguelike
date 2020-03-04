use rltk::{ColorPair, Point, RGB};
use specs::prelude::*;

use crate::{Context, ParticleLifetime, Position, Renderable, RenderAura, RenderBackground};

pub fn cull_dead_particles(ecs: &mut World, context: &mut Context) {
    let mut dead_particles: Vec<Entity> = Vec::new();
    {
        let mut particles = ecs.write_storage::<ParticleLifetime>();
        let entities = ecs.entities();
        for (entity, mut particle) in (&entities, &mut particles).join() {
            particle.lifetime_ms -= context.rltk.frame_time_ms;
            if particle.lifetime_ms < 0.0 {
                dead_particles.push(entity);
            }
        }
    }
    for dead in dead_particles.iter() {
        ecs.delete_entity(*dead).expect("Particle will not die");
    }
}

//TODO figure out better naming convention
enum ParticleRequestType {
    Entity {
        color: ColorPair,
        glyph: u8,
    },
    Background {
        bg: RGB,
    },
    Aura {
        fg: RGB,
        glyph: u8,
    },
}

pub struct ParticleRequest {
    position: Point,
    request_type: ParticleRequestType,
    lifetime: f32,
}

pub struct ParticleBuilder {
    requests: Vec<ParticleRequest>
}

impl ParticleBuilder {
    pub fn new() -> ParticleBuilder {
        ParticleBuilder { requests: Vec::new() }
    }

    pub fn request_entity(&mut self, position: Point, lifetime: f32, color: ColorPair, glyph: u8) {
        self.requests.push(ParticleRequest {
            position,
            request_type: ParticleRequestType::Entity {
                color,
                glyph,
            },
            lifetime,
        });
    }

    pub fn request_background(&mut self, position: Point, lifetime: f32, bg: RGB) {
        self.requests.push(ParticleRequest {
            position,
            request_type: ParticleRequestType::Background { bg },
            lifetime,
        });
    }

    pub fn request_aura(&mut self, position: Point, lifetime: f32, fg: RGB, glyph: u8) {
        self.requests.push(ParticleRequest {
            position,
            request_type: ParticleRequestType::Aura {
                fg,
                glyph,
            },
            lifetime,
        });
    }
}

pub struct ParticleSpawnSystem;

impl<'a> System<'a> for ParticleSpawnSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, RenderBackground>,
        WriteStorage<'a, RenderAura>,
        WriteStorage<'a, ParticleLifetime>,
        WriteExpect<'a, ParticleBuilder>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut positions,
            mut renderables,
            mut render_backgrounds,
            mut render_auras,
            mut particles,
            mut particle_builder
        ) = data;

        for new_particle in particle_builder.requests.iter() {
            let particle_entity = entities.create();
            positions.insert(
                particle_entity,
                Position {
                    x: new_particle.position.x,
                    y: new_particle.position.y,
                },
            ).expect("Unable to insert position");
            particles.insert(
                particle_entity,
                ParticleLifetime { lifetime_ms: new_particle.lifetime },
            ).expect("Unable to insert lifetime");

            match new_particle.request_type {
                ParticleRequestType::Entity { color, glyph } => {
                    renderables.insert(
                        particle_entity,
                        Renderable {
                            fg: color.fg,
                            bg: color.bg,
                            glyph,
                            render_order: 0,
                        },
                    ).expect("Unable to insert renderable");
                }
                ParticleRequestType::Background { bg } => {
                    render_backgrounds.insert(
                        particle_entity,
                        RenderBackground { bg },
                    ).expect("Unable to insert render background");
                }
                ParticleRequestType::Aura { fg, glyph } => {
                    render_auras.insert(
                        particle_entity,
                        RenderAura { fg, glyph },
                    ).expect("Unable to insert render background");
                }
            }
        }

        particle_builder.requests.clear();
    }
}