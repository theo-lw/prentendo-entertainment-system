use super::APU;
use crate::state::cpu::Interrupt;
use crate::state::cpu::Memory;
use crate::state::NES;

impl<'a> APU<'a> for NES {
    fn get_apu_buffer(&'a self) -> &'a [f32] {
        &self.apu.buffer
    }

    fn clear_apu_buffer(&mut self) {
        self.apu.buffer = Vec::new();
    }

    fn apu_cycle(&mut self) {
        // Calculate output
        if self.apu.frame_counter.is_output_cycle() {
            let pulse_output =
                0.00752 * f32::from(self.apu.pulse1.get_volume() + self.apu.pulse2.get_volume());
            let tnd_output = 0.00851 * f32::from(self.apu.triangle.get_volume())
                + 0.00494 * f32::from(self.apu.noise.get_volume())
                + 0.00335 * f32::from(self.apu.dmc.get_volume());
            self.apu.buffer.push(pulse_output + tnd_output);
        }

        // tick an APU/CPU cycle
        self.apu.frame_counter.clock_cpu_cycle();
        if self.apu.frame_counter.is_even_cycle() {
            self.apu.pulse1.clock();
            self.apu.pulse2.clock();
            self.apu.noise.clock();
        }
        self.apu.triangle.clock();
        self.apu.dmc.clock();

        // check for DMA
        if self.apu.dmc.is_dma_active() {
            let val = self.get_mem(self.apu.dmc.cur_addr);
            self.apu.dmc.load_buffer(val);
        }

        // quarter frame
        if self.apu.frame_counter.is_quarter_frame() {
            self.apu.half_frame();
        }

        // half frame
        if self.apu.frame_counter.is_half_frame() {
            self.apu.half_frame();
        }

        // check for interrupt
        if self.apu.frame_counter.irq_triggered || self.apu.dmc.irq_triggered {
            self.trigger_irq();
            self.apu.frame_counter.irq_triggered = false;
            self.apu.dmc.irq_triggered = false;
        }
    }
}
