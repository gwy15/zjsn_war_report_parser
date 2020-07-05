use super::attack::Attack;
use serde_json::Value;
use std::collections::HashMap;

/// 航向
#[derive(Debug)]
enum Course {
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

/// 阵型
#[derive(Debug)]
enum Formation {
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

#[derive(Debug)]
pub struct War {
    file_name: String,

    user_name: String,
    enemy_name: String,
    fleet_name: String,
    enemy_fleet_id: i32,
    enemy_fleet_name: String,

    /// 航向
    course: Course,
    /// 阵型
    self_formation: Formation,
    enemy_formation: Formation,
    /// 实际攻击
    attacks: HashMap<String, Vec<Attack>>,
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

        // .as_i64()? as i32;

        let enemy_fleet_name = enemy_fleet.get("title")?.as_str()?.to_owned();
        let enemy_formation =
            Formation::from(enemy_fleet.get("formation")?.as_str()?.parse().unwrap());

        let war = War {
            file_name,
            // 战斗相关
            user_name,
            enemy_name,
            fleet_name,
            enemy_fleet_id,
            enemy_fleet_name,
            //
            course,
            self_formation,
            enemy_formation,

            //
            attacks: HashMap::new(),
        };

        Some(war)
    }
}
