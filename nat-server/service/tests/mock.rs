mod prepare;

use entity::post;
use prepare::prepare_mock_db;
use service::{Mutation, Query};

#[tokio::test]
async fn main() {
    let db = &prepare_mock_db();

    {
        let post = Query::find_post_by_id(db, 1).await.unwrap().unwrap();

        assert_eq!(post.id, 1);
    }

    {
        let post = Query::find_post_by_id(db, 5).await.unwrap().unwrap();

        assert_eq!(post.id, 5);
    }

    {
        let post = Mutation::create_post(
            db,
            post::Model {
                id: 0,
                name: "name D".to_owned(),
                pwd: "pwd D".to_owned(),
            },
        )
        .await
        .unwrap();

        assert_eq!(
            post,
            post::ActiveModel {
                id: sea_orm::ActiveValue::Unchanged(6),
                name: sea_orm::ActiveValue::Unchanged("name D".to_owned()),
                pwd: sea_orm::ActiveValue::Unchanged("pwd D".to_owned())
            }
        );
    }

    {
        let post = Mutation::update_post_by_id(
            db,
            1,
            post::Model {
                id: 1,
                name: "New name A".to_owned(),
                pwd: "New pwd A".to_owned(),
            },
        )
        .await
        .unwrap();

        assert_eq!(
            post,
            post::Model {
                id: 1,
                name: "New name A".to_owned(),
                pwd: "New pwd A".to_owned(),
            }
        );
    }

    {
        let result = Mutation::delete_post(db, 5).await.unwrap();

        assert_eq!(result.rows_affected, 1);
    }

    {
        let result = Mutation::delete_all_posts(db).await.unwrap();

        assert_eq!(result.rows_affected, 5);
    }
}
