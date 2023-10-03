use crate::preamp_control::*;

pub fn initialize() {
    preamp_bias::PreampBiasSet::set_bias();
}
