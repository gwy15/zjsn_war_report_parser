use excel::*;
use simple_excel_writer as excel;

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

fn header() -> Row {
    let mut row = Row::new();
    const PRE_HEADER: [&str; 8] = [
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

pub struct Writer {
    wb: Workbook,
    open_missile: (Sheet, Sheet),
    normal: (Sheet, Sheet),
    normal2: (Sheet, Sheet),
    close_torpedo: (Sheet, Sheet),
}

impl Writer {
    /// Create a writer with new workbook and create sheets, write sheet header
    pub fn new() -> Self {
        let mut wb = Workbook::create("test.jpg.txt.avi.xlsx");

        macro_rules! pair {
            ($key:tt) => {{
                let sheet_name = sheet_name!($key);
                let mut sheet_1 = wb.create_sheet(&format_sheet_name(sheet_name, 1));
                wb.write_sheet(&mut sheet_1, |sw| sw.append_row(header()))
                    .unwrap();

                let mut sheet_2 = wb.create_sheet(&format_sheet_name(sheet_name, 2));
                wb.write_sheet(&mut sheet_2, |sw| sw.append_row(header()))
                    .unwrap();
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
}

impl std::ops::Drop for Writer {
    fn drop(&mut self) {
        self.wb.close().expect("保存 excel 文件失败");
    }
}
