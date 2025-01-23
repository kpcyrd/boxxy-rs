mod errors;

use crate::errors::*;
use clap::Parser;
use rusoto_core::Region;
use rusoto_lambda::{
    CreateFunctionRequest, FunctionCode, FunctionConfiguration, GetFunctionRequest,
    GetFunctionResponse, InvocationRequest, InvocationResponse, Lambda, LambdaClient,
};
use rusoto_sts::{StsAssumeRoleSessionCredentialsProvider, StsClient};
use rustyline::error::ReadlineError;
use serde::{Deserialize, Serialize};
use std::default::Default;
use std::fmt::{self, Display};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use zip::write::{FileOptions, ZipWriter};

#[derive(Debug, Parser)]
struct Args {
    /// Assume into this role to create the lambda, can be cross account
    #[clap(long = "assume-role")]
    assume_role: Option<String>,
    /// The role your lambda should use
    #[clap(long = "role")]
    role: String,
    /// The aws region name, eg eu-west-1
    region: String,
    /// The name if the lambda you want to create
    function_name: String, // TODO: generate one if empty
}

pub struct BoxxyLambdaClient {
    client: LambdaClient,
}

impl BoxxyLambdaClient {
    pub fn new(role: Option<String>, region: Region) -> BoxxyLambdaClient {
        let client = if let Some(role) = role {
            let sts = StsClient::new(region.clone());
            let provider = StsAssumeRoleSessionCredentialsProvider::new(
                sts,
                role,
                "default".to_string(),
                None,
                None,
                None,
                None,
            );
            // let dispatcher = RequestDispatcher::default();
            let dispatcher = rusoto_core::request::HttpClient::new().unwrap();
            LambdaClient::new_with(dispatcher, provider, region)
        } else {
            LambdaClient::new(region)
        };
        BoxxyLambdaClient { client }
    }

    #[inline]
    pub async fn get_function(&self, x: GetFunctionRequest) -> Result<GetFunctionResponse> {
        let r = self.client.get_function(x).await;
        debug!("get_function response: {:?}", r);
        Ok(r?)
    }

    #[inline]
    pub async fn create_function(&self, x: CreateFunctionRequest) -> Result<FunctionConfiguration> {
        let r = self.client.create_function(x).await;
        debug!("create_function response: {:?}", r);
        Ok(r?)
    }

    pub async fn create_boxxy<I: Into<String>, J: Into<String>>(
        &self,
        function_name: I,
        role: J,
        zip: Vec<u8>,
    ) -> Result<String> {
        let x = self
            .create_function(CreateFunctionRequest {
                code: FunctionCode {
                    zip_file: Some(zip.into()),
                    ..Default::default()
                },
                function_name: function_name.into(),
                handler: Some("client.run".to_string()),
                role: role.into(),
                runtime: Some("python3.6".to_string()),
                ..Default::default()
            })
            .await?;

        let function = x.function_arn.unwrap();
        Ok(function)
    }

    pub async fn ensure_function_exists<I: Into<String>, J: Into<String>>(
        &self,
        function: I,
        role: J,
        zip: Vec<u8>,
    ) -> Result<String> {
        let function_name = function.into();

        let f = self
            .get_function(GetFunctionRequest {
                function_name: function_name.clone(),
                ..Default::default()
            })
            .await;

        let function_arn = match f {
            Ok(f) => {
                println!("[+] lambda already exists");
                let config = f.configuration.unwrap();
                config.function_arn.unwrap()
            }
            Err(_) => {
                println!("[+] creating function...");
                self.create_boxxy(function_name, role, zip).await?
            }
        };

        Ok(function_arn)
    }

    #[inline]
    pub async fn invoke(&self, x: InvocationRequest) -> Result<InvocationResponse> {
        let r = self.client.invoke(x).await;
        debug!("invoke response: {:?}", r);
        Ok(r?)
    }

    pub async fn invoke_boxxy<I: Into<String>>(
        &self,
        function: I,
        cmd: &Command,
    ) -> Result<ResponseResult> {
        let function = function.into();
        info!("invoking {:?} with {:?}", function, cmd);
        let payload = serde_json::to_string(cmd)?;

        let r = self
            .invoke(InvocationRequest {
                function_name: function,
                payload: Some(payload.into()),
                ..Default::default()
            })
            .await?;

        let payload = r.payload.unwrap();

        let resp = String::from_utf8(payload.to_vec())?;
        debug!("invoke response payload: {:?}", resp);

        let resp = serde_json::from_str::<RawResponse>(&resp)?;
        // TODO: this should try to decode stdout and stderr
        debug!("invoke response payload decoded: {:?}", resp);

        Ok(resp.into())
    }
}

