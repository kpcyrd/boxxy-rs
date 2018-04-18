extern crate rusoto_core;
extern crate rusoto_lambda;
extern crate rusoto_sts;
extern crate zip;
extern crate base64;
extern crate rustyline;
extern crate env_logger;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate structopt;
#[macro_use] extern crate error_chain;
#[macro_use] extern crate log;

use rusoto_core::Region;
use rusoto_core::reactor::{RequestDispatcher, CredentialsProvider};
use rusoto_lambda::{LambdaClient, Lambda,
                    InvocationRequest, InvocationResponse,
                    GetFunctionRequest, GetFunctionResponse,
                    CreateFunctionRequest, FunctionConfiguration,
                    FunctionCode};
use rusoto_sts::{StsClient, StsAssumeRoleSessionCredentialsProvider};

use structopt::StructOpt;
use zip::write::{ZipWriter, FileOptions};
use rustyline::error::ReadlineError;

use std::io;
use std::io::prelude::*;
use std::fs::File;
use std::fmt::{self, Display};
use std::default::Default;


mod errors {
    use std;
    use serde_json;
    use rusoto_core;
    use rusoto_lambda;
    use zip;

    error_chain!{
        foreign_links {
            Io(std::io::Error);
            Utf8(std::string::FromUtf8Error);
            ParseRegion(rusoto_core::ParseRegionError);
            GetFunction(rusoto_lambda::GetFunctionError);
            CreateFunction(rusoto_lambda::CreateFunctionError);
            Invoke(rusoto_lambda::InvokeError);
            Json(serde_json::Error);
            Zip(zip::result::ZipError);
        }
    }
}
use errors::{Result};


#[derive(StructOpt, Debug)]
struct Args {
    #[structopt(long="assume-role",
                help="Assume into this role to create the lambda, can be cross account")]
    assume_role: Option<String>,
    #[structopt(long="role",
                help="The role your lambda should use")]
    role: String,
    #[structopt(help="The aws region name, eg eu-west-1")]
    region: String,
    #[structopt(help="The name if the lambda you want to create")]
    function_name: String, // TODO: generate one if empty
}


pub enum BoxxyLambdaClient {
    StsAssumeRole(LambdaClient<StsAssumeRoleSessionCredentialsProvider>),
    Credentials(LambdaClient<CredentialsProvider>),
}

impl BoxxyLambdaClient {
    pub fn new(role: Option<String>, region: Region) -> BoxxyLambdaClient {
        match role {
            Some(role) => {
                let sts = StsClient::simple(region.clone());
                let provider = StsAssumeRoleSessionCredentialsProvider::new(
                    sts,
                    role,
                    "default".to_string(),
                    None, None, None, None
                );
                let dispatcher = RequestDispatcher::default();
                let lambda = LambdaClient::new(dispatcher, provider, region.clone());
                BoxxyLambdaClient::StsAssumeRole(lambda)
            },
            None => {
                let provider = CredentialsProvider::default();
                let dispatcher = RequestDispatcher::default();
                let lambda = LambdaClient::new(dispatcher, provider, region.clone());
                BoxxyLambdaClient::Credentials(lambda)
            },
        }
    }

    #[inline]
    pub fn get_function(&self, x: &GetFunctionRequest) -> Result<GetFunctionResponse> {
        use BoxxyLambdaClient::*;
        let r = match *self {
            StsAssumeRole(ref c) => c.get_function(x).sync(),
            Credentials(ref c) => c.get_function(x).sync(),
        };
        debug!("get_function response: {:?}", r);
        Ok(r?)
    }

    #[inline]
    pub fn create_function(&self, x: &CreateFunctionRequest) -> Result<FunctionConfiguration> {
        use BoxxyLambdaClient::*;
        let r = match *self {
            StsAssumeRole(ref c) => c.create_function(x).sync(),
            Credentials(ref c) => c.create_function(x).sync(),
        };
        debug!("create_function response: {:?}", r);
        Ok(r?)
    }

    pub fn create_boxxy<I: Into<String>, J: Into<String>>(&self, function_name: I, role: J, zip: Vec<u8>) -> Result<String> {
        let x = self.create_function(&CreateFunctionRequest {
            code: FunctionCode {
                zip_file: Some(zip),
                ..Default::default()
            },
            function_name: function_name.into(),
            handler: "client.run".to_string(),
            role: role.into(),
            runtime: "python3.6".to_string(),
            ..Default::default()
        })?;

        let function = x.function_arn.unwrap();
        Ok(function)
    }

