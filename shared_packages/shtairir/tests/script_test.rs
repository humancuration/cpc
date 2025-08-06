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

    let script = parse_script(script_content).expect("Failed to parse script");
    
    assert_eq!(script.commands.len(), 5);
    
    // Check first command
    let cmd1 = &script.commands[0];
    assert_eq!(cmd1.app, "bevy");
    assert_eq!(cmd1.function, "create_entity");
    assert_eq!(cmd1.args.len(), 0);
    
    // Check second command
    let cmd2 = &script.commands[1];
    assert_eq!(cmd2.app, "bevy");
    assert_eq!(cmd2.function, "add_component");
    assert_eq!(cmd2.args.len(), 2);
    
    // Check third command
    let cmd3 = &script.commands[2];
    assert_eq!(cmd3.app, "bevy");
    assert_eq!(cmd3.function, "set_component");
    assert_eq!(cmd3.args.len(), 3);
    
    // Check fourth command
    let cmd4 = &script.commands[3];
    assert_eq!(cmd4.app, "redis");
    assert_eq!(cmd4.function, "set");
    assert_eq!(cmd4.args.len(), 2);
    
    // Check fifth command
    let cmd5 = &script.commands[4];
    assert_eq!(cmd5.app, "redis");
    assert_eq!(cmd5.function, "set");
    assert_eq!(cmd5.args.len(), 2);
}

#[test]
fn test_complex_script() {
    let script_content = r#"
// Complex workflow
ffmpeg:convert("input.mp4", "output.webm")
ffmpeg:extract_audio("input.mp4", "audio.opus")
bevy:create_entity()
bevy:add_component("camera", "Camera")
bevy:add_component("light", "Light")
redis:set("video:input", "input.mp4")
redis:set("video:output", "output.webm")
"#;

    let script = parse_script(script_content).expect("Failed to parse script");
    
    assert_eq!(script.commands.len(), 7);
    
    // Check that we have commands from different apps
    let apps: Vec<&str> = script.commands.iter().map(|c| c.app.as_str()).collect();
    assert!(apps.contains(&"ffmpeg"));
    assert!(apps.contains(&"bevy"));
    assert!(apps.contains(&"redis"));
}