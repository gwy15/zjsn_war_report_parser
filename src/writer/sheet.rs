use simple_excel_writer::sheet::SheetWriter;
use simple_excel_writer::*;
use std::io::Result;

use crate::model::War;

pub trait WarSheet {
    fn write_head(sheet_writer: &mut SheetWriter) -> Result<()>;

    fn write_war(war: &War, key: &str, sheet_writer: &mut SheetWriter) -> Result<()>;

    fn write(wars: &[War], key: &str, sheet_writer: &mut SheetWriter) -> Result<()> {
        Self::write_head(sheet_writer)?;
        for war in wars.iter() {
            Self::write_war(war, key, sheet_writer)?;
        }
        Ok(())
    }

    fn pre_header() -> &'static [&'static str] {
        return &[
            "文件名",
            "用户名",
            "敌用户名",
            "舰队名",
            "敌舰队id",
            "敌舰队名",
            "索敌成功",
            "航向",
            "我方阵型",
            "敌方阵型",
        ];
    }
}

pub struct NormalSheet {
    sheet: Sheet,
}

impl NormalSheet {
    pub fn from(sheet: Sheet) -> Self {
        Self { sheet }
    }

    pub fn inner(&mut self) -> &mut Sheet {
        &mut self.sheet
    }
}

impl WarSheet for NormalSheet {
    fn write_head(sheet_writer: &mut SheetWriter) -> Result<()> {
        let mut row = Row::new();
        for &col in Self::pre_header() {
            row.add_cell(col);
        }
        const ATK_HEADER: [&str; 4] = ["from", "target", "伤害", "暴击"];
        for _ in 0..6 {
            for &col in ATK_HEADER.iter() {
                row.add_cell(col);
            }
        }
        sheet_writer.append_row(row)
    }

    fn write_war(war: &War, key: &str, sheet_writer: &mut SheetWriter) -> Result<()> {
        let mut row = Row::new();

        row.add_cell(war.file_name.as_str()); // 文件名
        row.add_cell(war.user_name.as_str()); // 用户名
        row.add_cell(war.enemy_name.as_str()); // 敌用户名
        row.add_cell(war.fleet_name.as_str()); // 舰队
        row.add_cell(war.enemy_fleet_id as f64); // 敌舰队id
        row.add_cell(war.enemy_fleet_name.as_str()); // 敌舰队名
        row.add_cell(war.spy_success); // 索敌
        row.add_cell(&war.course); // 航向
        row.add_cell(&war.self_formation); // 我方阵型
        row.add_cell(&war.enemy_formation); // 敌方阵型

        for attack in war.attacks[key].iter() {
            row.add_cell(attack.from_index as f64);
            row.add_cell(attack.target_index as f64);
            row.add_cell(attack.damage as f64);
            row.add_cell(attack.is_critical);
        }
        sheet_writer.append_row(row)
    }
}

pub struct AirSheet {
    sheet: Sheet,
}

impl AirSheet {
    pub fn from(sheet: Sheet) -> Self {
        Self { sheet }
    }

    pub fn inner(&mut self) -> &mut Sheet {
        &mut self.sheet
    }
}

impl WarSheet for AirSheet {
    fn write_head(sheet_writer: &mut SheetWriter) -> Result<()> {
        let mut row = Row::new();
        for &col in Self::pre_header() {
            row.add_cell(col);
        }
        const ATK_HEADER: [&str; 7] = [
            "from",
            "target",
            "伤害",
            "暴击",
            //
            "飞机类型",
            "放飞",
            "击坠",
        ];
        for _ in 0..24 {
            for &col in ATK_HEADER.iter() {
                row.add_cell(col);
            }
        }
        sheet_writer.append_row(row)
    }

    fn write_war(war: &War, key: &str, sheet_writer: &mut SheetWriter) -> Result<()> {
        let mut row = Row::new();

        row.add_cell(war.file_name.as_str()); // 文件名
        row.add_cell(war.user_name.as_str()); // 用户名
        row.add_cell(war.enemy_name.as_str()); // 敌用户名
        row.add_cell(war.fleet_name.as_str()); // 舰队
        row.add_cell(war.enemy_fleet_id as f64); // 敌舰队id
        row.add_cell(war.enemy_fleet_name.as_str()); // 敌舰队名
        row.add_cell(war.spy_success); // 索敌
        row.add_cell(&war.course); // 航向
        row.add_cell(&war.self_formation); // 我方阵型
        row.add_cell(&war.enemy_formation); // 敌方阵型

        for attack in war.air_attacks[key].iter() {
            row.add_cell(attack.from_index as f64);
            row.add_cell(attack.target_index as f64);
            row.add_cell(attack.damage as f64);
            row.add_cell(attack.is_critical);
            //
            row.add_cell(attack.plane_type as f64);
            row.add_cell(attack.plane_type as f64);
            row.add_cell(attack.drop_amount as f64);
        }
        sheet_writer.append_row(row)
    }
}
