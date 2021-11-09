#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;

#[get("/<max>")]
fn password(max: u8) -> String {
    //use rand::prelude::*;
    let mut v = String::with_capacity(max as usize);
    for _ in 0..max {
        loop{
            let x = rand::random::<u8>() as char;
            if x == '\t'|| x == ' ' || x == '\n'{
                continue
            }
            v.push(x);
            break
        }
    }
    format!("{}",v)
}

fn main() {
    rocket::ignite().mount("/", routes![password]).launch();
}
