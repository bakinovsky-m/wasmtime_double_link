use wasmtime::component::{bindgen, HasSelf};
use wasmtime::component::ResourceTable;
// use wasmtime_wasi::p2::IoImpl;
use wasmtime_wasi::p2::IoView;
use wasmtime_wasi::p2::WasiCtx;
use wasmtime_wasi::p2::WasiCtxBuilder;
// use wasmtime_wasi::p2::WasiImpl;
use wasmtime_wasi::p2::WasiView;


bindgen!({
//   inline: r#"
//       package my:project;
//       package wasi:cli@0.2.3 {
//         interface environment {
//         //   export get_env: func(name: string) -> string;
//         }
//       }
//       world hello-world {
//         import wasi:cli/environment@0.2.3;
//         import name: func() -> string;
//         //   export greet: func();
//         export add: func(x: u32, y: u32) -> u32;
//       }
//   "#,
    inline: r#"
    package root:component;

world root {
  import wasi:cli/environment@0.2.3;
  import wasi:cli/exit@0.2.3;
  import wasi:io/error@0.2.3;
  import wasi:io/streams@0.2.3;
  import wasi:cli/stdin@0.2.3;
  import wasi:cli/stdout@0.2.3;
  import wasi:cli/stderr@0.2.3;
  import wasi:clocks/wall-clock@0.2.3;
  import wasi:filesystem/types@0.2.3;
  import wasi:filesystem/preopens@0.2.3;

  //export add: func(x: u32, y: u32) -> u32;
  export get-str: func() -> string;
}
package wasi:io@0.2.3 {
  interface error {
    resource error;
  }
  interface streams {
    use error.{error};

    resource output-stream {
      check-write: func() -> result<u64, stream-error>;
      write: func(contents: list<u8>) -> result<_, stream-error>;
      blocking-write-and-flush: func(contents: list<u8>) -> result<_, stream-error>;
      blocking-flush: func() -> result<_, stream-error>;
    }

    variant stream-error {
      last-operation-failed(error),
      closed,
    }

    resource input-stream;
  }
}


package wasi:cli@0.2.3 {
  interface environment {
    get-environment: func() -> list<tuple<string, string>>;
    //get-arguments: func() -> list<string>;
    //initial-cwd: func() -> option<string>;
  }
  interface exit {
    exit: func(status: result);
  }
  interface stdin {
    use wasi:io/streams@0.2.3.{input-stream};

    get-stdin: func() -> input-stream;
  }
  interface stdout {
    use wasi:io/streams@0.2.3.{output-stream};

    get-stdout: func() -> output-stream;
  }
  interface stderr {
    use wasi:io/streams@0.2.3.{output-stream};

    get-stderr: func() -> output-stream;
  }
}


package wasi:clocks@0.2.3 {
  interface wall-clock {
    record datetime {
      seconds: u64,
      nanoseconds: u32,
    }
  }
}


package wasi:filesystem@0.2.3 {
  interface types {
    use wasi:io/streams@0.2.3.{output-stream};
    use wasi:clocks/wall-clock@0.2.3.{datetime};
    use wasi:io/streams@0.2.3.{error};

    resource descriptor {
      write-via-stream: func(offset: filesize) -> result<output-stream, error-code>;
      append-via-stream: func() -> result<output-stream, error-code>;
      get-type: func() -> result<descriptor-type, error-code>;
      stat: func() -> result<descriptor-stat, error-code>;
    }

    type filesize = u64;

    enum error-code {
      access,
      would-block,
      already,
      bad-descriptor,
      busy,
      deadlock,
      quota,
      exist,
      file-too-large,
      illegal-byte-sequence,
      in-progress,
      interrupted,
      invalid,
      io,
      is-directory,
      loop,
      too-many-links,
      message-size,
      name-too-long,
      no-device,
      no-entry,
      no-lock,
      insufficient-memory,
      insufficient-space,
      not-directory,
      not-empty,
      not-recoverable,
      unsupported,
      no-tty,
      no-such-device,
      overflow,
      not-permitted,
      pipe,
      read-only,
      invalid-seek,
      text-file-busy,
      cross-device,
    }

    enum descriptor-type {
      unknown,
      block-device,
      character-device,
      directory,
      fifo,
      symbolic-link,
      regular-file,
      socket,
    }

    type link-count = u64;

    record descriptor-stat {
      %type: descriptor-type,
      link-count: link-count,
      size: filesize,
      data-access-timestamp: option<datetime>,
      data-modification-timestamp: option<datetime>,
      status-change-timestamp: option<datetime>,
    }

    filesystem-error-code: func(err: borrow<error>) -> option<error-code>;
  }
  interface preopens {
    use types.{descriptor};

    get-directories: func() -> list<tuple<descriptor, string>>;
  }
}
    "#,
});

struct MyState {
    ctx: WasiCtx,
    table: ResourceTable,
}
impl IoView for MyState {
    fn table(&mut self) -> &mut ResourceTable {
        &mut self.table
    }
}
impl WasiView for MyState {
    fn ctx(&mut self) -> &mut WasiCtx {
        &mut self.ctx
    }
}
pub struct MyWasiIoError;
pub enum StreamError {
    Default,
}

impl wasi::filesystem::preopens::Host for MyState {
    fn get_directories(
        &mut self,
    ) -> wasmtime::component::__internal::Vec<(
        wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        wasmtime::component::__internal::String,
    )> {
        Vec::<(
            wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
            wasmtime::component::__internal::String,
        )>::new()
    }
}

impl wasi::clocks::wall_clock::Host for MyState {}

