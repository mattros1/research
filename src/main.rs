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
fn retshm_interface_create(function: &IDLFunction) -> String {
    format!(
        r#"static int ret_nobjs = 16;
static size_t ret_objsz = sizeof({ret_type});
struct shm_bm{{
    void * shm;
    shm_bm_objid_t objid;
}};
SHM_BM_INTERFACE_CREATE({name}_return, ret_objsz, ret_nobjs);
"#,
        name = function.name,
        ret_type = function.return_type
    )
}
fn argshm_interface_create(function: &IDLFunction) -> String {
    format!(
        r#"static int args_nobjs = 16;
struct {name}_args{{
    {fields}
}};
static size_t args_objsz = sizeof(struct {name}_args);
SHM_BM_INTERFACE_CREATE({name}_params, args_objsz, args_nobjs);
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
fn reg_function_definitions(function: &IDLFunction) -> String {
    format!("{type} {name}_s({args});
{type} {name}_c({args});",
                name= function.name,
                type=function.return_type,
                args = function
                .arguments
                .iter()
                .map(|(arg_type, arg_name)| format!("{} {}", arg_type, arg_name))
                .collect::<Vec<String>>()
                .join(","),)
}
fn argshm_function_definitions(function: &IDLFunction) -> String {
    format!("{type} {name}_s(void * shm, shm_bm_objid_t objid);
{type} {name}_c({args});",
                name= function.name,
                type=function.return_type,
                args = function
                .arguments
                .iter()
                .map(|(arg_type, arg_name)| format!("{} {}", arg_type, arg_name))
                .collect::<Vec<String>>()
                .join(","),)
}
fn argshm_retshm_function_definitions(function: &IDLFunction) -> String {
    format!("struct shm_bm *{name}_s(void * shm, shm_bm_objid_t objid);
{type} {name}_c({args});",
                name= function.name,
                type=function.return_type,
                args = function
                .arguments
                .iter()
                .map(|(arg_type, arg_name)| format!("{} {}", arg_type, arg_name))
                .collect::<Vec<String>>()
                .join(","),)
}
fn retshm_function_definitions(function: &IDLFunction) -> String {
    format!("struct shm_bm *{name}_s({args});
{type} {name}_c({args});",
                name= function.name,
                type=function.return_type,
                args = function
                .arguments
                .iter()
                .map(|(arg_type, arg_name)| format!("{} {}", arg_type, arg_name))
                .collect::<Vec<String>>()
                .join(","),)
}
fn retshm_client_signature(function: &IDLFunction) -> String {
    format!(
        "{ret_type} {name}_c({args}){{
    struct shm_bm *ret_shm = {name}_s({args2});",
        ret_type = function.return_type,
        name = function.name,
        args = function
            .arguments
            .iter()
            .map(|(arg_type, arg_name)| format!("{} {}", arg_type, arg_name))
            .collect::<Vec<String>>()
            .join(","),
        args2 = function
            .arguments
            .iter()
            .map(|(_arg_type, arg_name)| format!("{}", arg_name))
            .collect::<Vec<String>>()
            .join(","),
    )
}

fn reg_client_signature(function: &IDLFunction) -> String {
    format!(
        "{ret_type} {name}_c({args}){{",
        ret_type = function.return_type,
        name = function.name,
        args = function
            .arguments
            .iter()
            .map(|(arg_type, arg_name)| format!("{} {}", arg_type, arg_name))
            .collect::<Vec<String>>()
            .join(","),
    )
}
fn argshm_client_body(function: &IDLFunction) -> String {
    format!(
        r#"
    struct {name}_args params=(struct {name}_args){{{args}}};
    //TODO: ensure proper allignment
    void * mem = calloc(args_nobjs,args_objsz);
    size_t memsz = args_nobjs*args_objsz;
    void * shm = shm_bm_create_{name}_params(mem,memsz);
    shm_bm_init_{name}_params(shm);
    shm_bm_objid_t objid;
    void * obj;
    obj = shm_bm_alloc_{name}_params(shm,&objid);
    *(struct {name}_args*)obj = params;"#,
        name = function.name,
        args = function
            .arguments
            .iter()
            .map(|(_arg_type, arg_name)| format!("{}", arg_name))
            .collect::<Vec<String>>()
            .join(","),
    )
}
fn argshm_retshm_interface(function: &IDLFunction) -> String {
    format!(
        r#"
    struct shm_bm * ret_shm = {name}_s(shm,objid);"#,
        name = function.name,
    )
}
fn retshm_client_return(function: &IDLFunction) -> String {
    format!(
        "
    return *({ret_type}*)shm_bm_take_{name}_return(ret_shm->shm,ret_shm->objid);
}}
",
        name = function.name,
        ret_type = function.return_type,
    )
}
fn reg_client_return(function: &IDLFunction) -> String {
    format!(
        "
    return {name}_s({args});
}}",
        name = function.name,
        args = function
            .arguments
            .iter()
            .map(|(_arg_type, arg_name)| format!("{}", arg_name))
            .collect::<Vec<String>>()
            .join(","),
    )
}
fn argshm_client_return(function: &IDLFunction) -> String {
    format!(
        "
    return {name}_s(shm,objid);
}}",
        name = function.name,
    )
}
fn reg_server_signature(function: &IDLFunction) -> String {
    format!(
        r#"{} {}_s({}){{"#,
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
fn argshm_server_signature(function: &IDLFunction) -> String {
    format!(
        "{} {}_s(void * shm, shm_bm_objid_t objid){{",
        function.return_type, function.name
    )
}
fn retshm_server_signature(function: &IDLFunction) -> String {
    format!(
        "struct shm_bm * {name}_s({args}){{
    {ret_type} ret = {name}({args2});",
        name = function.name,
        args = function
            .arguments
            .iter()
            .map(|(arg_type, arg_name)| format!("{} {}", arg_type, arg_name))
            .collect::<Vec<String>>()
            .join(","),
        args2 = function
            .arguments
            .iter()
            .map(|(_arg_type, arg_name)| format!("{}", arg_name))
            .collect::<Vec<String>>()
            .join(","),
        ret_type = function.return_type,
    )
}
fn retshm_argshm_server_signature(function: &IDLFunction) -> String {
    format!(
        "struct shm_bm * {}_s(void * shm, shm_bm_objid_t objid){{",
        function.name
    )
}

fn argshm_server_body(function: &IDLFunction) -> String {
    format!(
        r#"
    struct {name}_args params = *(struct {name}_args*)shm_bm_take_{name}_params(shm,objid);
    shm_bm_free_{name}_params(shm);
    {ret_type} ret = {name}({args});"#,
        name = function.name,
        args = function
            .arguments
            .iter()
            .map(|(_arg_type, arg_name)| format!("params.{}", arg_name))
            .collect::<Vec<String>>()
            .join(","),
        ret_type = function.return_type,
    )
}
fn retshm_server_return(function: &IDLFunction) -> String {
    format!(
        r#"
    void * mem = calloc(sizeof({ret_type}),ret_nobjs);
    size_t memsz = ret_nobjs*ret_objsz;
    void * ret_shm = shm_bm_create_{name}_return(mem,memsz);
    shm_bm_init_{name}_return(ret_shm);
    shm_bm_objid_t ret_objid;
    void * obj;
    obj = shm_bm_alloc_{name}_return(ret_shm,&ret_objid);
    *({ret_type}*)obj = ret;
    return &((struct shm_bm ){{ret_shm,ret_objid}});
}}"#,
        name = function.name,
        ret_type = function.return_type,
    )
}
fn reg_server_return(function: &IDLFunction) -> String {
    format!(
        "
    return {name}({args});
}}",
        name = function.name,
        args = function
            .arguments
            .iter()
            .map(|(_arg_type, arg_name)| format!("{}", arg_name))
            .collect::<Vec<String>>()
            .join(","),
    )
}
fn argshm_server_return(_function: &IDLFunction) -> String {
    format!(
        "
    return ret;
}}"
    )
}

