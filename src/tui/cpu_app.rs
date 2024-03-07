use crate::helper::{
    cpu_type::{CPUInfo, CPUTemp},
};

pub struct App<'a> {
    pub title: &'a str,
    pub should_quit: bool,
    pub tabs: TabsState<'a>,
    pub reload: bool,
    // pub cpu_data: CPUData,
}

impl<'a> App<'a> {
    pub fn new(title: &'a str) -> App<'a> {
        App {
            title,
            should_quit: false,
            tabs: TabsState::new(Self::define_tabs()),
            reload: false,
            // cpu_data: CPUData::new(),
        }
    }

    fn define_tabs() -> Vec<&'a str> {
        let mut tabs: Vec<&str> = Default::default();

        // match RBInfo::read_sub_board().unwrap_or(0) {
        //     0 => tabs = vec!["RB"],
        //     1 => tabs = vec!["RB", "LTB"],
        //     2 => tabs = vec!["RB", "PB", "Preamp"],
        //     _ => (),
        // };

        tabs
    }

    pub fn reload_data(&mut self) {
        if self.reload {
            // self.cpu_data = CPUData::new();
        }
    }
}

pub struct TabsState<'a> {
    pub titles: Vec<&'a str>,
    pub index: usize,
}

impl<'a> TabsState<'a> {
    pub fn new(titles: Vec<&'a str>) -> TabsState {
        TabsState {
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

// pub struct CPUData {
//     pub info: CPUInfo,
// }

// impl CPUData {
//     pub fn new() -> CPUData {
//         CPUData {
//             info: CPUInfoDebug::new(),
//         }
//     }
// }