fn build_zip() -> Result<Vec<u8>> {
    let mut zip = Vec::new();
    {
        let mut w = io::Cursor::new(&mut zip);
        let mut writer = ZipWriter::new(&mut w);

        let options = FileOptions::default().compression_method(zip::CompressionMethod::Stored);
        writer.start_file("client.py", options)?;
        writer.write_all(include_bytes!("../../contrib/aws-wrapper.py"))?;

        let mut buf2 = Vec::new();
        let mut file = File::open("../target/x86_64-unknown-linux-musl/release/examples/boxxy")?;
        file.read_to_end(&mut buf2)?;

        let options = FileOptions::default()
            .compression_method(zip::CompressionMethod::Stored)
            .unix_permissions(0o755);
        writer.start_file("boxxy", options)?;
        writer.write_all(&buf2)?;

        writer.finish()?;
    }
    Ok(zip)
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Command {
    stdin: String,
}

impl Command {
    #[inline]
    pub fn new<I: Into<String>>(cmd: I) -> Command {
        Command { stdin: cmd.into() }
    }
}

#[derive(Debug, Deserialize)]
pub struct RawResponse {
    #[serde(rename = "errorType")]
    error_type: Option<String>,
    #[serde(rename = "errorMessage")]
    error_message: Option<String>,
    stdout: Option<String>,
    stderr: Option<String>,
}

impl From<RawResponse> for ResponseResult {
    fn from(resp: RawResponse) -> ResponseResult {
        if resp.error_type.is_some() || resp.error_message.is_some() {
            ResponseResult::Err(LambdaError {
                error_type: resp.error_type,
                error_message: resp.error_message,
            })
        } else {
            let stdout = base64::decode(resp.stdout.unwrap()).unwrap();
            let stderr = base64::decode(resp.stderr.unwrap()).unwrap();
            ResponseResult::Ok(Response { stdout, stderr })
        }
    }
}

// TODO: maybe make this a Result<T, E>
#[derive(Debug)]
pub enum ResponseResult {
    Ok(Response),
    Err(LambdaError),
}

#[derive(Debug)]
pub struct Response {
    stdout: Vec<u8>,
    stderr: Vec<u8>,
}

#[derive(Debug)]
pub struct LambdaError {
    error_type: Option<String>,
    error_message: Option<String>,
}

#[derive(Debug)]
pub enum Mode {
    Boxxy,
    Exec,
}

impl Display for Mode {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Mode::Boxxy => write!(f, "boxxy"),
            Mode::Exec => write!(f, "boxxy/exec"),
        }
    }
}

pub fn escape(line: &str) -> String {
    line.chars()
        .flat_map(|c| match c {
            ' ' => vec!['\\', ' '],
            c => vec![c],
        })
        .collect()
}

#[tokio::main]
async fn main() -> Result<()> {
    env_logger::init();
    let args = Args::from_args();

    let region = args.region.parse::<Region>()?;
    info!("Using region: {:?}", region);

    let zip = build_zip()?;
    println!("[*] created zip, {} bytes", zip.len());

    let lambda = BoxxyLambdaClient::new(args.assume_role, region);
    println!("[*] created lambda client");

    let function_arn = lambda
        .ensure_function_exists(args.function_name.as_str(), args.role, zip)
        .await?;
    println!("[+] function {:?} is ready", function_arn);

    let mut stdout = io::stdout();
    let mut stderr = io::stderr();
    let mut mode = Mode::Boxxy;
    let mut rl = rustyline::Editor::<()>::new();

    println!("[*] type \"enter {{boxxy,exec}}\" to change between modes");
    loop {
        let readline = rl.readline(&format!("[lambda/{}]> ", mode));

        match readline {
            // ignore empty lines
            Ok(ref line) if line.is_empty() => (),
            Ok(line) => {
                rl.add_history_entry(line.as_str());

                if line.starts_with("enter ") {
                    if line.starts_with("enter b") {
                        mode = Mode::Boxxy;
                    } else if line.starts_with("enter e") {
                        mode = Mode::Exec;
                    } else {
                        error!("unknown mode: {:?}", line);
                    }
                    continue;
                }

                if line == "exit" {
                    break;
                }

                let cmd = match mode {
                    Mode::Boxxy => Command::new(line),
                    Mode::Exec => Command::new(format!("exec sh -c {}", escape(&line))),
                };

                match lambda
                    .invoke_boxxy(args.function_name.as_str(), &cmd)
                    .await?
                {
                    ResponseResult::Ok(resp) => {
                        stdout.write_all(&resp.stdout)?;
                        stdout.flush()?;
                        stderr.write_all(&resp.stderr)?;
                        stderr.flush()?;
                    }
                    ResponseResult::Err(err) => {
                        error!(
                            "lambda error: {:?}: {:?}",
                            err.error_type, err.error_message
                        );
                    }
                };
            }
            Err(ReadlineError::Interrupted) => (),
            Err(ReadlineError::Eof) => {
                break;
            }
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }

    // println!("TODO: cleanup");

    Ok(())
}
