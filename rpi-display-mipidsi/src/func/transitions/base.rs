//! The base transition function.
//!

use crate::{foreign_types::*, func, LcdDisplay};
use async_iterator::Iterator as AsyncIterator;
use std::{marker::PhantomData, sync::OnceLock, time::Duration};
use tokio::time::Instant;

/// A marker trait for transition functions.
pub trait TransitionFunction<'a, COLOUR, T1, T2, TO> {
    fn calculate_frame(
        &self,
        from: &'a T1,
        to: &'a T2,
        step: u32,
        steps: u32,
        w: u16,
        h: u16,
    ) -> RPiResult<'a, TO>;
}

impl<'a, COLOUR, T1, T2, TO, F> TransitionFunction<'a, COLOUR, T1, T2, TO> for F
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T1: ImageDrawable<Color = COLOUR> + 'a,
    T2: ImageDrawable<Color = COLOUR> + 'a,
    TO: ImageDrawable<Color = COLOUR>,
    F: Fn(&'a T1, &'a T2, u32, u32, u16, u16) -> RPiResult<'a, TO>,
{
    /// Calculate the frame for the given step.
    fn calculate_frame(
        &self,
        from: &'a T1,
        to: &'a T2,
        step: u32,
        steps: u32,
        w: u16,
        h: u16,
    ) -> RPiResult<'a, TO> {
        self(from, to, step, steps, w, h)
    }
}

/// Transition between two images.
pub struct Transition<'a, COLOUR, T1, T2, TO, F, const W: u16, const H: u16>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T1: ImageDrawable<Color = COLOUR> + 'a,
    T2: ImageDrawable<Color = COLOUR> + 'a,
    TO: ImageDrawable<Color = COLOUR> + 'a,
    F: TransitionFunction<'a, COLOUR, T1, T2, TO>,
{
    from: &'a T1,
    to: &'a T2,
    steps: u32,
    step: u32,

    start_time: OnceLock<Instant>,
    step_duration: Duration,

    transition: F,

    #[cfg(feature = "debug")]
    skipped_count: u32,

    _phantom: PhantomData<TO>,
}

impl<'a, COLOUR, T1, T2, TO, F, const W: u16, const H: u16>
    Transition<'a, COLOUR, T1, T2, TO, F, W, H>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T1: ImageDrawable<Color = COLOUR> + 'a,
    T2: ImageDrawable<Color = COLOUR> + 'a,
    TO: ImageDrawable<Color = COLOUR> + 'a,
    F: TransitionFunction<'a, COLOUR, T1, T2, TO>,
{
    /// Create a new transition.
    pub fn new(from: &'a T1, to: &'a T2, transition: F, steps: u32, duration: Duration) -> Self {
        let step_duration = duration / steps;

        Self {
            from,
            to,
            steps,
            step: 0,

            start_time: OnceLock::new(),
            step_duration,

            transition,

            #[cfg(feature = "debug")]
            skipped_count: 0,

            _phantom: PhantomData,
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

    /// Get the deadline for the next frame.
    fn next_deadline(&self) -> Instant {
        self.deadline(self.step + 1)
    }

    /// Draw the entire transition to the given draw target. On failure, return a
    /// [`RPiError`] in the [`RPiResult`].
    pub async fn draw<'e, D>(&mut self, target: &mut D) -> RPiResult<'a, ()>
    where
        'e: 'a,
        D: DrawTarget<Color = COLOUR>,
        D::Error: Into<RPiError<'e>>,
    {
        while let Some(result) = self.next().await {
            if let Ok(raw) = result {
                let image = func::image_conversions::image_from_raw(&raw, 0, 0);
                image.draw(target).into_rpi_result()?;

                tokio::time::sleep_until(self.next_deadline()).await;
            } else {
                result.and(Ok(()))?
            }
        }

        Ok(())
    }

    pub async fn draw_on_lcd<'e, DI, MODEL, RST>(
        &mut self,
        lcd: &mut LcdDisplay<DI, MODEL, RST, W, H>,
    ) -> RPiResult<'a, ()>
    where
        DI: WriteOnlyDataCommand,
        MODEL: DisplayModel<ColorFormat = COLOUR>,
        RST: OutputPinType,
    {
        self.draw(&mut lcd.display).await
    }
}

impl<'a, COLOUR, T, TO, F, const W: u16, const H: u16> Transition<'a, COLOUR, T, T, TO, F, W, H>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T: ImageDrawable<Color = COLOUR> + 'a,
    TO: ImageDrawable<Color = COLOUR> + 'a,
    F: TransitionFunction<'a, COLOUR, T, T, TO>,
{
    /// Create a new transition from an image to itself.
    pub fn new_self(target: &'a T, transition: F, steps: u32, duration: Duration) -> Self {
        Self::new(target, target, transition, steps, duration)
    }
}

impl<'a, COLOUR, T1, T2, TO, F, const W: u16, const H: u16> AsyncIterator
    for Transition<'a, COLOUR, T1, T2, TO, F, W, H>
where
    COLOUR: PixelColor + From<<COLOUR as PixelColor>::Raw>,
    T1: ImageDrawable<Color = COLOUR>,
    T2: ImageDrawable<Color = COLOUR>,
    TO: ImageDrawable<Color = COLOUR>,
    F: TransitionFunction<'a, COLOUR, T1, T2, TO>,
{
    type Item = RPiResult<'a, TO>;

    /// Get the next image in the transition.
    ///
    /// This does NOT block until the next frame time is reached; instead,
    /// duplicated frames will be returned if the iteration is faster than the
    /// desired frame time.
    ///
    /// The caller is responsible for waiting until the next frame time is
    /// reached; see [`deadline`] for more information.
    async fn next(&mut self) -> Option<Self::Item> {
        let step = self.calculate_current_step();

        // The second check is necessary to confirm that we have at least issued the
        // last step of the transition once.
        if step >= self.steps && self.step >= self.steps {
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

            return None;
        }

        let result = self
            .transition
            .calculate_frame(self.from, self.to, step, self.steps, W, H);

        #[cfg(feature = "debug")]
        if step > self.step + 1 {
            let skipped = step - self.step - 1;
            logger::debug(&format!("Frame overtime, skipping {} frames.", skipped));
            self.skipped_count += skipped;
        }

        self.step = step;
        Some(result)
    }
}
