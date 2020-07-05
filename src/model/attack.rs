use serde_json::Value;

#[derive(Debug)]
pub struct Attack {
    from_index: i32,
    target_index: i32,
    damage: i32,
    is_critical: bool,
}

impl Attack {
    pub fn from(vo: &Value) -> Option<Self> {
        let from_index = vo.get("fromIndex")?.as_i64()? as i32;
        let target_index = vo.get("targetIndex")?.as_array()?[0].as_i64()? as i32;

        let damage = vo.get("damage")?.as_array()?[0].as_i64()? as i32;

        let damage_item: &Value = &vo.get("damages")?.as_array()?[0];
        let is_critical = damage_item.get("isCritical")?.as_i64()? != 0;

        Some(Self {
            from_index,
            target_index,
            damage,
            is_critical,
        })
    }
}
