use shtairir::parser::parse_script;

#[test]
fn test_hello_world_script() {
    let script_content = r#"
// Hello World example in Shtairir
bevy:create_entity()
bevy:add_component("entity1", "Position")
bevy:set_component("entity1", "Position", {x=100, y=200})
redis:set("greeting", "Hello, World!")
redis:set("count", 42)
"#;
