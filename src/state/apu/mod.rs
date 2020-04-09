/// Represents the APU's internal state
pub struct APUState {
    pub sq1_vol: u8,
    pub sq1_sweep: u8,
    pub sq1_lo: u8,
    pub sq1_hi: u8,
    pub sq2_vol: u8,
    pub sq2_sweep: u8,
    pub sq2_lo: u8,
    pub sq2_hi: u8,
    pub tri_linear: u8,
    pub tri_lo: u8,
    pub tri_hi: u8,
    pub noise_vol: u8,
    pub noise_lo: u8,
    pub noise_hi: u8,
    pub dmc_freq: u8,
    pub dmc_raw: u8,
    pub dmc_start: u8,
    pub dmc_len: u8,
    pub snd_chn: u8,
}

impl APUState {
    #[cfg(test)]
    pub fn mock() -> Self {
        Self::new()
    }

    pub fn new() -> Self {
        APUState {
            sq1_vol: 0,
            sq1_sweep: 0,
            sq1_lo: 0,
            sq1_hi: 0,
            sq2_vol: 0,
            sq2_sweep: 0,
            sq2_lo: 0,
            sq2_hi: 0,
            tri_linear: 0,
            tri_lo: 0,
            tri_hi: 0,
            noise_vol: 0,
            noise_lo: 0,
            noise_hi: 0,
            dmc_freq: 0,
            dmc_raw: 0,
            dmc_start: 0,
            dmc_len: 0,
            snd_chn: 0,
        }
    }
}
