use super::attack::{AirAttack, Attack, AttackTrait};
use serde_json::Value;
use simple_excel_writer::Row;

use std::collections::HashMap;

use crate::utils::format_sheet_name;

use super::utils::{AirType, Course, Formation};

#[derive(Debug)]
pub struct War {
    file_name: String,

    user_name: String,
    enemy_name: String,
    fleet_name: String,
    enemy_fleet_id: i32,
    enemy_fleet_name: String,
    /// 索敌
    spy_success: bool,
    /// 航向
    course: Course,
    /// 制空
    air_type: AirType,
    /// 阵型
    self_formation: Formation,
    enemy_formation: Formation,
    /// 一般攻击
    attacks: HashMap<String, Vec<Attack>>,
    /// 航空攻击
    air_attacks: HashMap<String, Vec<AirAttack>>,
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
        let air_type = AirType::from(get!("airControlType")?.as_i64()?);

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
            "open_antisub" => "openAntiSubAttack",
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
            air_type,
            self_formation,
            enemy_formation,

            // 一般攻击
            attacks,

            // 航空
            air_attacks,
        };

        Some(war)
    }

    /// 从 war report 中解析出一种攻击，返回 (side1, side2)
    fn parse_attacks<AttackT>(vo: &Value, key: &str) -> Option<(Vec<AttackT>, Vec<AttackT>)>
    where
        AttackT: AttackTrait,
    {
        let mut attacks = (vec![], vec![]);
        for atk_item in vo.get(key)?.as_array()?.into_iter() {
            let side = atk_item.get("attackSide")?.as_i64()?;
            let atk = AttackT::from(atk_item)?;

            match side {
                1 => attacks.0.push(atk),
                2 => attacks.1.push(atk),
                _ => panic!("未知攻击 side={}", side),
            }
        }

        Some(attacks)
    }

    /// outputs

    fn header_prefix_row() -> Row {
        const HEADER_PREFIX: &[&str] = &[
            "文件名",
            "用户名",
            "敌用户名",
            "舰队名",
            "敌舰队id",
            "敌舰队名",
            "索敌成功",
            "航向",
            "制空",
            "我方阵型",
            "敌方阵型",
        ];
        let mut row = Row::new();
        for &col in HEADER_PREFIX.iter() {
            row.add_cell(col);
        }
        row
    }

    pub fn header(air: bool) -> Row {
        let mut row = Self::header_prefix_row();
        match air {
            false => {
                const ATK_HEADER: [&str; 4] = ["from", "target", "伤害", "暴击"];
                for _ in 0..6 {
                    for &col in ATK_HEADER.iter() {
                        row.add_cell(col);
                    }
                }
            }
            true => {
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
            }
        }
        row
    }

    fn prefix_row(&self) -> Row {
        let mut row = Row::new();

        row.add_cell(self.file_name.as_str()); // 文件名
        row.add_cell(self.user_name.as_str()); // 用户名
        row.add_cell(self.enemy_name.as_str()); // 敌用户名
        row.add_cell(self.fleet_name.as_str()); // 舰队
        row.add_cell(self.enemy_fleet_id as f64); // 敌舰队id
        row.add_cell(self.enemy_fleet_name.as_str()); // 敌舰队名
        row.add_cell(self.spy_success); // 索敌
        row.add_cell(&self.course); // 航向
        row.add_cell(&self.air_type); // 制空
        row.add_cell(&self.self_formation); // 我方阵型
        row.add_cell(&self.enemy_formation); // 敌方阵型
        row
    }

    pub fn row(&self, key: &str, air: bool) -> Row {
        let mut row = self.prefix_row();
        match air {
            false => {
                for attack in self.attacks[key].iter() {
                    row.add_cell(attack.from_index as f64);
                    row.add_cell(attack.target_index as f64);
                    row.add_cell(attack.damage as f64);
                    row.add_cell(attack.is_critical);
                }
            }
            true => {
                for attack in self.air_attacks[key].iter() {
                    row.add_cell(attack.from_index as f64);
                    row.add_cell(attack.target_index as f64);
                    row.add_cell(attack.damage as f64);
                    row.add_cell(attack.is_critical);
                    //
                    row.add_cell(attack.plane_type as f64);
                    row.add_cell(attack.plane_type as f64);
                    row.add_cell(attack.drop_amount as f64);
                }
            }
        };
        row
    }
}
