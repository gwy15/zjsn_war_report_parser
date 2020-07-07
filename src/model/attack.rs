use serde_json::Value;

fn parse_basic(vo: &Value) -> Option<(i32, i32, i32, bool)> {
    let from_index = vo.get("fromIndex")?.as_i64()? as i32;
    let target_index = vo.get("targetIndex")?.as_array()?[0].as_i64()? as i32;

    let damage = vo.get("damage")?.as_array()?[0].as_i64()? as i32;

    let damage_item: &Value = &vo.get("damages")?.as_array()?[0];
    let is_critical = damage_item.get("isCritical")?.as_i64()? != 0;

    Some((from_index, target_index, damage, is_critical))
}

pub trait AttackTrait: Sized {
    fn from(vo: &Value) -> Option<Self>;
}

#[derive(Debug)]
pub struct Attack {
    pub from_index: i32,
    pub target_index: i32,
    pub damage: i32,
    pub is_critical: bool,
}

impl AttackTrait for Attack {
    fn from(vo: &Value) -> Option<Self> {
        let (from_index, target_index, damage, is_critical) = parse_basic(vo)?;
        Some(Self {
            from_index,
            target_index,
            damage,
            is_critical,
        })
    }
}

#[derive(Debug)]
pub struct AirAttack {
    pub from_index: i32,
    pub target_index: i32,
    pub damage: i32,
    pub is_critical: bool,

    pub plane_type: i32,   // 攻击类型
    pub plane_amount: i32, // 放飞
    pub drop_amount: i32,  // 航空击坠
}

impl AttackTrait for AirAttack {
    fn from(vo: &Value) -> Option<Self> {
        let (from_index, target_index, damage, is_critical) = parse_basic(vo)?;

        let plane_type = vo.get("planeType")?.as_i64()? as i32;
        let plane_amount = vo.get("planeAmount")?.as_i64()? as i32;
        let drop_amount = vo.get("dropAmount")?.as_i64()? as i32;

        Some(Self {
            from_index,
            target_index,
            damage,
            is_critical,

            plane_type,
            plane_amount,
            drop_amount,
        })
    }
}
