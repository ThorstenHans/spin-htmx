use build_html::{Container, ContainerType, Html, HtmlContainer};
use http::Response;
use serde::{Deserialize, Serialize};
use spin_sdk::http::{IntoResponse, Json, Params, Request, Router};
use spin_sdk::http_component;
use spin_sdk::sqlite::{Connection, Value};

#[http_component]
fn handle_api(req: Request) -> anyhow::Result<impl IntoResponse> {

    // lets use the Router to handle requests based on method and path
    let mut r = Router::default();
    r.post("/api/items", add_new);
    r.get("/api/items", get_all);
    r.delete("/api/items/:id", delete_one);
    Ok(r.handle(req))
}

#[derive(Debug, Deserialize, Serialize)]
struct Item {
    #[serde(skip_deserializing)]
    id: i64,
    value: String,
}

impl Html for Item {
    fn to_html_string(&self) -> String {
        Container::new(ContainerType::Div)
            .with_attributes(vec![
                ("class", "item"),
                ("id", format!("item-{}", &self.id).as_str()),
            ])
            .with_container(
                Container::new(ContainerType::Div)
                    .with_attributes(vec![("class", "value")])
                    .with_raw(&self.value),
            )
            .with_container(
                Container::new(ContainerType::Div)
                    .with_attributes(vec![
                        ("class", "delete-item"),
                        ("hx-delete", format!("/api/items/{}", &self.id).as_str()),
                    ])
                    .with_raw("❌"),
            )
            .to_html_string()
    }
}

fn get_all(_r: Request, _p: Params) -> anyhow::Result<impl IntoResponse> {
    let connection = Connection::open_default()?;

    let row_set = connection.execute("SELECT ID, VALUE FROM ITEMS ORDER BY ID DESC", &[])?;
    let items = row_set
        .rows()
        .map(|row| Item {
            id: row.get::<i64>("ID").unwrap(),
            value: row.get::<&str>("VALUE").unwrap().to_owned(),
        })
        .map(|item| item.to_html_string())
        .reduce(|acc, e| format!("{} {}", acc, e))
        .unwrap_or(String::from(""));

    Ok(Response::builder()
        .status(200)
        .header("Content-Type", "text/html")
        .body(items)?)
}

fn add_new(req: http::Request<Json<Item>>, _params: Params) -> anyhow::Result<impl IntoResponse> {
    let item = req.into_body().0;
    let connection = Connection::open_default()?;
    let parameters = &[Value::Text(item.value)];
    connection.execute("INSERT INTO ITEMS (VALUE) VALUES (?)", parameters)?;
    Ok(Response::builder()
        .status(200)
        .header("HX-Trigger", "newItem")
        .body(())?)
}

fn delete_one(_req: Request, params: Params) -> anyhow::Result<impl IntoResponse> {
    let Some(id) = params.get("id") else {
        return Ok(Response::builder().status(404).body("Missing identifier")?);
    };
    let Ok(id) = id.parse::<i64>() else {
        return Ok(Response::builder()
            .status(400)
            .body("Unexpected identifier format")?);
    };

    let connection = Connection::open_default()?;
    let parameters = &[Value::Integer(id)];
    match connection.execute("DELETE FROM ITEMS WHERE ID = ?", parameters) {
        Ok(_) => Ok(Response::default()),
        Err(e) => {
            println!("Error while deleting item: {}", e);
            Ok(Response::builder()
                .status(500)
                .body("Error while deleting item")?)
        }
    }
}
