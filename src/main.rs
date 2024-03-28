use serde::Deserialize;
use std::fs::File;
use std::io::{self, Read};


size_t Limit = 1;
#[derive(Debug, Deserialize)]
struct IDLFunction {
    name: String,
    arguments: Vec<(String, String)>,
    return_type: String,
}

#[derive(Debug, Deserialize)]
struct SystemConfig {
    functions: Vec<IDLFunction>,
}

fn read_toml(file_path: &str) -> io::Result<String> {
    let mut file = File::open(file_path)?;
    let mut content = String::new();
    file.read_to_string(&mut content)?;
    Ok(content)
}
fn get_size(args: <Vec<String>>){
    return Limit +1;
}

fn generate_client(function: &IDLFunction, ipc : int) -> String {
    let function_signature = format!(
        "{} {}_c({})",
        function.return_type,
        function.name,
        function.arguments
            .iter()
            .map(|(arg_type, arg_name)| format!("{} {}", arg_type, arg_name))
            .collect::<Vec<String>>()
            .join(",")
    );
    if(ipc==0){
        let function_body = format!(
            r#"
            {{
                return {name}_s({args});
            }}"#,
            name = function.name,
            args = function.arguments
                .iter()
                .map(|(_arg_type, arg_name)| format!("{}", arg_name))
                .collect::<Vec<String>>()
                .join(",")
        );
    }
    else if(ipc==1){
        let function_body = format!(
            r#"
            {{
                struct obj_args params=//serialize {args} into testobj
                size_t objsz=sizeOf(struct obj_args);
                //ensure proper allignment
                void * mem = calloc(64,objsz);
                size_t memsz=64*objSize;
                SHM_BM_INTERFACE_CREATE(args, size, 64);
                shm=shm_bm_create_args(mem,memsz,objsz,64);
                shm_bm_init_args(shm,objsz,nobj);
                shm_objid_t objid;
                struct obj_args * obj;
                obj=shm_bm_alloc_args(shm,&objid,objsz, 64);
                *obj= params;
                return {name}_s(shm,obj,objsz,64);
            }}"#,
            name = function.name,
            args = function.arguments
                .iter()
                .map(|(_arg_type, arg_name)| format!("{}", arg_name))
                .collect::<Vec<String>>()
                .join(",")
    }

    format!("{} {}", function_signature, function_body)
}

fn generate_server(function: &IDLFunction, ipc: int) -> String {

    if(ipc == 0){
        let function_signature = format!(
            "{} {}_s({})",
            function.return_type,
            function.name,
            function.arguments
                .iter()
                .map(|(arg_type, arg_name)| format!("{} {}", arg_type, arg_name))
                .collect::<Vec<String>>()
                .join(",")
        );

        let function_body = format!(
            r#"
            {{
                return {name}({args});
            }}"#,
            name = function.name,
            args = function.arguments
                .iter()
                .map(|(_arg_type, arg_name)| format!("{}", arg_name))
                .collect::<Vec<String>>()
                .join(",")
        );
    }
    else if(ipc==1){
        let function_signature = format!(
            "{} {}_s(String shm, shm_objid_t objid,size_t objsz, unsigned int nobj)",
            function.return_type,
            function.name
        );
        let function_body = format!(
            r#"
            {{
                struct obj_args * params=shm_bm_take_args(shm,obj,objsz,nobj);
                return {name}({args});
            }}"#,
            name = function.name,
            args = function.arguments,
            .iter()
            .map(|(_arg_type, arg_name)| format!("params.{}", arg_name))
            .collect::<Vec<String>>()
            .join(",")
        );
    }
    format!("{} {}", function_signature, function_body)

}

fn main() {
    let file_content = read_toml("file.toml").expect("Failed to read file");

    let config: SystemConfig = toml::from_str(&file_content).expect("Failed to parse TOML");

    let mut client_stub = String::new();
    let mut server_stub = String::new();

    // Moved logic from generate_stubs into main
    for function in config.functions {
        //size=getSize(function.arguments);
        if(size>Limit){
            client_stub += &generate_client(&function,1);
            server_stub += &generate_server(&function,1);
        }
        else{
            client_stub += &generate_client(&function,0);
            server_stub += &generate_server(&function,0);
        }

    }

    println!("Client Stub:\n{}\nServer Stub:\n{}\n", client_stub, server_stub);
}
