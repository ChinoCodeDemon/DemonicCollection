use std::fs::File;
use std::io::{self, BufRead, Split};
use std::path::{Path, self};
use rocket_dyn_templates::{Template, context};
use rocket::http::{Status, ContentType};

#[rocket::get("/list")]
fn repo_list()->(Status, (ContentType, &'static str)){
    (Status::Accepted, (ContentType::JSON, Box::leak(jsonlist_from_file("resources/list.csv").into_boxed_str())))
}

#[rocket::get("/")]
fn index() -> Template {
    Template::render("index", context! {
        repos: &jsonlist_from_file("resources/list.csv")
    })
}

#[rocket::launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", rocket::routes![index, repo_list])
        .attach(Template::fairing())
}

fn jsonlist_from_file(filename: &str) -> String{
    let mut jlist = "".to_owned();
    if let Ok(lines) = read_lines(filename){
        let mut first = true;
        let mut header = Vec::new();
        for line in lines {
            if let Ok(data) = line{
                let row = Box::leak(data.into_boxed_str()).split(",").to_owned();
                if first{
                    header = row.collect();
                    first = false;
                    continue;
                }
                let mut jattr = "".to_owned();
                for (&key, value) in header.iter().zip(row){
                    jattr.push_str(&format!("\"{}\": \"{}\",\n", key, value));
                }
                jattr = format!("{{\n{}\n}},\n", &jattr[0..jattr.len() - 2]);
                jlist.push_str(&jattr)
            }
        }
    }
    jlist = format!("[\n{}\n]", &jlist[0..jlist.len()-2]);
    jlist
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where P: AsRef<Path>, {
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}