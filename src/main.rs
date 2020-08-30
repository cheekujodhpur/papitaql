use mysql::*;
use mysql::prelude::*;

#[derive(Debug, PartialEq, Eq)]
struct Ages {
    name: String,
    age: i32,
}

impl FromRow for Ages {
    fn from_row_opt(r: Row) -> core::result::Result<Ages, FromRowError> {
        let age = Ages {
            name: r.get(0).unwrap(),
            age: r.get(1).unwrap()
        };
        Ok(age)
    }

    fn from_row(r: Row) -> Ages {
        Ages {
            name: r.get(0).unwrap(),
            age: r.get(1).unwrap()
        }
    }
}

fn main() {
    let url = "mysql://cheeku:{}@localhost/test";
    let pool = Pool::new(url).unwrap();
    let mut conn = pool.get_conn().unwrap();

    let result: Ages = conn.query_first("select * from ages").unwrap().unwrap();
    println!("{:?}", result);
        //.map(|mut row|{
        //    Ages {
        //        name: from_value(row.pop().unwrap()),
        //        age: from_value(row.pop().unwrap()),
        //    }
        //}).collect();
}
