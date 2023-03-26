use comfy_table::presets::UTF8_FULL;
use comfy_table::*;

use crate::preamp_control::*;


pub fn preamp_table() {
    println!("Preamp Environmental Status");
    preamp_temp_table();
    println!("");
    println!("Preamp SiPM Bias Status");
    preamp_bias_table();
}

pub fn preamp_temp_table() {
    let mut table = Table::new();

    let preamp_temp = preamp_temp::PreampTemp::new();

    table.load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Measurement").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
            Cell::new("Unit").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("Preamp 1 Temperature"),
            Cell::new(&format!("{:.3}", preamp_temp.preamp_tmp_1)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("Preamp 2 Temperature"),
            Cell::new(&format!("{:.3}", preamp_temp.preamp_tmp_2)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("Preamp 3 Temperature"),
            Cell::new(&format!("{:.3}", preamp_temp.preamp_tmp_3)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("Preamp 4 Temperature"),
            Cell::new(&format!("{:.3}", preamp_temp.preamp_tmp_4)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("Preamp 5 Temperature"),
            Cell::new(&format!("{:.3}", preamp_temp.preamp_tmp_5)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("Preamp 6 Temperature"),
            Cell::new(&format!("{:.3}", preamp_temp.preamp_tmp_6)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("Preamp 7 Temperature"),
            Cell::new(&format!("{:.3}", preamp_temp.preamp_tmp_7)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("Preamp 8 Temperature"),
            Cell::new(&format!("{:.3}", preamp_temp.preamp_tmp_8)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("Preamp 9 Temperature"),
            Cell::new(&format!("{:.3}", preamp_temp.preamp_tmp_9)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("Preamp 10 Temperature"),
            Cell::new(&format!("{:.3}", preamp_temp.preamp_tmp_10)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("Preamp 11 Temperature"),
            Cell::new(&format!("{:.3}", preamp_temp.preamp_tmp_11)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("Preamp 12 Temperature"),
            Cell::new(&format!("{:.3}", preamp_temp.preamp_tmp_12)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("Preamp 13 Temperature"),
            Cell::new(&format!("{:.3}", preamp_temp.preamp_tmp_13)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("Preamp 14 Temperature"),
            Cell::new(&format!("{:.3}", preamp_temp.preamp_tmp_14)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("Preamp 15 Temperature"),
            Cell::new(&format!("{:.3}", preamp_temp.preamp_tmp_15)),
            Cell::new("°C"),
        ])
        .add_row(vec![
            Cell::new("Preamp 16 Temperature"),
            Cell::new(&format!("{:.3}", preamp_temp.preamp_tmp_16)),
            Cell::new("°C"),
        ]);

    println!("{table}");
}

pub fn preamp_bias_table() {
    let mut table = Table::new();

    let preamp_bias = preamp_bias::PreampBiasRead::new();

    table.load_preset(UTF8_FULL)
        .set_header(vec![
            Cell::new("Measurement").add_attribute(Attribute::Bold),
            Cell::new("Value").add_attribute(Attribute::Bold),
            Cell::new("Unit").add_attribute(Attribute::Bold),
        ])
        .add_row(vec![
            Cell::new("Preamp 1 Bias Voltage"),
            Cell::new(&format!("{:.3}", preamp_bias.preamp_bias_read_1)),
            Cell::new("V"),
        ])
        .add_row(vec![
            Cell::new("Preamp 2 Bias Voltage"),
            Cell::new(&format!("{:.3}", preamp_bias.preamp_bias_read_2)),
            Cell::new("V"),
        ])
        .add_row(vec![
            Cell::new("Preamp 3 Bias Voltage"),
            Cell::new(&format!("{:.3}", preamp_bias.preamp_bias_read_3)),
            Cell::new("V"),
        ])
        .add_row(vec![
            Cell::new("Preamp 4 Bias Voltage"),
            Cell::new(&format!("{:.3}", preamp_bias.preamp_bias_read_4)),
            Cell::new("V"),
        ])
        .add_row(vec![
            Cell::new("Preamp 5 Bias Voltage"),
            Cell::new(&format!("{:.3}", preamp_bias.preamp_bias_read_5)),
            Cell::new("V"),
        ])
        .add_row(vec![
            Cell::new("Preamp 6 Bias Voltage"),
            Cell::new(&format!("{:.3}", preamp_bias.preamp_bias_read_6)),
            Cell::new("V"),
        ])
        .add_row(vec![
            Cell::new("Preamp 7 Bias Voltage"),
            Cell::new(&format!("{:.3}", preamp_bias.preamp_bias_read_7)),
            Cell::new("V"),
        ])
        .add_row(vec![
            Cell::new("Preamp 8 Bias Voltage"),
            Cell::new(&format!("{:.3}", preamp_bias.preamp_bias_read_8)),
            Cell::new("V"),
        ])
        .add_row(vec![
            Cell::new("Preamp 9 Bias Voltage"),
            Cell::new(&format!("{:.3}", preamp_bias.preamp_bias_read_9)),
            Cell::new("V"),
        ])
        .add_row(vec![
            Cell::new("Preamp 10 Bias Voltage"),
            Cell::new(&format!("{:.3}", preamp_bias.preamp_bias_read_10)),
            Cell::new("V"),
        ])
        .add_row(vec![
            Cell::new("Preamp 11 Bias Voltage"),
            Cell::new(&format!("{:.3}", preamp_bias.preamp_bias_read_11)),
            Cell::new("V"),
        ])
        .add_row(vec![
            Cell::new("Preamp 12 Bias Voltage"),
            Cell::new(&format!("{:.3}", preamp_bias.preamp_bias_read_12)),
            Cell::new("V"),
        ])
        .add_row(vec![
            Cell::new("Preamp 13 Bias Voltage"),
            Cell::new(&format!("{:.3}", preamp_bias.preamp_bias_read_13)),
            Cell::new("V"),
        ])
        .add_row(vec![
            Cell::new("Preamp 14 Bias Voltage"),
            Cell::new(&format!("{:.3}", preamp_bias.preamp_bias_read_14)),
            Cell::new("V"),
        ])
        .add_row(vec![
            Cell::new("Preamp 15 Bias Voltage"),
            Cell::new(&format!("{:.3}", preamp_bias.preamp_bias_read_15)),
            Cell::new("V"),
        ])
        .add_row(vec![
            Cell::new("Preamp 16 Bias Voltage"),
            Cell::new(&format!("{:.3}", preamp_bias.preamp_bias_read_16)),
            Cell::new("V"),
        ]);
        
    println!("{table}");
}