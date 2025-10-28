use crate::helper::{
    rb_type::{RBInfo, RBTemp, RBVcp, RBPh, RBMag},
    ltb_type::{LTBTemp, LTBThreshold},
    pb_type::{PBTemp, PBVcp},
    pa_type::{PATemp, PAReadBias},
};

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub reload: bool,
    pub rb_data: RBData,
    pub ltb_data: LTBData,
    pub pb_data: PBData,
    pub pa_data: PAData,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(Self::define_tabs()),
            reload: false,
            rb_data: RBData::new(),
            ltb_data: LTBData::new(),
            pb_data: PBData::new(),
            pa_data: PAData::new(),
        }
    }

    fn define_tabs() -> Vec<&'a str> {
        let mut tabs: Vec<&str> = Default::default();

        match RBInfo::read_sub_board().unwrap_or(0) {
            0 => tabs = vec!["RB"],
            1 => tabs = vec!["RB", "LTB"],
            2 => tabs = vec!["RB", "PB", "PA"],
            _ => (),
        };

        tabs
    }

    pub fn reload_data(&mut self) {
        if self.reload {
            self.rb_data = RBData::new();
            self.ltb_data = LTBData::new();
            self.pb_data = PBData::new();
            self.pa_data = PAData::new();
        }
    }
}

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> Self {
        Self {
            titles,
            index: 0,
        }
    }

    pub fn next(&mut self) {
        self.index = (self.index + 1) % self.titles.len();
    }

    pub fn previous(&mut self) {
        if self.index > 0 {
            self.index -= 1;
        } else {
            self.index = self.titles.len() - 1;
        }
    }
}

pub struct RBData {
    pub info: RBInfo,
    pub temp: RBTemp,
    pub vcp: RBVcp,
    pub ph: RBPh,
    pub mag: RBMag,
}

impl RBData {
    pub fn new() -> RBData {
        RBData {
            info: RBInfo::new(),
            temp: RBTemp::new(),
            vcp: RBVcp::new(),
            ph: RBPh::new(),
            mag: RBMag::new(),
        }
    }
}

pub struct LTBData {
    pub temp: LTBTemp,
    pub threshold: LTBThreshold,
}

impl LTBData {
    pub fn new() -> LTBData {
        LTBData {
            temp: LTBTemp::new(),
            threshold: LTBThreshold::new(),
        }
    }
}

pub struct PBData {
    pub temp: PBTemp,
    pub vcp: PBVcp,
}

impl PBData {
    pub fn new() -> PBData {
        PBData {
            temp: PBTemp::new(),
            vcp: PBVcp::new(),
        }
    }
}

pub struct PAData {
    pub temp: PATemp,
    pub bias: PAReadBias,
}

impl PAData {
    pub fn new() -> PAData {
        PAData {
            temp: PATemp::new(),
            bias: PAReadBias::new(),
        }
    }
}