use simple_excel_writer::sheet::{CellValue, ToCellValue};

/// 航向
#[derive(Debug)]
pub enum Course {
    Same,
    Reverse,
    TNice,
    TFuck,
}

impl Course {
    pub fn from(i: i64) -> Course {
        match i {
            1 => Course::Same,
            2 => Course::Reverse,
            3 => Course::TNice,
            4 => Course::TFuck,
            _ => panic!("航向数字 {} 未知", i),
        }
    }
}

impl ToCellValue for &Course {
    fn to_cell_value(&self) -> CellValue {
        let s = match self {
            Course::Same => "同航",
            Course::Reverse => "反航",
            Course::TNice => "T优",
            Course::TFuck => "T劣",
        };
        CellValue::String(s.to_owned())
    }
}

/// 阵型
#[derive(Debug)]
pub enum Formation {
    DanZong,
    FuZong,
    LunXing,
    TXing,
    DanHeng,
}

impl Formation {
    pub fn from(i: i64) -> Formation {
        match i {
            1 => Formation::DanZong,
            2 => Formation::FuZong,
            3 => Formation::LunXing,
            4 => Formation::TXing,
            5 => Formation::DanHeng,
            _ => panic!("阵型数字 {} 未知", i),
        }
    }
}

impl ToCellValue for &Formation {
    fn to_cell_value(&self) -> CellValue {
        let s = match self {
            Formation::DanZong => "单纵",
            Formation::FuZong => "复纵",
            Formation::LunXing => "轮形",
            Formation::TXing => "梯形",
            Formation::DanHeng => "单横",
        };
        CellValue::String(s.to_owned())
    }
}

/// 制空状态
#[derive(Debug)]
pub enum AirType {
    Dominate,
    Superiority,
    Even,
    Inferior,
    Lost,
}

impl AirType {
    pub fn from(i: i64) -> AirType {
        use AirType::*;
        match i {
            1 => Dominate,
            2 => Superiority,
            3 => Even,
            4 => Inferior,
            5 => Lost,
            _ => panic!("未知制空状态：{}", i),
        }
    }
}

impl ToCellValue for &AirType {
    fn to_cell_value(&self) -> CellValue {
        use AirType::*;
        let s = match self {
            Dominate => "空确",
            Superiority => "空优",
            Even => "空均",
            Inferior => "空劣",
            Lost => "空丧",
        };
        CellValue::String(s.to_owned())
    }
}
