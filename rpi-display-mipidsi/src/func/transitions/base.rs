//! The base transition function.
//!

use crate::foreign_types::*;
use std::{sync::OnceLock, time::Duration};
use tokio::time::Instant;

use super::traits::DrawTransition;

/// Transition between two images.
pub struct Transition<'a, COLOUR, T1, T2, F, DT>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T1: ImageDrawable<Color = COLOUR> + 'a,
    T2: ImageDrawable<Color = COLOUR> + 'a,
    F: DrawTransition<'a, COLOUR, T1, T2, DT>,
    DT: DrawTarget<Color = COLOUR>,
{
    target: &'a mut DT,

    from: &'a T1,
    to: &'a T2,
    steps: u32,
    step: u32,

    start_time: OnceLock<Instant>,
    step_duration: Duration,

    transition: F,

    #[cfg(feature = "debug")]
    skipped_count: u32,
}

impl<'a, COLOUR, T1, T2, F, DT> Transition<'a, COLOUR, T1, T2, F, DT>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T1: ImageDrawable<Color = COLOUR> + 'a,
    T2: ImageDrawable<Color = COLOUR> + 'a,
    F: DrawTransition<'a, COLOUR, T1, T2, DT>,
    DT: DrawTarget<Color = COLOUR>,
{
    /// Create a new transition.
    pub fn new(
        target: &'a mut DT,
        from: &'a T1,
        to: &'a T2,
        transition: F,
        steps: u32,
        duration: Duration,
    ) -> Self {
        let step_duration = duration / steps;

        Self {
            target,
            from,
            to,
            steps,
            step: 0,

            start_time: OnceLock::new(),
            step_duration,

            transition,

            #[cfg(feature = "debug")]
            skipped_count: 0,
        }
    }

    /// Get the start time of the transition; if it has not been started yet,
    /// start it now.
    fn start_time(&self) -> &Instant {
        self.start_time.get_or_init(Instant::now)
    }

    /// Get the current step of the transition.
    ///
    /// This calculation is based on the start time of the transition, and the
    /// duration of each step. This means that steps can be skipped if the
    /// transition has taken more than the desired frame time. Skipped frames
    /// would not be calculated at all.
    ///
    /// The transition will simply try its best to keep up with the desired
    /// frame rate.
    fn calculate_current_step(&self) -> u32 {
        let elapsed = self.start_time().elapsed();
        let step = (elapsed.as_millis() / self.step_duration.as_millis()) as u32;

        step.max(self.step)
    }

    /// Get the deadline for the given frame.
    fn deadline(&self, step: u32) -> Instant {
        *self.start_time() + self.step_duration * step
    }

    /// Draw the entire transition to the given draw target. On failure, return a
    /// [`RPiError`] in the [`RPiResult`].
    pub async fn start<'e>(&mut self) -> RPiResult<'e, ()>
    where
        'a: 'e,
    {
        loop {
            let step = self.calculate_current_step();

            // The second check is necessary to confirm that we have at least issued the
            // last step of the transition once.
            if step > self.steps && self.step > self.steps {
                #[cfg(feature = "debug")]
                {
                    let rendered_count = self.step - self.skipped_count;
                    logger::info(&format!(
                        "Transition finished, rendered {} frames, skipped {} frames @ {} FPS.",
                        rendered_count,
                        self.skipped_count,
                        rendered_count as f32 / self.start_time().elapsed().as_secs_f32()
                    ));
                }

                break;
            }

            self.transition
                .draw_frame(self.target, self.from, self.to, self.step)?;

            #[cfg(feature = "debug")]
            if step > self.step + 1 {
                let skipped = step - self.step - 1;
                logger::debug(&format!("Frame overtime, skipping {} frames.", skipped));
                self.skipped_count += skipped;
            }

            self.step = step;
            tokio::time::sleep_until(self.deadline(step + 1)).await;
        }

        Ok(())
    }
}

impl<'a, COLOUR, T, F, DT> Transition<'a, COLOUR, T, T, F, DT>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T: ImageDrawable<Color = COLOUR> + 'a,
    F: DrawTransition<'a, COLOUR, T, T, DT>,
    DT: DrawTarget<Color = COLOUR>,
{
    /// Create a new transition from an image to itself.
    pub fn new_self(
        target: &'a mut DT,
        image: &'a T,
        transition: F,
        steps: u32,
        duration: Duration,
    ) -> Self {
        Self::new(target, image, image, transition, steps, duration)
    }
}
