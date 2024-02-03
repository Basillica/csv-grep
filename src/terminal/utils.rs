use std::collections::HashMap;


// Function to dynamically create a struct based on CSV headers
pub fn create_struct(headers: &[String]) {
    // Define a HashMap to store the field names and types
    let mut field_types: HashMap<String, String> = HashMap::new();

    // For simplicity, assuming all fields are of type String
    for header in headers {
        field_types.insert(header.clone(), "String".to_string());
    }

    // Generate the struct definition
    let struct_definition = format!(
        "struct DynamicStruct {{\n{}\n}}",
        field_types
            .iter()
            .map(|(name, field_type)| format!("    {}: {},", name, field_type))
            .collect::<Vec<String>>()
            .join("\n")
    );

    // Print the dynamically generated struct definition
    println!("{}", struct_definition);
}


// pub fn init_error_hooks() -> color_eyre::Result<()> {
//     let (panic, error) = HookBuilder::default().into_hooks();
//     let panic = panic.into_panic_hook();
//     let error = error.into_eyre_hook();
//     color_eyre::eyre::set_hook(Box::new(move |e| {
//         let _ = restore_terminal();
//         error(e)
//     }))?;
//     std::panic::set_hook(Box::new(move |info| {
//         let _ = restore_terminal();
//         panic(info)
//     }));
//     Ok(())
// }

// pub fn init_terminal() -> color_eyre::Result<Terminal<impl Backend>> {
//     enable_raw_mode()?;
//     stdout().execute(EnterAlternateScreen)?;
//     stdout().execute(EnableMouseCapture)?;
//     let backend = CrosstermBackend::new(stdout());
//     let terminal = Terminal::new(backend)?;
//     Ok(terminal)
// }

// pub fn restore_terminal() -> color_eyre::Result<()> {
//     disable_raw_mode()?;
//     stdout().execute(LeaveAlternateScreen)?;
//     Ok(())
// }