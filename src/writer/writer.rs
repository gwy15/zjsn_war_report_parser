use super::sheet::{AirSheet, NormalSheet, WarSheet};
use crate::model::War;
use crate::utils::format_sheet_name;
use simple_excel_writer::Workbook;
use std::collections::HashMap;

pub struct Writer {
    wb: Workbook,
    normal_sheets: HashMap<String, NormalSheet>,
    air_sheets: HashMap<String, AirSheet>,
}

impl Writer {
    /// Create a writer with new workbook and create sheets, write sheet header
    pub fn new() -> Self {
        let mut wb = Workbook::create("test.jpg.txt.avi.xlsx");

        macro_rules! make_sheets {
            ($type:ty, { $($key:expr => $name:expr),* }) => {{
                let mut map = HashMap::new();
                $({
                    for &side in &[1, 2] {
                        let sheet = wb.create_sheet(&format_sheet_name($name, side));
                        map.insert(format_sheet_name($key, side), <$type>::from(sheet));
                    }
                })*
                map
            }};
        }

        let air_sheets = make_sheets!(AirSheet, {
            "open_air" => "开幕空袭"
        });

        let normal_sheets = make_sheets! (NormalSheet, {
            "open_missile" => "开幕导弹",
            "open_torpedo" => "开幕雷击",
            "normal" => "炮击",
            "normal2" => "次轮炮击",
            "close_torpedo" => "闭幕雷",
            "close_missile" => "闭幕导弹"
        });

        Writer {
            wb,
            normal_sheets,
            air_sheets,
        }
    }

    /// Write a war to all sheets
    pub fn write<'a>(&mut self, wars: Vec<War>) {
        // 其他
        for (key, sheet) in self.normal_sheets.iter_mut() {
            self.wb
                .write_sheet(sheet.inner(), |sheet_writer| {
                    NormalSheet::write(&wars, key, sheet_writer)
                })
                .expect(&format!("写入数据到表 {} 失败", key));
        }
        // 空战部分
        for (key, sheet) in self.air_sheets.iter_mut() {
            self.wb
                .write_sheet(sheet.inner(), |sheet_writer| {
                    AirSheet::write(&wars, key, sheet_writer)
                })
                .expect(&format!("写入数据到表 {} 失败", key));
        }
    }
}

impl std::ops::Drop for Writer {
    fn drop(&mut self) {
        self.wb.close().expect("保存 excel 文件失败");
    }
}
