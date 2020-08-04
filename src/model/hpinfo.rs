use serde_json::Value;

#[derive(Debug)]
pub struct HpInfo {
    pub max: Vec<i32>,
    pub start: Vec<i32>,
    pub end: Vec<i32>,
}

impl HpInfo {
    fn parse_ships(ships: &Vec<Value>, end_hp: &Vec<Value>) -> HpInfo {
        let (mut max, mut start) = (vec![], vec![]);
        ships.iter().enumerate().for_each(|(i, v)| {
            if v["indexInFleet"].as_i64().unwrap() != i as i64 {
                panic!("selfShips 乱序");
            }
            max.push(v["hpMax"].as_i64().unwrap() as i32);
            start.push(v["hp"].as_i64().unwrap() as i32);
        });
        let end = end_hp
            .iter()
            .take(max.len())
            .map(|v| v.as_i64().unwrap() as i32)
            .collect();

        HpInfo { max, start, end }
    }
    pub fn parse(vo: &Value) -> (HpInfo, HpInfo) {
        let hpinfo_self = Self::parse_ships(
            vo["selfShips"].as_array().unwrap(),
            vo["hpBeforeNightWarSelf"].as_array().unwrap(),
        );
        let hpinfo_enemy = Self::parse_ships(
            vo["enemyShips"].as_array().unwrap(),
            vo["hpBeforeNightWarEnemy"].as_array().unwrap(),
        );
        //
        (hpinfo_self, hpinfo_enemy)
    }
}
