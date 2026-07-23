use uuid::Uuid;



pub struct ComponentOrigin {
    pub origin_id: Uuid,
    pub component_id: Uuid,
    pub part_number: Option<String>,
    pub price: Option<i32>
}