fn main() {
    let file_content = read_toml("file.toml").expect("Failed to read file");

    let config: SystemConfig = toml::from_str(&file_content).expect("Failed to parse TOML");

    let mut client_stub = String::new();
    let mut server_stub = String::new();
    let mut shared_header = String::new();

    // Moved logic from generate_stubs into main
    client_stub += &format!("#include \"shared.h\" \n");
    server_stub += &format!("#include \"shared.h\" \n#include \"server.h\"\n");
    shared_header += &format!("#ifndef SHARED_H\n#define SHARED_H\n#include \"shm_bm.h\"\n");

    //getSize() is not working so for now, if returns greater than limit, it must use shm, if not, it uses regular arguments
    for function in config.functions {
        if get_size(&function) < LIMIT {
            //only shm for return
            if get_size(&function) > LIMIT {
                shared_header += &retshm_interface_create(&function);
                shared_header += &retshm_function_definitions(&function);
                server_stub += &retshm_server_signature(&function);
                server_stub += &retshm_server_return(&function);
                client_stub += &retshm_client_signature(&function);
                client_stub += &retshm_client_return(&function);
            } else {
                //no shm used
                shared_header += &reg_function_definitions(&function);
                server_stub += &reg_server_signature(&function);
                server_stub += &reg_server_return(&function);
                client_stub += &reg_client_signature(&function);
                client_stub += &reg_client_return(&function);
            }
        } else {
            //shm for both return and args
            if get_size(&function) > LIMIT {
                shared_header += &retshm_interface_create(&function);
                shared_header += &argshm_interface_create(&function);
                shared_header += &argshm_retshm_function_definitions(&function);
                server_stub += &retshm_argshm_server_signature(&function);
                server_stub += &argshm_server_body(&function);
                server_stub += &retshm_server_return(&function);
                client_stub += &reg_client_signature(&function);
                client_stub += &argshm_client_body(&function);
                client_stub += &argshm_retshm_interface(&function);
                client_stub += &retshm_client_return(&function);
            } else {
                //shm for args only
                shared_header += &argshm_function_definitions(&function);
                shared_header += &argshm_interface_create(&function);
                server_stub += &argshm_server_signature(&function);
                server_stub += &argshm_server_body(&function);
                server_stub += &argshm_server_return(&function);
                client_stub += &reg_client_signature(&function);
                client_stub += &argshm_client_body(&function);
                client_stub += &argshm_client_return(&function);
            }
        }
    }
    shared_header += &format!("\n#endif");
    println!("{}", shared_header);

    match fs::write("client_stubs.c", &client_stub) {
        Ok(_) => println!("C code written to file: {}", "/src/stubs.h"),
        Err(error) => println!("Error writing to file: {}", error),
    }
    match fs::write("server_stubs.c", &server_stub) {
        Ok(_) => println!("C code written to file: {}", "/src/stubs.h"),
        Err(error) => println!("Error writing to file: {}", error),
    }
    match fs::write("shared.h", &shared_header) {
        Ok(_) => println!("C code written to file: {}", "/src/stubs.h"),
        Err(error) => println!("Error writing to file: {}", error),
    }
}
