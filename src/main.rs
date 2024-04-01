use serde::Deserialize;
use std::fs;
use std::fs::File;
use std::io::{self, Read};

static LIMIT: u32 = 1;
#[derive(Debug, Deserialize)]
struct IDLFunction {
    name: String,
    arguments: Vec<(String, String)>,
    return_type: String,
}
// struct argument {
//     enum: ,
//     type: ,
//     size: size_t,
//     ownership: ,
// }

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
fn get_size(_function: &IDLFunction) -> u32 {
    LIMIT + 1
}

// fn generate_client(function: &IDLFunction) -> String {
//     if get_size(&function) < LIMIT {
//         return format!(
//             "{} {}",
//             client_signature(&function),
//             client_body_reg(&function)
//         );
//     } else {
//         return format!(
//             "{} {}",
//             client_signature(&function),
//             client_body_shm(&function)
//         );
//     }
// }
// fn generate_server(function: &IDLFunction) -> String {
//     if get_size(&function) < LIMIT {
//         return format!(
//             "{} {}",
//             server_signature_reg(&function),
//             server_body_reg(&function)
//         );
//     } else {
//         return format!(
//             "{} {}",
//             server_signature_shm(&function),
//             server_body_shm(&function),
//         );
//     }
// }
fn interface_create(function: &IDLFunction) -> String {
    format!(
        r#"int nobjs = 16;
struct {name}_args{{
    {fields}
}};
size_t objsz = sizeof(struct {name}_args);
SHM_BM_INTERFACE_CREATE({name}obj, objsz, nobjs);

"#,
        name = function.name,
        fields = function
            .arguments
            .iter()
            .map(|(arg_type, arg_name)| format!("{} {};", arg_type, arg_name))
            .collect::<Vec<String>>()
            .join("\n\t"),
    )
}
fn client_signature(function: &IDLFunction) -> String {
    format!(
        "{ret} {name}_c({args})",
        ret = function.return_type,
        name = function.name,
        args = function
            .arguments
            .iter()
            .map(|(arg_type, arg_name)| format!("{} {}", arg_type, arg_name))
            .collect::<Vec<String>>()
            .join(","),
    )
}
fn client_body_reg(function: &IDLFunction) -> String {
    format!(
        r#"{{
    return {name}_s({args});
}}"#,
        name = function.name,
        args = function
            .arguments
            .iter()
            .map(|(_arg_type, arg_name)| format!("{}", arg_name))
            .collect::<Vec<String>>()
            .join(",")
    )
}
fn client_body_shm(function: &IDLFunction) -> String {
    format!(
        r#"{{
    struct {name}_args params=(struct {name}_args){{{args}}};
    //TODO: ensure proper allignment
    void * mem = calloc(nobjs,objsz);
    size_t memsz = nobjs*objsz;
    void * shm = shm_bm_create_{name}obj(mem,memsz);
    shm_bm_init_{name}obj(shm);
    shm_bm_objid_t objid;
    void * obj;
    obj = shm_bm_alloc_{name}obj(shm,&objid);
    *(struct {name}_args*)obj = params;
    return {name}_s(shm,objid);
}}"#,
        name = function.name,
        args = function
            .arguments
            .iter()
            .map(|(_arg_type, arg_name)| format!("{}", arg_name))
            .collect::<Vec<String>>()
            .join(",")
    )
}
fn server_signature_reg(function: &IDLFunction) -> String {
    format!(
        "{} {}_s({})",
        function.return_type,
        function.name,
        function
            .arguments
            .iter()
            .map(|(arg_type, arg_name)| format!("{} {}", arg_type, arg_name))
            .collect::<Vec<String>>()
            .join(",")
    )
}
fn server_signature_shm(function: &IDLFunction) -> String {
    format!(
        "{} {}_s(void * shm, shm_bm_objid_t objid)",
        function.return_type, function.name
    )
}
fn server_body_reg(function: &IDLFunction) -> String {
    format!(
        r#"{{
    return {name}({args});
}}"#,
        name = function.name,
        args = function
            .arguments
            .iter()
            .map(|(_arg_type, arg_name)| format!("{}", arg_name))
            .collect::<Vec<String>>()
            .join(",")
    )
}
fn server_body_shm(function: &IDLFunction) -> String {
    format!(
        r#"{{
    struct {name}_args params = *(struct {name}_args*)shm_bm_take_{name}obj(shm,objid);
    shm_bm_free_{name}obj(shm);
    return {name}({args});
}}"#,
        name = function.name,
        args = function
            .arguments
            .iter()
            .map(|(_arg_type, arg_name)| format!("params.{}", arg_name))
            .collect::<Vec<String>>()
            .join(","),
    )
}

fn main() {
    let file_content = read_toml("file.toml").expect("Failed to read file");

    let config: SystemConfig = toml::from_str(&file_content).expect("Failed to parse TOML");

    let mut client_stub = String::new();
    let mut server_stub = String::new();
    let mut total = String::new();
    total += &format!("#include \"shm_bm.h\"\n#include \"header.h\" \n");
    // Moved logic from generate_stubs into main
    for function in config.functions {
        if get_size(&function) < LIMIT {
            server_stub += &server_signature_reg(&function);
            server_stub += &server_body_reg(&function);
            client_stub += &client_signature(&function);
            client_stub += &client_body_reg(&function);
        } else {
            total += &interface_create(&function);
            server_stub += &server_signature_shm(&function);
            server_stub += &server_body_shm(&function);
            client_stub += &client_signature(&function);
            client_stub += &client_body_shm(&function);
        }

        total += &server_stub;
        total += "\n";
        total += &client_stub;
    }
    match fs::write("stubs.h", &total) {
        Ok(_) => println!("C code written to file: {}", "/src/stubs.h"),
        Err(error) => println!("Error writing to file: {}", error),
    }

    println!(
        "Client Stub:\n{}\nServer Stub:\n{}\n",
        client_stub, server_stub
    );
}