    pub fn ensure_function_exists<I: Into<String>, J: Into<String>>(&self, function: I, role: J, zip: Vec<u8>) -> Result<String> {
        let function_name = function.into();

        let f = self.get_function(&GetFunctionRequest {
            function_name: function_name.clone(),
            ..Default::default()
        });

        let function_arn = match f {
            Ok(f) => {
                println!("[+] lambda already exists");
                let config = f.configuration.unwrap();
                config.function_arn.unwrap()
            },
            Err(_) => {
                println!("[+] creating function...");
                self.create_boxxy(function_name,
                                  role,
                                  zip)?
            },
        };

        Ok(function_arn)
    }

    #[inline]
    pub fn invoke(&self, x: &InvocationRequest) -> Result<InvocationResponse> {
        use BoxxyLambdaClient::*;
        let r = match *self {
            StsAssumeRole(ref c) => c.invoke(x).sync(),
            Credentials(ref c) => c.invoke(x).sync(),
        };
        debug!("invoke response: {:?}", r);
        Ok(r?)
    }

    pub fn invoke_boxxy<I: Into<String>>(&self, function: I, cmd: &Command) -> Result<ResponseResult> {
        let function = function.into();
        info!("invoking {:?} with {:?}", function, cmd);
        let payload = serde_json::to_string(cmd)?;

        let r = self.invoke(&InvocationRequest {
            function_name: function,
            payload: Some(payload.into_bytes()),
            ..Default::default()
        })?;

        let payload = r.payload.unwrap();

        let resp = String::from_utf8(payload)?;
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

        let options = FileOptions::default()
                        .compression_method(zip::CompressionMethod::Stored);
        writer.start_file("client.py", options)?;
        writer.write_all(include_bytes!("../contrib/aws-wrapper.py"))?;

        let mut buf2 = Vec::new();
        let mut file = File::open("target/x86_64-unknown-linux-musl/release/examples/boxxy")?;
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
        Command {
            stdin: cmd.into(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct RawResponse {
    #[serde(rename="errorType")]
    error_type: Option<String>,
    #[serde(rename="errorMessage")]
    error_message: Option<String>,
    stdout: Option<String>,
    stderr: Option<String>,
}

impl Into<ResponseResult> for RawResponse {
    fn into(self) -> ResponseResult {
        if self.error_type.is_some() || self.error_message.is_some() {
            ResponseResult::Err(LambdaError {
                error_type: self.error_type,
                error_message: self.error_message,
            })
        } else {
            let stdout = base64::decode(&self.stdout.unwrap()).unwrap();
            let stderr = base64::decode(&self.stderr.unwrap()).unwrap();
            ResponseResult::Ok(Response {
                stdout,
                stderr,
            })
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
        .flat_map(|c| {
            match c {
                ' ' => vec!['\\', ' '],
                c   => vec![c],
            }
        })
        .collect()
}

fn run() -> Result<()> {
    env_logger::init();
    let args = Args::from_args();

    let region = args.region.parse::<Region>()?;
    info!("Using region: {:?}", region);

    let zip = build_zip()?;
    println!("[*] created zip, {} bytes", zip.len());

    let lambda = BoxxyLambdaClient::new(args.assume_role, region);
    println!("[*] created lambda client");

    let function_arn = lambda.ensure_function_exists(args.function_name.as_str(),
                                                     args.role,
                                                     zip)?;
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
            Ok(ref line) if line.len() == 0 => (),
            Ok(line) => {
                rl.add_history_entry(line.as_ref());

                if line.starts_with("enter ") {
                    if line.starts_with("enter b") {
                        mode = Mode::Boxxy;
                    } else if line.starts_with("enter e") {
                        mode = Mode::Exec;
                    } else {
                        error!("unknown mode: {:?}", line);
                    }
                    continue
                }

                if line == "exit" {
                    break;
                }

                let cmd = match mode {
                    Mode::Boxxy => Command::new(line),
                    Mode::Exec => {
                        Command::new(format!("exec sh -c {}", escape(&line)))
                    },
                };

                match lambda.invoke_boxxy(args.function_name.as_str(), &cmd)? {
                    ResponseResult::Ok(resp) => {
                        stdout.write_all(&resp.stdout)?;
                        stdout.flush()?;
                        stderr.write_all(&resp.stderr)?;
                        stderr.flush()?;
                    },
                    ResponseResult::Err(err) => {
                        error!("lambda error: {:?}: {:?}", err.error_type, err.error_message);
                    }
                };
            },
            Err(ReadlineError::Interrupted) => (),
            Err(ReadlineError::Eof) => {
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            },
        }
    }

    // println!("TODO: cleanup");

    Ok(())
}

fn main() {
    if let Err(ref e) = run() {
        eprintln!("error: {}", e);

        for e in e.iter().skip(1) {
            eprintln!("caused by: {}", e);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            eprintln!("backtrace: {:?}", backtrace);
        }

        ::std::process::exit(1);
    }
}
