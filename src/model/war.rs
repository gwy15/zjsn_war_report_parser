use super::attack::{AirAttack, Attack, AttackTrait};
use serde_json::Value;
use simple_excel_writer::sheet::{CellValue, ToCellValue};
use std::collections::HashMap;

use crate::utils::format_sheet_name;

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

#[derive(Debug)]
pub struct War {
    pub file_name: String,

    pub user_name: String,
    pub enemy_name: String,
    pub fleet_name: String,
    pub enemy_fleet_id: i32,
    pub enemy_fleet_name: String,
    /// 索敌
    pub spy_success: bool,
    /// 航向
    pub course: Course,
    /// 阵型
    pub self_formation: Formation,
    pub enemy_formation: Formation,
    /// 一般攻击
    pub attacks: HashMap<String, Vec<Attack>>,
    /// 航空攻击
    pub air_attacks: HashMap<String, Vec<AirAttack>>,
}

impl War {
    pub fn from(vo: &Value, file_name: String) -> Option<War> {
        let report = vo.get("warReport")?;

        macro_rules! get {
            ($key:expr) => {
                report.get($key).or_else(|| {
                    log::error!("文件 {} warReport 缺少 key {}", file_name, $key);
                    None
                })
            };
        }

        let user_name = get!("userName")?.as_str()?.to_owned();
        let enemy_name = get!("enemyName")?.as_str()?.to_owned();

        let spy_success = get!("isExploreSuccess")?
            .as_str()?
            .parse::<i32>()
            .expect("isExploreSuccess 不是数字")
            != 0;
        let course = Course::from(get!("warType")?.as_i64()?);

        let self_fleet = get!("selfFleet")?;
        let fleet_name = self_fleet.get("title")?.as_str()?.to_owned();
        let self_formation =
            Formation::from(self_fleet.get("formation")?.as_str()?.parse().unwrap());

        let enemy_fleet = get!("enemyFleet")?;
        let enemy_fleet_id = enemy_fleet.get("id")?;
        let enemy_fleet_id: i32 = match enemy_fleet_id {
            Value::Number(n) => n.as_i64()? as i32,
            Value::String(s) => s.parse().unwrap(),
            _ => panic!("傻逼幻萌，enemy fleet id 又瞎几把传"),
        };

        let enemy_fleet_name = enemy_fleet.get("title")?.as_str()?.to_owned();
        let enemy_formation =
            Formation::from(enemy_fleet.get("formation")?.as_str()?.parse().unwrap());

        // 一般攻击
        macro_rules! parse_attacks {
            ($($key:expr => $voKey:expr),*) => {{
                let mut attacks = HashMap::new();
                $({
                    let attacks_tuple = Self::parse_attacks(report, $voKey)?;
                    attacks.insert(format_sheet_name($key, 1), attacks_tuple.0);
                    attacks.insert(format_sheet_name($key, 2), attacks_tuple.1);
                })*
                attacks
            }};
        }
        let attacks = parse_attacks! {
            "open_missile" => "openMissileAttack",
            "open_torpedo" => "openTorpedoAttack",
            "normal" => "normalAttacks",
            "normal2" => "normalAttacks2",
            "close_torpedo" => "closeTorpedoAttack",
            "close_missile" => "closeMissileAttack"
        };

        // 航空攻击
        let air_attacks = parse_attacks! {
            "open_air" => "openAirAttack"
        };

        let war = War {
            file_name,
            // 战斗相关
            user_name,
            enemy_name,
            fleet_name,
            enemy_fleet_id,
            enemy_fleet_name,
            //
            spy_success,
            course,
            self_formation,
            enemy_formation,

            // 一般攻击
            attacks,

            // 航空
            air_attacks,
        };

        Some(war)
    }

    fn parse_attacks<T>(vo: &Value, key: &str) -> Option<(Vec<T>, Vec<T>)>
    where
        T: AttackTrait,
    {
        let mut attacks = (vec![], vec![]);
        for atk_item in vo.get(key)?.as_array()?.into_iter() {
            let side = atk_item.get("attackSide")?.as_i64()?;
            let atk = T::from(atk_item)?;

            match side {
                1 => attacks.0.push(atk),
                2 => attacks.1.push(atk),
                _ => panic!("未知攻击 side={}", side),
            }
        }

        Some(attacks)
    }
}
