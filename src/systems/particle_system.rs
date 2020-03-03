use rltk::{ColorPair, Point};
use specs::prelude::*;

use crate::{Context, ParticleLifetime, Position, Renderable};

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

struct ParticleRequest {
    position: Point,
    color: ColorPair,
    glyph: u8,
    lifetime: f32,
}

pub struct ParticleBuilder {
    requests: Vec<ParticleRequest>
}

impl ParticleBuilder {
    pub fn new() -> ParticleBuilder {
        ParticleBuilder { requests: Vec::new() }
    }

    pub fn request(&mut self, position: Point, color: ColorPair, glyph: u8, lifetime: f32) {
        self.requests.push(
            ParticleRequest {
                position,
                color,
                glyph,
                lifetime,
            }
        );
    }
}

pub struct ParticleSpawnSystem;

impl<'a> System<'a> for ParticleSpawnSystem {
    type SystemData = (
        Entities<'a>,
        WriteStorage<'a, Position>,
        WriteStorage<'a, Renderable>,
        WriteStorage<'a, ParticleLifetime>,
        WriteExpect<'a, ParticleBuilder>
    );

    fn run(&mut self, data: Self::SystemData) {
        let (
            entities,
            mut positions,
            mut renderables,
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
            renderables.insert(
                particle_entity,
                Renderable {
                    fg: new_particle.color.fg,
                    bg: new_particle.color.bg,
                    glyph: new_particle.glyph,
                    render_order: 0,
                },
            ).expect("Unable to insert renderable");
            particles.insert(
                particle_entity,
                ParticleLifetime { lifetime_ms: new_particle.lifetime },
            ).expect("Unable to insert lifetime");
        }

        particle_builder.requests.clear();
    }
}