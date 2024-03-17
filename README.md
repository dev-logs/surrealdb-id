# <a href="url"><img src="https://github.com/dev-logs/surreal-derive/assets/27767477/a10ad106-83af-48a2-894f-a599613e0d79" width="48"></a>  Surrealdb-id

# Description:
Using [SurrealDB](https://surrealdb.com/) is great, but using it with Rust can be tough,  I faced challenges creating a struct for representing
[RecordId](https://surrealdb.com/docs/surrealdb/surrealql/datamodel/ids) and [ Relate statement](https://surrealdb.com/docs/surrealdb/surrealql/statements/relate)
Take a look at my solution here, I want to share with you, it might help you too.
### Reducing the effort on writing SurrealQL: 
Before diving in, allow me to introduce my solution for simplify the process of writing SurrealQL.
Additionally, for leveraging its full potential, I highly recommend exploring another library known as surreal-derive-plus.

You can find it here:
https://crates.io/crates/surreal-derive-plus

Here is an example when they come together:
```rust
let user: RecordId = RecordId::from(("user", "Devlog"));
let blogPost: RecordId = RecordId::from(("blogPost", "How to use surrealdb"));
let discussion = Discussion { content: "Hello I really want to know more".to_string(), created_at: Default::default() };
let relation = discussion.relate(user, blogPost)

assert_eq!(
    surreal_quote!("#relate(&relation)"),
    "RELATE user:Devlog -> discuss -> blogPost:⟨How to use surrealdb⟩ SET content = 'Hello I really want to know more', created_at = '1970-01-01T00:00:00Z'"
);
```

# Link:
A Link can be both `record` or `id`.
### Create a Link
#### Simplest way
```rust
let user_link: Link<User> = Link::<User>::from(("user", "devlog"));
```
#### From any type that impl `From<T> for RecordId` type  
```rust
struct UserId {
    display_name: String
}

impl From<UserId> for RecordId {
    fn from(value: UserId) -> Self {
        RecordId::from(("user", value.display_name.as_str()))
    }
}

let user_link: Link<User> = Link::<User>::from(UserId {display_name: "Devlog"});
```
#### Bonus: Create link with new keyword
This approach offers greater type safety compared to using From as mentioned earlier, as it restricts users to specific parameters when creating a link.
```rust
impl NewLink<User, String> for Link<User> {
    fn new(params: String) -> Link<User> {
        Link::<User>::from(UserId { display_name: params })
    }
}

// Or with multiple params
//impl NewLink<User, (String, DateTime)> for Link<User> {
//  TOOD: Impl here
//}

let user_link = Link::<User>::new(("Devlog"));
```
### Link usage
```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
struct User {
    name: String
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
        me: Link::<User>::new(UserId { display_name: "Devlog".to_string() }), // `::new` will have more constrained to make sure only UserId can be passed
        my_friend: Link::<User>::from(UserId { display_name: "TienDang".to_string() }) // while `::from` will work with any E where RecordId: From<E>
    };

    let friend: Option<Friend> = db.query(
        "SELECT * FROM (CREATE friend:Devlog set me=user:Devlog, my_friend=user:TienDang) FETCH me"
    ).await?.take(0)?;

    let friend = friend.unwrap();
    // Right here, the my_friend is only id type
    // and me is object type
    assert_eq!(friend.my_friend.id().id.to_string(), "TienDang".to_owned());
    assert_eq!(friend.me.name, "Devlog".to_owned());
    Ok(())
}
```
## Relation:
### Create Relation
#### Use IdRelation
It will use `RecordId` as the default type for `in` and `out`
```rust
let relation: IdRelation<Discussion> = db.query("RELATE user:Devlog -> discuss -> blog:⟨How to use surrealdb⟩").await?.take(0)?;
```
#### use LinkRelation
The relation will wrap `in` and `out` in a `Link`
```rust
let relation: LinkRelation<User, Discussion, Blog> = db.query("SELECT * FROM RELATE user:Devlog -> discuss -> blog:⟨How to use surrealdb⟩ FETCH in, out").await?.take(0)?;
relation.r#in.id().id // => Devlog
relation.r#in.id().tb // => user
relation.r#out.id().id // ⟨How to use surrealdb⟩
relation.r#out.id().tb // => blog
```
## Relation usage
For example, you are in context of wring blog post, you want to
create feature discussion for your blog
```rust
use chrono::{DateTime, Utc};
use serde_derive::{Deserialize, Serialize};
use surrealdb::engine::local::Mem;
use surrealdb::opt::{RecordId, Resource};
use surrealdb::sql::Id;
use surrealdb::Surreal;
use crate::link::Link;
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
```

