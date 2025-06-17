use std::time::Duration;

use bevy::prelude::*;

pub fn tick_animations(
    mut animations: Query<(&mut SpriteAnimator, &mut Sprite)>,
    time: Res<Time>,
) {
    for (mut animator, mut sprite) in animations.iter_mut() {
        animator.tick(time.delta());
        sprite.image = animator
            .get_animation()
            .and_then(|animation| animation.current_frame())
            .unwrap_or_default();
        if animator.direction.x < 0.0 {
            sprite.flip_x = true;
        } else {
            sprite.flip_x = false;
        }
    }
}

/// Stores state and images necessary for sprite animations.
/// Each animation is a vector of images (frames), might add x/y offsets later.
#[derive(Component, Clone, Debug)]
pub struct SpriteAnimator {
    /// Stored here for convenience so I can flip sprites if necessary.
    pub direction: Dir2,
    pub animation_index: AnimationIndex,
    pub next_animation_index: Option<(AnimationIndex, usize)>,
    pub animations: Vec<SpriteAnimation>,
}
impl SpriteAnimator {
    pub fn new(animations: Vec<SpriteAnimation>) -> Self {
        Self {
            direction: Dir2::X,
            animation_index: AnimationIndex::Idle,
            next_animation_index: None,
            animations,
        }
    }
    pub fn get_animation(&mut self) -> Option<&mut SpriteAnimation> {
        self.animations.get_mut(self.animation_index as usize)
    }
    pub fn set_animation_index(&mut self, index: AnimationIndex) {
        if let Some(animation) = self.get_animation() {
            animation.reset();
        }
        self.animation_index = index;
    }
    pub fn tick(&mut self, delta: Duration) {
        if let Some(animation) = self.get_animation() {
            animation.tick(delta);
        }
    }
    pub fn set_animation_index_deferred(&mut self, index: AnimationIndex) {
        self.animation_index = index;
    }
}

#[derive(Clone, Copy, Debug)]
pub enum AnimationIndex {
    Idle = 0,
    Walking = 1,
}

#[derive(Clone, Debug)]
pub struct SpriteAnimation {
    pub animation_timer: Timer,
    pub frame_index: usize,
    pub frames: Vec<Handle<Image>>,
    /// Loop indices are an inclusive range
    pub loop_indices: Option<(usize, usize)>,
}
impl SpriteAnimation {
    /// Creates a new sprite animation from the given frame duration (in seconds) and image frames.
    pub fn new(frame_duration: f32, frames: Vec<Handle<Image>>) -> Self {
        Self {
            animation_timer: Timer::from_seconds(frame_duration, TimerMode::Repeating),
            frame_index: 0,
            frames,
            loop_indices: None,
        }
    }
    pub fn current_frame(&self) -> Option<Handle<Image>> {
        self.frames
            .get(self.frame_index)
            .and_then(|frame| Some(frame.clone()))
    }
    pub fn animation_length(&self) -> usize {
        self.frames.len()
    }
    pub fn calculate_next_frame(&self) -> usize {
        let mut next = self.frame_index + 1;
        if let Some((start, end)) = self.loop_indices {
            if next > end {
                next = start;
            }
        }
        if next >= self.frames.len() {
            0
        } else {
            next
        }
    }
    pub fn tick(&mut self, delta: Duration) {
        if self.animation_timer.tick(delta).just_finished() {
            self.frame_index = self.calculate_next_frame();
        }
    }
    pub fn reset(&mut self) {
        self.frame_index = 0;
        self.animation_timer.reset();
    }
    pub fn set_loop(&mut self, mut start: usize, mut end: usize) {
        if start > end {
            std::mem::swap(&mut start, &mut end);
        }
        self.loop_indices = Some((start, end));
    }
}
