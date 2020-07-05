use excel::*;
use simple_excel_writer as excel;

use crate::model::{Attack, War};

macro_rules! sheet_name {
    (open_missile) => {
        "开幕导弹"
    };
    (normal) => {
        "炮击"
    };
    (normal2) => {
        "次轮炮击"
    };
    (close_torpedo) => {
        "闭幕雷"
    };
}

fn format_sheet_name(sheet_name: &str, side: i32) -> String {
    format!("{}_{}", sheet_name, side)
}

pub struct Writer {
    wb: Workbook,
    open_missile: (Sheet, Sheet),
    normal: (Sheet, Sheet),
    normal2: (Sheet, Sheet),
    close_torpedo: (Sheet, Sheet),
}

impl Writer {
    fn header() -> Row {
        let mut row = Row::new();
        const PRE_HEADER: [&str; 9] = [
            "文件名",
            "用户名",
            "敌用户名",
            "舰队名",
            "敌舰队id",
            "敌舰队名",
            "航向",
            "我方阵型",
            "敌方阵型",
        ];
        for &col in PRE_HEADER.iter() {
            row.add_cell(col);
        }
        const ATK_HEADER: [&str; 4] = ["from", "target", "damage", "critical"];
        for _ in 0..6 {
            for &col in ATK_HEADER.iter() {
                row.add_cell(col);
            }
        }

        row
    }

    /// Create a writer with new workbook and create sheets, write sheet header
    pub fn new() -> Self {
        let mut wb = Workbook::create("test.jpg.txt.avi.xlsx");

        macro_rules! pair {
            ($key:tt) => {{
                let sheet_name = sheet_name!($key);
                let sheet_1 = wb.create_sheet(&format_sheet_name(sheet_name, 1));
                let sheet_2 = wb.create_sheet(&format_sheet_name(sheet_name, 2));
                (sheet_1, sheet_2)
            }};
        }

        let open_missile = pair!(open_missile);
        let normal = pair!(normal);
        let normal2 = pair!(normal2);
        let close_torpedo = pair!(close_torpedo);

        Writer {
            wb,
            open_missile,
            normal,
            normal2,
            close_torpedo,
        }
    }

    fn row(war: &War, attacks: &Vec<Attack>) -> Row {
        let mut row = Row::new();

        row.add_cell(war.file_name.as_str()); // 文件名
        row.add_cell(war.user_name.as_str()); // 用户名
        row.add_cell(war.enemy_name.as_str()); // 敌用户名
        row.add_cell(war.fleet_name.as_str()); // 舰队
        row.add_cell(war.enemy_fleet_id as f64); // 敌舰队id
        row.add_cell(war.enemy_fleet_name.as_str()); // 敌舰队名
        row.add_cell(&war.course); // 航向
        row.add_cell(&war.self_formation); // 我方阵型
        row.add_cell(&war.enemy_formation); // 敌方阵型

        for attack in attacks {
            row.add_cell(attack.from_index as f64);
            row.add_cell(attack.target_index as f64);
            row.add_cell(attack.damage as f64);
            row.add_cell(attack.is_critical);
        }

        row
    }

    /// Write a war to all sheets
    pub fn write<'a>(&mut self, wars: Vec<War>) {
        macro_rules! write {
            ($sheet:expr, $atks:expr) => {
                self.wb
                    .write_sheet($sheet, |sw| {
                        sw.append_row(Self::header())?;
                        // write each war
                        for (war, atks) in wars.iter().zip($atks) {
                            // let atks = $atks;
                            let row = Self::row(war, atks);
                            sw.append_row(row)?;
                        }
                        Ok(())
                    })
                    .expect("Write failed.");
            };
        }

        // 开幕
        write!(
            &mut self.open_missile.0,
            wars.iter().map(|w| &w.atks_open_msl.0)
        );
        write!(
            &mut self.open_missile.1,
            wars.iter().map(|w| &w.atks_open_msl.1)
        );
        // 首轮炮击
        write!(&mut self.normal.0, wars.iter().map(|w| &w.atks_normal.0));
        write!(&mut self.normal.1, wars.iter().map(|w| &w.atks_normal.1));
        // 次轮炮击
        write!(&mut self.normal2.0, wars.iter().map(|w| &w.atks_normal2.0));
        write!(&mut self.normal2.1, wars.iter().map(|w| &w.atks_normal2.1));
        // 闭幕鱼雷
        write!(
            &mut self.close_torpedo.0,
            wars.iter().map(|w| &w.atks_close_tpd.0)
        );
        write!(
            &mut self.close_torpedo.1,
            wars.iter().map(|w| &w.atks_close_tpd.1)
        );
    }
}

impl std::ops::Drop for Writer {
    fn drop(&mut self) {
        self.wb.close().expect("保存 excel 文件失败");
    }
}
