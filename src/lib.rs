use crate::link::Link;

pub mod link;
pub mod relation;

// Define many name for user to easy to use
pub type SurrealDbId<T> = Link<T>;
pub type SurrealId<T> = Link<T>;
pub type DbId<T> = Link<T>;
pub type Identifier<T> = Link<T>;

#[cfg(test)]
mod test {
    use chrono::{DateTime, Utc};
    use serde_derive::{Deserialize, Serialize};
    use surrealdb::engine::local::Mem;
    use surrealdb::key::root::us::Us;
    use surrealdb::opt::{IntoResource, RecordId, Resource};
    use surrealdb::sql::Id;
    use surrealdb::Surreal;
    use crate::link::Link;
    use crate::relation::Relation;

    // Entity 1
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct User {
        name: String
    }

    // Entity 2
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct BlogPost {
        title: String
    }

    // Relation
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Discussion {
        content: String,
        created_at: DateTime<Utc>
    }

    impl Into<RecordId> for User {
        fn into(self) -> RecordId {
            ("user", self.name.as_str()).into()
        }
    }

    impl Into<RecordId> for Discussion {
        fn into(self) -> RecordId {
            ("discussion", Id::Number(self.created_at.timestamp_millis())).into()
        }
    }

    impl Into<RecordId> for BlogPost {
        fn into(self) -> RecordId {
            ("blogPost", self.title.as_str()).into()
        }
    }

    #[tokio::test]
    pub async fn should_convert_to_link() -> surrealdb::Result<()> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("test").use_db("test").await?;

        let user = Link::Record(User { name: "Devlog".to_string() });
        let blogPost = Link::Record(BlogPost { title: "How to use surrealdb".to_string() });
        db.create(Resource::RecordId(user.id())).content(&user).await?;
        db.create(Resource::RecordId(blogPost.id())).content(&blogPost).await?;

        let relation: Option<Relation<Discussion, User, BlogPost>> = db.query(
            "RELATE user:Devlog->discuss->blog:`How to use surrealdb` SET content='Hello I really want to know more', created_at='2020-01-01T00:00:00Z'"
        ).await?.take(0)?;

        assert!(&relation.is_some());
        assert_ne!(relation.unwrap().r#in.id().id.to_string(), "user:Devlog");
        Ok(())
    }
}