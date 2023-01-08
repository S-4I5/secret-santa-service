use ureq::{Error};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args
{
    #[arg(short, long)]
    s_command: String,
    #[arg(short, long)]
    initiator_id: Option<i32>,
    #[arg(short, long)]
    name: Option<String>,
    #[arg(short, long)]
    group_id: Option<i32>,
    #[arg(short, long)]
    candidate_id: Option<i32>,
}

fn main() {
    let args = Args::parse();
    let addr = "127.0.0.1:8079";
    let resp = match args.s_command.as_str() {

        "create_user" => ureq::post(format!("{}/users", addr).as_str())
            .send_json(ureq::json!({
                "name" : args.name
            })),

        "get_groups" => ureq::get(format!("{}/groups", addr).as_str()).send_json(ureq::json!({})),

        "create_group" => ureq::post(format!("{}/groups", addr).as_str())
            .send_json(ureq::json!({
                "creator_id" : args.initiator_id,
                "group_title" : args.name
            })),

        "get_group_members" => ureq::get(format!("{}/group/{:?}/members", addr, args.group_id).as_str()).send_json(ureq::json!({})),

        "get_group_admins" => ureq::get(format!("{}/group/{:?}/admins", addr, args.group_id).as_str()).send_json(ureq::json!({})),

        "join_group" => ureq::post(format!("{}/group/{:?}/join", addr, args.group_id).as_str())
            .send_json(ureq::json!({
                "id" : args.initiator_id,
            })),

        "leave_group" => ureq::post(format!("{}/group/{:?}/leave", addr, args.group_id).as_str())
            .send_json(ureq::json!({
                "id" : args.initiator_id,
            })),

        "delete_group" => ureq::put(format!("{}/groups/{:?}/delete", addr, args.group_id).as_str())
            .send_json(ureq::json!({
                "id" : args.initiator_id,
            })),

        "add_admin" => ureq::post(format!("{}/groups/{:?}/admin", addr, args.group_id).as_str())
            .send_json(ureq::json!({
                "initiator_id" : args.initiator_id,
                "candidate_id": args.candidate_id
            })),

        "remove_admin" => ureq::post(format!("{}/groups/{:?}/unadmin", addr, args.group_id).as_str())
            .send_json(ureq::json!({
                "initiator_id": args.initiator_id,
                "candidate_id": args.candidate_id
            })),

        "start_secret_santa" => ureq::post(format!("{}/groups/{:?}/secret-santa/start", addr, args.group_id).as_str())
            .send_json(ureq::json!({
                "id": args.initiator_id,
            })),

        "get_santa_name" => ureq::get(format!("{}/groups/{:?}/secret-santa", addr, args.group_id).as_str())
            .send_json(ureq::json!({
                "id": args.initiator_id,
            })),

        _ => panic!("Unknown command!")

    };
    match resp {
        Ok(response) => {
            println!("Response: {response:?}");
        }
        Err(Error::Status(code, response)) => {
            println!("Status code: {code} {0:?}", response.status_text());
            println!("Response: {response:?}");
        }
        Err(_) => {}
    }
}