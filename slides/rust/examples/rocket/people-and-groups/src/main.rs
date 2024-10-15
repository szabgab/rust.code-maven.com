#[macro_use]
extern crate rocket;

use std::vec;

use rocket::form::Form;
use rocket::State;
use rocket_dyn_templates::{context, Template};

use surrealdb::engine::remote::ws::Client;
use surrealdb::Surreal;

struct Config {
    database: String,
}
pub mod db;

mod mytera;

#[derive(FromForm)]
struct AddPerson<'r> {
    name: &'r str,
}

#[derive(FromForm)]
struct AddGroup<'r> {
    name: &'r str,
    uid: &'r str,
}

#[derive(FromForm)]
struct AddMember<'r> {
    gid: &'r str,
    uid: &'r str,
}

// ----------------------------------

#[get("/")]
async fn get_index(dbh: &State<Surreal<Client>>) -> Template {
    let people = db::get_people(dbh).await.unwrap();
    let groups = db::get_groups(dbh).await.unwrap();
    Template::render(
        "index",
        context! {
            title: "People and Groups",
            people: people,
            groups: groups,
        },
    )
}

#[get("/clear")]
async fn clear_db(dbh: &State<Surreal<Client>>, config: &State<Config>) -> Template {
    rocket::info!("Clearing database {}", config.database);
    db::clear(&dbh, config.database.clone()).await.unwrap();
    Template::render(
        "database_cleared",
        context! {
            title: "Database cleared",
        },
    )
}

#[get("/people")]
async fn get_people(dbh: &State<Surreal<Client>>) -> Template {
    let people = db::get_people(dbh).await.unwrap();
    rocket::info!("People: {:?}", people);

    Template::render(
        "people",
        context! {
            title: "People",
            people,
        },
    )
}

#[post("/add-person", data = "<input>")]
async fn post_add_person(dbh: &State<Surreal<Client>>, input: Form<AddPerson<'_>>) -> Template {
    let name = input.name.trim();
    rocket::info!("Add  person called '{name}'");
    let person = db::add_person(dbh, name).await.unwrap();

    Template::render(
        "person_added",
        context! {
            title: "Person added",
            person,
        },
    )
}

#[get("/person/<id>")]
async fn get_person(dbh: &State<Surreal<Client>>, id: String) -> Option<Template> {
    if let Some((person, owned_groups, memberships)) =
        db::get_person_with_groups(dbh, &id).await.unwrap()
    {
        return Some(Template::render(
            "person",
            context! {
                title: person.name.clone(),
                person,
                owned_groups,
                memberships,
            },
        ));
    }

    None
}

#[get("/groups")]
async fn get_groups(dbh: &State<Surreal<Client>>) -> Template {
    let groups = db::get_groups(dbh).await.unwrap();
    rocket::info!("Groups: {:?}", groups);

    Template::render(
        "groups",
        context! {
            title: "Groups",
            groups,
        },
    )
}

#[get("/add-group?<uid>")]
async fn get_add_group(dbh: &State<Surreal<Client>>, uid: String) -> Template {
    let person = db::get_person(dbh, &uid).await.unwrap().unwrap();

    Template::render(
        "add_group",
        context! {
            title: format!("Add Group owned by {}", person.name),
            uid: uid.to_string(),
        },
    )
}

#[post("/add-group", data = "<input>")]
async fn post_add_group(dbh: &State<Surreal<Client>>, input: Form<AddGroup<'_>>) -> Template {
    let name = input.name.trim();
    let uid = input.uid.trim();
    rocket::info!("Add  group called '{name}' to user '{uid}'");
    let group = db::add_group(dbh, name, uid).await.unwrap();

    let person = db::get_person(dbh, &uid).await.unwrap().unwrap();

    Template::render(
        "group_added",
        context! {
            title: format!("Group called {} owned by {} was added", name, person.name),
            uid: uid.to_string(),
            group,
        },
    )
}

#[get("/add-membership?<uid>")]
async fn get_add_membership(dbh: &State<Surreal<Client>>, uid: String) -> Template {
    let person = db::get_person(dbh, &uid).await.unwrap().unwrap();

    //let groups = db::get_groups(dbh).await.unwrap();
    let groups = db::get_groups_not(dbh, &uid).await.unwrap();
    //let memberships = db::get_memberships_of_person(dbh, &person.id.to_string()).await.unwrap();

    // remove the groups that the person already owns or is a member of
    let groups = groups
        .into_iter()
        .filter(|group| group.owner.id != person.id.id)
        .collect::<Vec<_>>();
    Template::render(
        "add_membership",
        context! {
            title: format!("Add {} to one of the groups as a member", person.name),
            uid: uid.to_string(),
            groups
        },
    )
}

#[post("/add-membership", data = "<input>")]
async fn post_add_membership(dbh: &State<Surreal<Client>>, input: Form<AddMember<'_>>) -> Template {
    let gid = input.gid.trim();
    let uid = input.uid.trim();
    rocket::info!("Add  person '{uid}' to group '{gid}'");
    let group = db::get_group(dbh, gid).await.unwrap().unwrap();
    let person = db::get_person(dbh, &uid).await.unwrap().unwrap();
    db::add_member(dbh, uid, gid).await.unwrap();

    Template::render(
        "added_to_group",
        context! {
            title: format!("'{}' was added to group '{}'", person.name, group.name),
            person,
            group,
        },
    )
}

