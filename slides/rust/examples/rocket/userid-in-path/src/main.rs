#[macro_use] extern crate rocket;

#[cfg(test)] mod tests;

use rocket::response::content;

use rocket::request::FromParam;
#[derive(Debug)]
struct User {
    uid: usize,
}

impl<'r> FromParam<'r> for User {
    type Error = &'r str;

    fn from_param(param: &'r str) -> Result<Self, Self::Error> {
        rocket::info!("from_param: {:?}", param);
        match param.parse::<usize>() {
            Ok(uid) => {
                if uid < 10000 {
                    Ok(Self { uid })
                } else {
                    Err("bad uid")
                }
            },
            Err(_) => Err("not a usize")
        }
    }
}

#[get("/")]
fn index() -> content::RawHtml<String> {
    let html = format!(
        r#"
    <a href="/user/42">id 42</a><br>
    <a href="/user/10001">id 10001</a> (not in database)<br>
    <a href="/user/text">text</a><br>
    "#
    );
    content::RawHtml(html)
}

#[get("/user/<user>")]
fn user(user: User) -> content::RawHtml<String> {
    rocket::info!("slug: {:?}", user);

    let html = format!("uid: {}", user.uid);

    content::RawHtml(html)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![index, user])
}
