use crate::model::{War, WriteType};
use crate::utils::format_sheet_name;
use simple_excel_writer::{Sheet, Workbook};
use std::collections::HashMap;

pub struct Writer {
    wb: Workbook,
    normal_sheets: HashMap<String, Sheet>,
    air_sheets: HashMap<String, Sheet>,
    hp_sheets: HashMap<String, Sheet>,
}

impl Writer {
    /// Create a writer with new workbook and create sheets, write sheet header
    pub fn new() -> Self {
        let mut wb = Workbook::create("test.jpg.txt.avi.xlsx");

        macro_rules! make_sheets {
            ($($key:expr => $name:expr),* ) => {{
                let mut map = HashMap::new();
                $({
                    for &side in &[1, 2] {
                        let sheet_name = format_sheet_name($name, side);
                        let sheet = wb.create_sheet(&sheet_name);
                        let sheet_key = format_sheet_name($key, side);
                        map.insert(sheet_key, sheet);
                    }
                })*
                map
            }};
        }

        let air_sheets = make_sheets! {
            "open_air" => "开幕空袭"
        };

        let normal_sheets = make_sheets! {
            "open_missile" => "开幕导弹",
            "open_torpedo" => "开幕雷击",
            "open_antisub" => "开幕反潜",
            "normal" => "炮击",
            "normal2" => "次轮炮击",
            "close_torpedo" => "闭幕雷",
            "close_missile" => "闭幕导弹"
        };

        let hp_sheets = make_sheets! {
            "hp" => "hp"
        };

        Writer {
            wb,
            normal_sheets,
            air_sheets,
            hp_sheets,
        }
    }

    /// Write a war to all sheets
    pub fn write<'a>(&mut self, wars: Vec<War>) {
        // 正常部分
        for (key, sheet) in self.normal_sheets.iter_mut() {
            self.wb
                .write_sheet(sheet, |sheet_writer| {
                    sheet_writer.append_row(War::header(WriteType::NormalBattle))?;
                    for war in wars.iter() {
                        sheet_writer.append_row(war.row(key, WriteType::NormalBattle))?;
                    }
                    Ok(())
                })
                .expect(&format!("写入数据到表 {} 失败", key));
        }
        // 空战部分
        for (key, sheet) in self.air_sheets.iter_mut() {
            self.wb
                .write_sheet(sheet, |sheet_writer| {
                    sheet_writer.append_row(War::header(WriteType::AirBattle))?;
                    for war in wars.iter() {
                        sheet_writer.append_row(war.row(key, WriteType::AirBattle))?;
                    }
                    Ok(())
                })
                .expect(&format!("写入数据到表 {} 失败", key));
        }
        // 血量部分
        for (key, sheet) in self.hp_sheets.iter_mut() {
            self.wb
                .write_sheet(sheet, |sheet_writer| {
                    sheet_writer.append_row(War::header(WriteType::HpInfo))?;
                    for war in wars.iter() {
                        sheet_writer.append_row(war.row(key, WriteType::HpInfo))?;
                    }
                    Ok(())
                })
                .expect(&format!("写入血量表 {} 失败", key));
        }
    }
}

impl std::ops::Drop for Writer {
    fn drop(&mut self) {
        self.wb.close().expect("保存 excel 文件失败");
    }
}
