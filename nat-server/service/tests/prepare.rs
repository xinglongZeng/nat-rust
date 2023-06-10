use ::entity::post;
use sea_orm::*;

#[cfg(feature = "mock")]
pub fn prepare_mock_db() -> DatabaseConnection {
    MockDatabase::new(DatabaseBackend::MySql)
        .append_query_results([
            [post::Model {
                id: 1,
                name: "name A".to_owned(),
                pwd: "pwd A".to_owned(),
            }],
            [post::Model {
                id: 5,
                name: "name C".to_owned(),
                pwd: "pwd C".to_owned(),
            }],
            [post::Model {
                id: 6,
                name: "name D".to_owned(),
                pwd: "pwd D".to_owned(),
            }],
            [post::Model {
                id: 1,
                name: "name A".to_owned(),
                pwd: "pwd A".to_owned(),
            }],
            [post::Model {
                id: 1,
                name: "New name A".to_owned(),
                pwd: "New pwd A".to_owned(),
            }],
            [post::Model {
                id: 5,
                name: "name C".to_owned(),
                pwd: "pwd C".to_owned(),
            }],
        ])
        .append_exec_results([
            MockExecResult {
                last_insert_id: 6,
                rows_affected: 1,
            },
            MockExecResult {
                last_insert_id: 6,
                rows_affected: 5,
            },
        ])
        .into_connection()
}
