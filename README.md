# <a href="url"><img src="https://github.com/dev-logs/surreal-derive/assets/27767477/a10ad106-83af-48a2-894f-a599613e0d79" width="48"></a>  Surrealdb-id

# Description:
The surrealdb is greate, but working with it on Rust, especially create a struct that represent the id and relation feature of surrealdb is hard,
So check out on my solution here, to see if it suitable for you or not

# Usage:
#### Link:
A Link can be either record or id or both at the same time.

```rust
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct User {
    name: String
}

impl Into<RecordId> for User {
    fn into(self) -> RecordId {
        ("user", self.name.as_str()).into()
    }
}

// As an id for table user
let link_as_id: Link<User> = Link::from(("user", "Devlogs"));
// or: let link_as_id: Link<User> = Link::Id(("user", "Devlogs").into());

// As an record data for table user
let user: User = User {name: "Devlogs"};
let link_as_record: Link<User> = Link::Record(user);

// Extract the id from link
let user_id: RecordId = link_as_record.id();

// link between records
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Discussion {
    user: Link<User>
}

// impl Into<RecordId> for Discussion { impl here... }

// Serialize and deserialize
// 1. With fetch (include)
let discussion: Vec<Discussion> = db.query("SELECT * FROM discussion FETCH user").await?.take(0)?;
discussion.user.name // auto deref to the user data
discussion.user.id() // User:Devlog
// 2. Without fetch (include)
let discussion: Vec<Discussion> = db.query("SELECT * FROM discussion").await?.take(0)?;
discussion.user.name // will not be available because you are not FETCH the user table
discussion.user.id() // => user:Devlog

serde_json::to_string(discussion) // => {user: {....}};
```
#### Relation:
The relation feature in surrealdb is the most interesting feature, but the  way it response is too much flexible that really hard for 
us to control using Rust. Let take a look on my solution:
```rust
// For example, you are in context of wring blog post, you want to
// create feature discussion for your blog

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
        ("discuss", Id::Number(self.created_at.timestamp_millis())).into()
    }
}

impl Into<RecordId> for BlogPost {
    fn into(self) -> RecordId {
        ("blogPost", self.title.as_str()).into()
    }
}

// Retrieve the result from surrealdb
let relation: IdRelation<Discussion> = db.query("RELATE blogPost:⟨How to use surrealdb⟩ -> discuss -> user:Devlog SET content = 'Content', created_at = '1970-01-01T00:00:00Z'").await?.take(0)?.unwrap();
relation.r#in // access in
relation.out // access out
relation.relation // access relation detail

// Use with Link to retrieve the fetch (include) result for in and out
let relation: LinkRelation<User, Discussion, Blog> = db.query("SELECT * FROM RELATE blogPost:⟨How to use surrealdb⟩ -> discuss -> user:Devlog SET content = 'Content', created_at = '1970-01-01T00:00:00Z' FETCH in, out").await?.take(0)?.unwrap();
relation.r#in.name // => devlog 
relation.out.title // => How to use surrealdb
relation.relation.content // => I want to learn more
```

#### Use it with surreal-derive-plus:
Link here:
https://crates.io/crates/surreal-derive-plus

When it come together, it can prevent you from  
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

