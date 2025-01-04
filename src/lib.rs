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
    use crate::link::{Link, NewLink};
    use crate::relation::{IdRelation, LinkRelation};
    use chrono::{DateTime, Utc};
    use serde_derive::{Deserialize, Serialize};
    use surrealdb::engine::local::Mem;
    use surrealdb::sql::{Id, Thing};
    use surrealdb::{RecordId, Response, Surreal, Value};

    // Entity 1
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct User {
        name: String,
    }

    // Entity 2
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct BlogPost {
        title: String,
    }

    // Relation
    #[derive(Debug, Clone, Serialize, Deserialize)]
    struct Discussion {
        content: String,
        created_at: DateTime<Utc>,
    }

    impl Into<Thing> for User {
        fn into(self) -> Thing {
            ("user", self.name.as_str()).into()
        }
    }

    impl Into<Thing> for Discussion {
        fn into(self) -> Thing {
            (
                "discussion",
                self.created_at.timestamp_millis().to_string().as_str(),
            )
                .into()
        }
    }

    impl Into<Thing> for BlogPost {
        fn into(self) -> Thing {
            ("blog", self.title.as_str()).into()
        }
    }

    #[tokio::test]
    pub async fn should_convert_to_relation() -> surrealdb::Result<()> {
        let db = Surreal::new::<Mem>(()).await?;
        db.use_ns("test").use_db("test").await?;

        let user = Link::Record(User {
            name: "Devlog".to_string(),
        });
        let blogPost = Link::Record(BlogPost {
            title: "How to use surrealdb".to_string(),
        });

        println!("Tiendang-debug");
        let relation: Response = db.query(
            "SELECT * FROM (RELATE user:Devlog->discuss->blog:⟨How to use surrealdb⟩ SET content='Hello I really want to know more', created_at='2020-01-01T00:00:00Z')"
        ).await?;

        //let relation = relation.unwrap();
        let json = serde_json::to_string(&relation);

        println!("Tiendang-debug {:?}", json);

        let relation: surrealdb::Value = db.query(
            "SELECT * FROM (RELATE user:Devlog->discuss->blog:⟨How to use surrealdb⟩ SET content='Hello I really want to know more', created_at='2020-01-01T00:00:00Z') FETCH in, out"
        ).await?.take(0)?;

        let relation: LinkRelation<User, Discussion, BlogPost> = relation.into()?;
        assert_eq!(
            relation.r#in.as_ref().unwrap().name.to_string(),
            "Devlog".to_owned()
        );
        assert_eq!(
            relation.out.as_ref().unwrap().title.to_string(),
            "How to use surrealdb".to_owned()
        );
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
        assert_ne!(
            relation.unwrap().r#in.as_ref().unwrap().id.to_string(),
            "user:Devlog"
        );
        Ok(())
    }

    #[derive(Clone, Debug, Serialize, Deserialize)]
    pub struct Friend {
        me: Link<User>,
        my_friend: Link<User>,
    }

    // Every record must be impl Into<RecordId>
    impl Into<Thing> for Friend {
        fn into(self) -> Thing {
            Thing::from(("friend", self.me.name.as_str()))
        }
    }

    // If the id needed to be type safe, create another struct
    pub struct UserId {
        pub display_name: String,
    }

    // Implement From<UserId> to RecordId
    // so that we can use it with Link::<User>::from(UserId)
    impl From<UserId> for Thing {
        fn from(value: UserId) -> Self {
            Thing::from(("user", value.display_name.as_str()))
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
        let user = User {
            name: "Devlog".to_string(),
        };
        let th: Thing = user.clone().into();
        let friend = Friend {
            me: Link::<User>::new(UserId {
                display_name: "Devlog".to_string(),
            }), // new will has constraint to make sure only UserId can be passed
            my_friend: Link::<User>::from(UserId {
                display_name: "TienDang".to_string(),
            }), // from will work with any E where RecordId: From<E>
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
