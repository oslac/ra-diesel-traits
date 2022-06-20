#[macro_use]
extern crate diesel;

pub mod schema;
type Res<T> = Result<T, diesel::result::Error>;

fn main() {
    let conn = get_conn();
    let res = queries::find(1, &conn);
    println!("{:#?}", res);
}

pub mod queries {
    use crate::model::{NewPost, Post};
    use crate::schema::posts;
    use crate::{schema, Res};
    use diesel::query_dsl::methods::FilterDsl;
    // `eq` trait
    use diesel::ExpressionMethods;
    // `execute` trait.
    use diesel::RunQueryDsl;
    use diesel::SqliteConnection;

    pub fn find(id: i32, conn: &SqliteConnection) -> Res<Vec<Post>> {
        schema::posts::table
            //      *--------------* as `unknown` when hovering over `eq`
            .filter(posts::id.eq(id))
            .load::<Post>(conn)
    }

    pub fn insert(new_post: &NewPost, conn: &SqliteConnection) -> Res<usize> {
        let stmt = diesel::insert_into(posts::table).values(new_post);
        //        *----------------* as `unknown when hovering over `execute`
        let res = stmt.execute(conn)?;
        Ok(res)
    }
}

pub mod model {
    use crate::schema::posts;

    #[derive(Queryable, Debug)]
    pub struct Post {
        pub id: i32,
        pub title: String,
        pub body: String,
        pub published: bool,
    }

    #[derive(Insertable, Debug)]
    #[table_name = "posts"]
    pub struct NewPost {
        pub id: i32,
        pub title: String,
        pub body: String,
        pub published: bool,
    }
}

fn get_conn() -> diesel::SqliteConnection {
    use diesel::{Connection, SqliteConnection};
    SqliteConnection::establish("data.db").unwrap()
}
