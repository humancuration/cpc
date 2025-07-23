use rusqlite_migration::{Migration, M};

pub(crate) fn migration() -> Migration<'static> {
    Migration::new(vec![
        M::up(include_str!("20250722195308_impact_tables/up.sql")),
        M::down(include_str!("20250722195308_impact_tables/down.sql")),
    ])
}