impl wasi::filesystem::types::Host for MyState {
    fn filesystem_error_code(
        &mut self,
        err: wasmtime::component::Resource<wasi::io::streams::Error>,
    ) -> Option<wasi::filesystem::types::ErrorCode> {
        match err {
            _ => Some(wasi::filesystem::types::ErrorCode::Unsupported),
        }
    }
}

impl wasi::filesystem::types::HostDescriptor for MyState {
    fn write_via_stream(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
        _offset: wasi::filesystem::types::Filesize,
    ) -> Result<
        wasmtime::component::Resource<wasi::filesystem::types::OutputStream>,
        wasi::filesystem::types::ErrorCode,
    > {
        Err(wasi::filesystem::types::ErrorCode::Unsupported)
    }

    fn append_via_stream(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
    ) -> Result<
        wasmtime::component::Resource<wasi::filesystem::types::OutputStream>,
        wasi::filesystem::types::ErrorCode,
    > {
        Err(wasi::filesystem::types::ErrorCode::Unsupported)
    }

    fn get_type(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
    ) -> Result<wasi::filesystem::types::DescriptorType, wasi::filesystem::types::ErrorCode> {
        Ok(wasi::filesystem::types::DescriptorType::Unknown)
    }

    fn stat(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
    ) -> Result<wasi::filesystem::types::DescriptorStat, wasi::filesystem::types::ErrorCode> {
        Err(wasi::filesystem::types::ErrorCode::Unsupported)
    }

    fn drop(
        &mut self,
        _rep: wasmtime::component::Resource<wasi::filesystem::types::Descriptor>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl wasi::cli::stderr::Host for MyState {
    fn get_stderr(&mut self) -> wasmtime::component::Resource<wasi::io::streams::OutputStream> {
        wasmtime::component::Resource::<wasi::io::streams::OutputStream>::new_own(10000)
    }
}

impl wasi::cli::stdout::Host for MyState {
    fn get_stdout(&mut self) -> wasmtime::component::Resource<wasi::io::streams::OutputStream> {
        wasmtime::component::Resource::<wasi::io::streams::OutputStream>::new_own(10000)
    }
}

impl wasi::cli::stdin::Host for MyState {
    fn get_stdin(&mut self) -> wasmtime::component::Resource<wasi::io::streams::InputStream> {
        wasmtime::component::Resource::<wasi::io::streams::InputStream>::new_own(10000)
    }
}

impl wasi::cli::exit::Host for MyState {
    fn exit(&mut self, status: wasmtime::Result<(), ()>) {
        println!("exit: {:?}", status);
    }
}

impl wasi::cli::environment::Host for MyState {
    fn get_environment(&mut self) -> Vec<(String,String)> {
        vec![("ALL GOOD".to_string(), "ALL GOOD".to_string())]
    }
    // fn get_arguments(&mut self) -> Vec<String> {
    //     vec![]
    // }
    // fn initial_cwd(&mut self) -> Option<String> {
    //     Some(".".to_string())
    // }
}

impl wasi::io::streams::OutputStream {}

impl wasi::io::streams::HostOutputStream for MyState {
    fn check_write(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
    ) -> Result<u64, wasi::io::streams::StreamError> {
        Ok(0) //Err(wasi::io::streams::StreamError::Closed)
    }

    fn write(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
        _contents: wasmtime::component::__internal::Vec<u8>,
    ) -> Result<(), wasi::io::streams::StreamError> {
        Ok(()) //Err(wasi::io::streams::StreamError::Closed)
    }

    fn blocking_write_and_flush(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
        _contents: wasmtime::component::__internal::Vec<u8>,
    ) -> Result<(), wasi::io::streams::StreamError> {
        Ok(()) //Err(wasi::io::streams::StreamError::Closed)
    }

    fn blocking_flush(
        &mut self,
        _self_: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
    ) -> Result<(), wasi::io::streams::StreamError> {
        Ok(()) //Err(wasi::io::streams::StreamError::Closed)
    }

    fn drop(
        &mut self,
        _rep: wasmtime::component::Resource<wasi::io::streams::OutputStream>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl wasi::io::streams::HostInputStream for MyState {
    fn drop(
        &mut self,
        _rep: wasmtime::component::Resource<wasi::io::streams::InputStream>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}

impl wasi::io::streams::Host for MyState {}
impl wasi::io::error::HostError for MyState {
    fn drop(
        &mut self,
        _rep: wasmtime::component::Resource<wasi::io::error::Error>,
    ) -> wasmtime::Result<()> {
        Ok(())
    }
}
impl wasi::io::error::Host for MyState {}

fn main() -> wasmtime::Result<()> {
    let engine = wasmtime::Engine::default();
    let mut store = wasmtime::Store::new(&engine, MyState{
        ctx: WasiCtxBuilder::new().build(),
        table: ResourceTable::new(),
    });

    let path = "./a.wasm";
    let component = wasmtime::component::Component::from_file(&engine, path)
        .expect("failed to compile module");

    let mut linker = wasmtime::component::Linker::new(&engine);
    Root::add_to_linker::<_, HasSelf<_>>(&mut linker, |state| state)?;

    let instance = linker.instantiate(&mut store, &component)
        .expect("failed to instantiate module");

    let get_str = instance
        .get_func(&mut store, "get-str")
        .expect("failed to get get_str function")
        .typed::<(), (String,)>(&store)
        .expect("failed to type get_str function");

    let (result,) = get_str
        .call(&mut store, ())
        .expect("failed to call get_str function");

    println!("RESULT: {}", result);

    Ok(())

}