use std::default::Default;
use diesel;


pub struct FilterSettings {
    pub id: i64,
    pub user_id: i64,
    pub created_at: diesel::data_types::PgTimestamp,
    pub updated_at: diesel::data_types::PgTimestamp,

    pub limit: i64,
}

impl Default for FilterSettings {
    fn default() -> FilterSettings {
        FilterSettings {
            id: 0,
            user_id: 0,
            created_at: ::diesel::data_types::PgTimestamp(0),
            updated_at: ::diesel::data_types::PgTimestamp(0),

            limit: 20,
        }
    }
}
