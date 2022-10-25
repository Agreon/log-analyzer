use std::collections::HashMap;

use actix_web::{body::MessageBody, error, web, App, HttpResponse, HttpServer, Responder};

use serde_json::Value;

#[derive(Debug)]
enum ColumnType {
    Null,
    Bool,
    Number,
    String,
}

#[derive(Debug)]
struct Column {
    key: String,
    column_type: ColumnType,
    value: serde_json::Value,
}

struct Log {
    pub gid: String,
    pub time: u64,
    // TODO: Rather HashMap?
    pub columns: Vec<Column>,
}

struct IColumn<'a> {
    pub key: String,
    pub value: &'a serde_json::Value,
}

fn parse_log_into_columns(raw_json: String) -> Result<Vec<Column>, serde_json::Error> {
    let body: HashMap<String, serde_json::Value> = serde_json::from_str(&raw_json)?;
    let mut columns: Vec<Column> = Vec::with_capacity(body.len());

    let mut values_to_check: Vec<IColumn> =
        Vec::from_iter(body.iter().map(|(key, value)| IColumn {
            key: key.clone(),
            value,
        }));

    let mut i = 0;
    loop {
        if i >= values_to_check.len() {
            break;
        }

        let current = &values_to_check[i];
        let current_key = current.key.clone();

        if current.value.is_object() {
            let children = current
                .value
                .as_object()
                .unwrap()
                .iter()
                .map(|(key, value)| IColumn {
                    key: format!("{}.{}", current_key, key),
                    value,
                });

            values_to_check.extend(children);
        // TODO: Should we even index array values?
        } else if current.value.is_array() {
            // let current_children = current.value.as_array().unwrap().iter();

            // let object_children = current_children
            //     .filter(|predicate| predicate.is_object())
            //     .flat_map(|obj| {
            //         obj.as_object().unwrap().iter().map(|(key, value)| IColumn {
            //             key: format!("{}.{}", current_key, key),
            //             value,
            //         })
            //     });

            // values_to_check.extend(object_children);
            // let children = current
            //     .value
            //     .as_array()
            //     .unwrap()
            //     .iter()
            //     .map(|(key, value)| IColumn { key, value });
        } else {
            columns.push(Column {
                key: current_key,
                value: current.value.clone(),
                column_type: match current.value {
                    Value::Null => ColumnType::Null,
                    Value::Bool(_) => ColumnType::Bool,
                    Value::Number(_) => ColumnType::Number,
                    Value::String(_) => ColumnType::String,
                    // _ => return Result::Err(format!("Unexpected type {:?}", current.value)),
                    // TODO: What on array?
                    _ => panic!("Unexpected type {:?}", current.value),
                },
            })
        }

        i += 1;
    }

    return Result::Ok(columns);
}

struct AddLogErr {
    pub message: String,
}

// TODO: Return ref of log?
fn add_log(req_body: String) -> Result<Log, AddLogErr> {
    let parsed_columns = parse_log_into_columns(req_body);

    // if parsed_columns.is_err() {
    //     return Err(AddLogErr {
    //         message: "Parsing error".to_string(),
    //     });
    // }

    let columns = parsed_columns.unwrap();
    // let cols = parsed_columns..ok_or_else(|_| AddLogErr {
    //     message: "".to_string(),
    // })?;

    let time: u64 = columns
        .iter()
        .find(|column| column.key == "time")
        .ok_or_else(|| AddLogErr {
            message: "'time' column is missing".to_string(),
        })?
        .value
        .as_u64()
        .ok_or_else(|| AddLogErr {
            message: "'time' column has wrong format".to_string(),
        })?;

    Ok(Log {
        gid: xid::new().to_string(),
        time,
        columns,
    })
}

async fn add_log_wrap(req_body: String) -> impl Responder {
    let res = add_log(req_body);

    match res {
        Err(error) => {
            return HttpResponse::BadRequest()
                .message_body(MessageBody::boxed(error.message))
                .unwrap()
        }
        // TODO: Push the full log into the queue
        Ok(log) => {
            println!("{} {}", log.time, log.gid);
            for column in log.columns {
                println!(
                    "{}: {} ({:?})",
                    column.key, column.value, column.column_type
                );
            }
            return HttpResponse::Ok().finish();
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let entries = [
    //     LogEntry {
    //         time: 1665622800655,
    //     },
    //     LogEntry {
    //         time: 1665622800656,
    //     },
    //     LogEntry {
    //         time: 1665622800657,
    //     },
    // ];

    // let tree = MainTree::new();
    // tree.insert(LogEntry {
    //     time: 1665622800655,
    // });

    HttpServer::new(|| {
        let json_config = web::JsonConfig::default()
            .limit(4096)
            .error_handler(|err, _req| {
                error::InternalError::from_response(err, HttpResponse::BadRequest().finish()).into()
            });

        App::new().service(
            web::resource("/log")
                .app_data(json_config)
                .route(web::post().to(add_log_wrap)),
        )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

// #[cfg(test)]
// mod tests {
//     use crate::{ListNode, UnrolledLinkedList};

//     #[test]
//     fn it_works() {
//         let mut first_node = ListNode::new(16);
//         first_node.items.push(111);
//         let mut second_node: ListNode<i32> = ListNode::new(16);
//         second_node.items.push(222);

//         let mut l_list: UnrolledLinkedList<i32> = UnrolledLinkedList::new(first_node);

//         l_list.add(second_node);

//         // assert_eq!(l_list.first, l_list.last);

//         // format!("{:?}", l_list);
//     }
// }
