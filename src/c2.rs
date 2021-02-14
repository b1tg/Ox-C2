#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoReq {}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteReq {
    #[prost(string, tag = "1")]
    pub cmd: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Task {
    #[prost(oneof = "task::Data", tags = "1, 2")]
    pub data: ::std::option::Option<task::Data>,
}
pub mod task {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "1")]
        Info(super::InfoReq),
        #[prost(message, tag = "2")]
        Execute(super::ExecuteReq),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct InfoRes {
    #[prost(string, tag = "1")]
    pub mac: std::string::String,
    #[prost(string, tag = "2")]
    pub ip: std::string::String,
    #[prost(string, tag = "3")]
    pub username: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ExecuteRes {
    #[prost(bool, tag = "1")]
    pub status: bool,
    #[prost(string, tag = "2")]
    pub data: std::string::String,
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct TaskResult {
    #[prost(oneof = "task_result::Data", tags = "1, 2")]
    pub data: ::std::option::Option<task_result::Data>,
}
pub mod task_result {
    #[derive(Clone, PartialEq, ::prost::Oneof)]
    pub enum Data {
        #[prost(message, tag = "1")]
        Info(super::InfoRes),
        #[prost(message, tag = "2")]
        Execute(super::ExecuteRes),
    }
}
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Empty {}
