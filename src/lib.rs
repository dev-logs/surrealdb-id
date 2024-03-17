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
    use surrealdb::opt::{RecordId, Resource};
    use surrealdb::sql::Id;
    use surrealdb::Surreal;
    use crate::link::{Link, NewLink};
    use crate::link::*;
    use crate::relation::{IdRelation, LinkRelation};

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
            ("blog", self.title.as_str()).into()
        }
    }

    #[tokio::test]
    pub async fn should_convert_to_relation() -> surrealdb::Result<()> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("test").use_db("test").await?;

        let user = Link::Record(User { name: "Devlog".to_string() });
        let blogPost = Link::Record(BlogPost { title: "How to use surrealdb".to_string() });
        db.create(Resource::RecordId(user.id())).content(&user).await?;
        db.create(Resource::RecordId(blogPost.id())).content(&blogPost).await?;

        let relation: Option<LinkRelation<User, Discussion, BlogPost>> = db.query(
            "SELECT * FROM RELATE user:Devlog->discuss->blog:⟨How to use surrealdb⟩ SET content='Hello I really want to know more', created_at='2020-01-01T00:00:00Z'"
        ).await?.take(0)?;
        let relation = relation.unwrap();

        assert_eq!(relation.r#in.as_ref().unwrap().id().id.to_string(), "Devlog".to_owned());
        assert_eq!(relation.r#in.as_ref().unwrap().id().tb.as_str(), "user");
        assert_eq!(relation.out.as_ref().unwrap().id().id.to_string(), "⟨How to use surrealdb⟩".to_owned());
        assert_eq!(relation.out.as_ref().unwrap().id().tb.as_str(), "blog");

        let relation: Option<LinkRelation<User, Discussion, BlogPost>> = db.query(
            "SELECT * FROM (RELATE user:Devlog->discuss->blog:⟨How to use surrealdb⟩ SET content='Hello I really want to know more', created_at='2020-01-01T00:00:00Z') FETCH in, out"
        ).await?.take(0)?;

        let relation = relation.unwrap();
        assert_eq!(relation.r#in.as_ref().unwrap().name.to_string(), "Devlog".to_owned());
        assert_eq!(relation.out.as_ref().unwrap().title.to_string(), "How to use surrealdb".to_owned());
        Ok(())
    }

    #[tokio::test]
    pub async fn should_work_with_id_relation() -> surrealdb::Result<()> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("test").use_db("test").await?;

        let relation: Option<IdRelation<Discussion>> = db.query(
            "RELATE user:Devlog->discuss->blog:`How to use surrealdb` SET content='Hello I really want to know more', created_at='2020-01-01T00:00:00Z'"
        ).await?.take(0)?;

        assert!(&relation.is_some());
        assert_ne!(relation.unwrap().r#in.as_ref().unwrap().id.to_string(), "user:Devlog");
        Ok(())
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Friend {
        me: Link<User>,
        my_friend: Link<User>
    }

    // Every record must be impl Into<RecordId>
    impl Into<RecordId> for Friend {
        fn into(self) -> RecordId {
            RecordId::from(("friend", self.me.name.as_str()))
        }
    }

    // If the id needed to be type safe, create another struct
    pub struct UserId {
       pub display_name: String
    }

    // Implement From<UserId> to RecordId
    // so that we can use it with Link::<User>::from(UserId)
    impl From<UserId> for RecordId {
        fn from(value: UserId) -> Self {
            RecordId::from(("user", value.display_name.as_str()))
        }
    }

    // Or with more constraint
    // by forcing Link::User::new() can only be trigger with UserId
    impl NewLink<User, UserId> for Link<User> {
        fn new(params: UserId) -> Link<User> {
           Link::from(("user", params.display_name.as_str()))
        }
    }

    #[tokio::test]
    pub async fn should_work_with_link() -> surrealdb::Result<()> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("test").use_db("test").await?;
        let user = User { name: "Devlog".to_string() };
        db.create(Resource::RecordId(user.clone().into())).content(&user).await?;
        let friend = Friend {
            me: Link::<User>::new(UserId { display_name: "Devlog".to_string() }), // new will has constraint to make sure only UserId can be passed
            my_friend: Link::<User>::from(UserId { display_name: "TienDang".to_string() }) // from will work with any E where RecordId: From<E>
        };

        let friend : Option<Friend> = db.query(
           "SELECT * FROM (CREATE friend:Devlog set me=user:Devlog, my_friend=user:TienDang) FETCH me"
        ).await?.take(0)?;

        let friend = friend.unwrap();
        // Right here, the my_friend is only id type
        // and me is object type
        assert_eq!(friend.my_friend.id().id.to_string(), "TienDang".to_owned());
        assert_eq!(friend.me.name, "Devlog".to_owned());
        Ok(())
    }
}