#[get("/delete-membership?<id>")]
async fn delete_membership(dbh: &State<Surreal<Client>>, id: String) -> Template {
    db::delete_membership(dbh, &id).await.unwrap();

    Template::render(
        "membership_deleted",
        context! {
            title: "Membership deleted",
        },
    )
}

#[get("/memberships")]
async fn get_membership(dbh: &State<Surreal<Client>>) -> Template {
    let memberships = db::get_memberships(dbh).await.unwrap();

    Template::render(
        "memberships",
        context! {
            title: "Memberships",
            memberships
        },
    )
}

#[get("/group/<id>")]
async fn get_group(dbh: &State<Surreal<Client>>, id: String) -> Option<Template> {
    if let Some(group) = db::get_group_with_owner(dbh, &id).await.unwrap() {
        rocket::info!("Group: {}", group.owner.id);
        rocket::info!("Group: {:?}", group);
        return Some(Template::render(
            "group",
            context! {
                title: group.name.clone(),
                group,
            },
        ));
    }

    None
}

#[launch]
fn rocket() -> _ {
    start(String::from("people-and-groups"))
}

fn start(database: String) -> rocket::Rocket<rocket::Build> {
    rocket::build()
        .mount(
            "/",
            routes![
                clear_db,
                delete_membership,
                get_add_membership,
                get_index,
                get_people,
                get_person,
                get_groups,
                get_group,
                get_membership,
                get_add_group,
                post_add_group,
                post_add_membership,
                post_add_person,
            ],
        )
        .manage(Config {
            database: database.clone(),
        })
        .attach(Template::custom(|engines| {
            mytera::customize(&mut engines.tera);
        }))
        .attach(db::fairing(database))
}

#[cfg(test)]
mod tests {
    use super::start;

    use rocket::http::{ContentType, Status};
    use rocket::local::blocking::Client;
    //use scraper::{Html, Selector};

    #[test]
    fn test_main() {
        let client = Client::tracked(start(String::from("test-people-and-groups"))).unwrap();

        let response = client.get("/clear").dispatch();
        assert_eq!(response.status(), Status::Ok);

        // Make sure we can clear the database even if it does not exist
        let response = client.get("/clear").dispatch();
        assert_eq!(response.status(), Status::Ok);

        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert!(html.contains(r#"<title>People and Groups</title>"#));
        assert!(html.contains(r#"<div>Number of people: 0</div>"#));
        assert!(html.contains(r#"<div>Number of groups: 0</div>"#));

        let response = client
            .post("/add-person")
            .header(ContentType::Form)
            .body("name=John")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert!(html.contains(r#"Person added:"#));
        let document = scraper::Html::parse_document(&html);
        let selector = scraper::Selector::parse("#added").unwrap();
        assert_eq!(
            &document.select(&selector).next().unwrap().inner_html(),
            "John"
        );
        let john_path = &document
            .select(&selector)
            .next()
            .unwrap()
            .attr("href")
            .unwrap();
        let john_id = john_path.split('/').nth(2).unwrap();

        let response = client
            .post("/add-person")
            .header(ContentType::Form)
            .body("name=Mary Ann")
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert!(html.contains(r#"Person added:"#));
        let document = scraper::Html::parse_document(&html);
        let selector = scraper::Selector::parse("#added").unwrap();
        assert_eq!(
            &document.select(&selector).next().unwrap().inner_html(),
            "Mary Ann"
        );
        let mary_ann_path = &document
            .select(&selector)
            .next()
            .unwrap()
            .attr("href")
            .unwrap();
        let mary_ann_id = mary_ann_path.split('/').nth(2).unwrap();

        let response = client.get("/people").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert!(html.contains(r#">John</a></li>"#));
        assert!(html.contains(r#">Mary Ann</a></li>"#));

        let response = client.get("/").dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert!(html.contains(r#"<title>People and Groups</title>"#));
        assert!(html.contains(r#"<div>Number of people: 2</div>"#));
        assert!(html.contains(r#"<div>Number of groups: 0</div>"#));

        let response = client.get(john_path.to_string()).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert!(html.contains(r#"<h2>Name: John</h2>"#));
        let expected = format!(r#"<div><a  href="/add-group?uid={john_id}">add group</a></div>"#);
        assert!(html.contains(&expected));

        let response = client.get(format!("/add-group?uid={john_id}")).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert!(html.contains(r#"<title>Add Group owned by John</title>"#));

        let response = client
            .post("/add-group")
            .header(ContentType::Form)
            .body(format!("name=group one&uid={mary_ann_id}"))
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        println!("html: {}", html);
        assert!(html.contains(r#"<h1>Group called group one owned by Mary Ann was added</h1>"#));
        assert!(html.contains(r#"Group added:"#));

        let document = scraper::Html::parse_document(&html);
        let selector = scraper::Selector::parse("#added").unwrap();
        assert_eq!(
            &document.select(&selector).next().unwrap().inner_html(),
            "group one"
        );
        let group_one_path = &document
            .select(&selector)
            .next()
            .unwrap()
            .attr("href")
            .unwrap();
        let _group_one_id = group_one_path.split('/').nth(2).unwrap();

        let response = client.get(group_one_path.to_string()).dispatch();
        assert_eq!(response.status(), Status::Ok);
        let html = response.into_string().unwrap();
        assert!(html.contains(r#"<h2>Name: group one</h2>"#));
        let expected =
            format!(r#"<h2>Owner name: <a href="/person/{mary_ann_id}">Mary Ann</a></h2>"#);
        assert!(html.contains(&expected));
    }
}
