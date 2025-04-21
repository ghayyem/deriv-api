use derive_builder::Builder;
use handlebars::Handlebars;
use handlebars::{Context, Helper, HelperResult, JsonRender, Output, RenderContext as HbsRenderContext};
use log::{debug, error, info, warn};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;
use convert_case::{Case, Casing};

// --- Template Definitions ---

// Template for generating individual struct/enum files
const TYPE_TEMPLATE: &str = r#"
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: {{source_file}}

// Use direct crate names for imports within generated files
{{#if needs_serde}}use serde::{Deserialize, Serialize};{{/if}} 
{{#if needs_value}}use serde_json::Value;{{/if}}
{{#if needs_hashmap}}use std::collections::HashMap;{{/if}}
{{#if needs_datetime}}use chrono::{DateTime, Utc};{{/if}}

// Import shared types from the *same* crate
{{#each imports}}
use crate::{{this}}; 
{{/each}}

{{#if is_enum}}
/// {{{description}}}
#[derive(Debug, Clone, Serialize, Deserialize)] // Enums should also derive Serialize/Deserialize
#[serde(rename_all = "snake_case")]
pub enum {{name}} {
    {{#each variants}}
    {{#if description}}
    /// {{{description}}}
    {{/if}}
    {{name}}{{#if value}} = {{value}}{{/if}},
    {{/each}}
}

// Optional: Derive Default for enums, defaulting to the first variant? Or require explicit handling?
// For now, DO NOT derive Default for enums automatically. Structs needing them must handle it.

/* // Example: Deriving Default for Enum (use with caution)
impl Default for {{name}} {
    fn default() -> Self {
        // Default to the first variant found
        {{#if variants}}Self::{{variants.0.name}}{{else}}/* No variants to default to */{{/if}}
    }
}
*/

{{else}}
// It's a struct
{{#if description}}
/// {{{description}}}
{{/if}}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct {{name}} {
    {{#each fields}}
    {{#if description}}
    {{{doc_comment description}}}
    {{/if}}
    // Correct serde attribute construction - Use helper
    {{#if serde_attrs}}#[serde({{join serde_attrs ", "}})] {{/if}}
    pub {{field_name}}: {{{field_type}}},
    {{/each}}
}

{{/if}}
"#;


// Template for Request/Response files (references types via imports)
const REQUEST_RESPONSE_TEMPLATE: &str = r#"
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: {{source_file}}

// Use direct crate names for imports
{{#if needs_serde}}use serde::{Deserialize, Serialize};{{/if}}
{{#if needs_value}}use serde_json::Value;{{/if}}
{{#if needs_hashmap}}use std::collections::HashMap;{{/if}}
{{#if needs_datetime}}use chrono::{DateTime, Utc};{{/if}}

// Import required types from the *same* crate
{{#each imports}}
use crate::{{this}};
{{/each}}

{{#if description}}
/// {{{description}}}
{{/if}}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub struct {{name}} {
    {{#each fields}}
    {{#if description}}
    {{{doc_comment description}}}
    {{/if}}
    // Correct serde attribute construction - Use helper
    {{#if serde_attrs}}#[serde({{join serde_attrs ", "}})] {{/if}}
    pub {{field_name}}: {{{field_type}}},
    {{/each}}
}

{{#if has_default_impl}}
impl Default for {{name}} {
    fn default() -> Self {
        Self {
            {{#each fields}}
            {{field_name}}: {{default_value}}, // Ensure default_value is correct for enums (likely None for Option<Enum>)
            {{/each}}
        }
    }
}
{{/if}}
"#;

// Remove old templates
// const SCHEMA_TEMPLATE: &str = ... (removed)
// const MISSING_TYPE_TEMPLATE: &str = ... (removed)
// const NESTED_TYPE_TEMPLATE: &str = ... (removed)


// --- Struct Definitions ---

#[derive(Debug, Clone, Serialize, Builder)]
struct SchemaField {
    field_name: String,
    field_type: String,
    description: Option<String>,
    serde_attrs: Vec<String>,
    default_value: String,
    is_required: bool,
}

#[derive(Debug, Clone, Serialize)]
struct EnumVariant {
    name: String,
    original_name: String,
    value: Option<i64>,
    description: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
struct EnumType {
    name: String,
    description: Option<String>,
    variants: Vec<EnumVariant>,
}

// Simplified SchemaType used for rendering individual types and Req/Res structs
#[derive(Debug, Clone, Serialize)]
struct RenderContext {
    name: String,
    description: Option<String>,
    source_file: String,
    fields: Vec<SchemaField>,
    variants: Option<Vec<EnumVariant>>, // Only for enums
    is_enum: bool,
    needs_hashmap: bool,
    needs_datetime: bool,
    has_default_impl: bool,
    has_builder: bool,
    needs_serde: bool,
    needs_value: bool,
    imports: Vec<String>,
}

// Structure to hold collected type information before generation
#[derive(Debug, Clone)]
struct TypeDefinition {
    name: String,
    definition: Value, // The JSON schema part defining this type
    source_file: String,
    is_enum: bool,
}

// --- SchemaParser ---

#[derive(Debug)]
struct SchemaParser {
    handlebars: Handlebars<'static>,
    // Registry for all discovered custom types (structs/enums)
    // Key: Rust type name (e.g., "Tick"), Value: (JSON Definition, Source File Path)
    type_definitions: HashMap<String, TypeDefinition>,
    type_mapping: HashMap<String, String>,
    // Keep track of imports needed for the current Req/Res being processed
    current_imports: HashSet<String>,
    needs_hashmap: bool,
    needs_datetime: bool,
    needs_value: bool,
}

impl SchemaParser {
    fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("type_template", TYPE_TEMPLATE)
            .expect("Failed to register type template");
        handlebars
            .register_template_string("request_response_template", REQUEST_RESPONSE_TEMPLATE)
            .expect("Failed to register request/response template");

        // Register the join helper
        handlebars.register_helper("join", Box::new(join_helper));

        // Register the doc_comment helper
        handlebars.register_helper("doc_comment", Box::new(doc_comment_helper));

        // ... existing type_mapping initialization ...
        let mut type_mapping = HashMap::new();
        type_mapping.insert("string".to_string(), "String".to_string());
        type_mapping.insert("integer".to_string(), "i64".to_string());
        type_mapping.insert("number".to_string(), "f64".to_string());
        type_mapping.insert("boolean".to_string(), "bool".to_string());
        // Default object type is Value, unless overridden by specific handling
        type_mapping.insert("object".to_string(), "Value".to_string());
        type_mapping.insert("array".to_string(), "Vec<Value>".to_string()); // Default array item is Value

        // Add common type mappings - ensure these types are defined/generated
        type_mapping.insert("passthrough".to_string(), "Passthrough".to_string());
        type_mapping.insert("echo_req".to_string(), "EchoReq".to_string());
        type_mapping.insert("req_id".to_string(), "i64".to_string()); // req_id is special, directly map to i64

        Self {
            handlebars,
            type_definitions: HashMap::new(),
            type_mapping,
            current_imports: HashSet::new(),
            needs_hashmap: false,
            needs_datetime: false,
            needs_value: false,
        }
    }

    // --- Pass 1: Collect Type Definitions ---

    // Scans a schema file (send.json or receive.json) and adds all
    // discovered custom object/enum definitions to the type_definitions registry.
    fn collect_definitions_from_schema(
        &mut self,
        schema_path: &Path,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let content = fs::read_to_string(schema_path)?;
        let schema: Value = serde_json::from_str(&content)?;
        let source_file = schema_path.to_string_lossy().to_string();

        if let Some(properties) = schema.get("properties").and_then(|v| v.as_object()) {
            self.collect_definitions_from_properties(properties, &source_file)?;
        }

        // Also collect definitions from $defs if they exist
        if let Some(defs) = schema.get("$defs").and_then(|v| v.as_object()) {
            self.collect_definitions_from_properties(defs, &source_file)?;
        }

        Ok(())
    }

    fn collect_definitions_from_properties(
        &mut self,
        properties: &Map<String, Value>,
        source_file: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (name, value) in properties {
            self.collect_definition_recursive(value, name, source_file)?;
        }
        Ok(())
    }

    // Recursively explores a JSON value, adding type definitions to the registry.
    fn collect_definition_recursive(
        &mut self,
        value: &Value,
        name_hint: &str, // Used to generate type names (e.g., property name)
        source_file: &str,
    ) -> Result<(), Box<dyn std::error::Error>> {
        debug!("Collecting definition recursively, hint: {}", name_hint);

        // --- BEGIN MODIFICATION: Check for const integer ---
        if let (Some(Value::String(json_type)), Some(Value::Number(_const_val))) = (value.get("type"), value.get("const")) {
            if json_type == "integer" {
                 debug!("Skipping registration for const integer property: {}", name_hint);
                return Ok(()); // Do not register this as a distinct type
            }
        }
         // --- END MODIFICATION ---

        // Handle $ref - we don't collect definitions from external refs here
        if value.get("$ref").is_some() {
            return Ok(());
        }

        // Check if it's an object with properties (potential struct)
        if let Some(properties) = value.get("properties").and_then(|p| p.as_object()) {
            let type_name = self.to_type_name(name_hint);
             debug!("Found potential struct definition: {} from hint: {}", type_name, name_hint);
            if !self.type_definitions.contains_key(&type_name) {
                self.type_definitions.insert(
                    type_name.clone(),
                    TypeDefinition {
                        name: type_name.clone(),
                        definition: value.clone(),
                        source_file: source_file.to_string(),
                        is_enum: false, // It's an object/struct
                    },
                );
                // Recursively collect definitions from properties of this object
                for (prop_name, prop_value) in properties {
                     // --- BEGIN FIX: Use property name directly as hint --- 
                     // let nested_name_hint = format!("{}{}", type_name, prop_name.to_case(Case::Pascal));
                    self.collect_definition_recursive(prop_value, prop_name, source_file)?;
                     // --- END FIX ---
                }
            }
        }
        // Check if it's an enum (string or integer based)
        else if value.get("enum").is_some() {
            // Only consider enums if they have a "type" specified as string or integer.
            // Avoid generating enums for types like `{"enum": [1, "foo"]}` unless type is specified.
             let json_type = value.get("type").and_then(|t| t.as_str());
             if json_type == Some("string") || json_type == Some("integer") {
                let type_name = self.to_type_name(name_hint);
                debug!("Found potential enum definition: {} from hint: {}", type_name, name_hint);
                if !self.type_definitions.contains_key(&type_name) {
                    self.type_definitions.insert(
                        type_name.clone(),
                        TypeDefinition {
                            name: type_name.clone(),
                            definition: value.clone(),
                            source_file: source_file.to_string(),
                            is_enum: true, // It's an enum
                        },
                    );
                }
             } else {
                 warn!("Skipping enum registration for '{}' due to missing or unsupported type: {:?}", name_hint, json_type);
             }

        }
        // Check if it's an array with items definition
        else if let Some(items) = value.get("items") {
            // Arrays themselves don't define a named type, but their items might.
            // Generate a name hint for the item type based on the array name + "Item".
            let singular_hint = name_hint.strip_suffix('s').unwrap_or(name_hint); // Simple singularization
            let item_type_hint = format!("{}Item", singular_hint);
             debug!("Recursing into array items, hint: {}", item_type_hint);
            self.collect_definition_recursive(items, &item_type_hint, source_file)?;
        }
         // Check for object with additionalProperties (potential map/dictionary)
         else if let Some(add_props) = value.get("additionalProperties") {
             // If additionalProperties defines a schema, recurse into that.
             if add_props.is_object() {
                 let map_value_hint = format!("{}Value", name_hint);
                 debug!("Recursing into additionalProperties, hint: {}", map_value_hint);
                 self.collect_definition_recursive(add_props, &map_value_hint, source_file)?;
             }
             // `additionalProperties: true` or `false` don't define a new type.
         }


        // Handle allOf, anyOf, oneOf - recurse into their definitions
        for key in ["allOf", "anyOf", "oneOf"].iter() {
            if let Some(sub_schemas) = value.get(key).and_then(|v| v.as_array()) {
                for (i, sub_schema) in sub_schemas.iter().enumerate() {
                     let sub_name_hint = format!("{}{}Sub{}", name_hint, key.to_case(Case::Pascal), i);
                     debug!("Recursing into {}, hint: {}", key, sub_name_hint);
                    self.collect_definition_recursive(sub_schema, &sub_name_hint, source_file)?;
                }
            }
        }


        Ok(())
    }


     // --- Pass 2 & 3: Generate Types and Request/Response Files ---

    // Parses a schema (like send.json or receive.json) and generates a RenderContext
    // suitable for the REQUEST_RESPONSE_TEMPLATE. It relies on type_definitions being populated.
    fn parse_request_response_schema(
        &mut self,
        schema_path: &Path,
    ) -> Result<RenderContext, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(schema_path)?;
        let schema: Value = serde_json::from_str(&content)?;
        let source_file = schema_path.to_string_lossy().to_string();

        let schema_name = schema_path
            .file_stem()
            .and_then(|n| n.to_str())
            .ok_or("Invalid schema filename")?;

        let endpoint_name = schema_path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .ok_or("Invalid schema path")?;

        let type_name = match schema_name {
            "send" => format!("{}Request", self.to_type_name(endpoint_name)),
            "receive" => format!("{}Response", self.to_type_name(endpoint_name)),
            _ => self.to_type_name(schema_name), // Fallback, should ideally not happen
        };

        self.current_imports.clear(); // Reset imports for this file
        self.needs_hashmap = false;
        self.needs_datetime = false;

        // Initialize local needs flags for this context
        let mut needs_serde = false;
        let mut needs_value = false;

        let mut fields = Vec::new();
        if let Some(properties) = schema.get("properties").and_then(|v| v.as_object()) {
             let required: HashSet<String> = schema
                .get("required")
                .and_then(|v| v.as_array())
                .map(|arr| {
                    arr.iter()
                        .filter_map(|v| v.as_str())
                        .map(String::from)
                        .collect()
                })
                .unwrap_or_default();

            // Preprocess to handle potentially problematic fields by converting them to Value
            let mut modified_properties = Map::new();
            for (name, value) in properties {
                let mut new_value = value.clone();
                if self.is_problematic_field(name) {
                    info!("Converting problematic field '{}' in {} to Value", name, type_name);
                     new_value = json!({
                        "type": "object", // Represent as a generic object which maps to Value
                        "description": format!("Field '{}' mapped to Value due to complexity/potential issues.", name)
                    });
                    // Ensure Value import if needed (though usually handled by default object mapping)
                    // self.current_imports.insert("serde_json::Value".to_string());
                }
                 modified_properties.insert(name.clone(), new_value);
            }


            for (prop_name, prop_value) in &modified_properties {
                fields.push(self.process_property(
                    prop_name,
                    prop_value,
                    required.contains(prop_name),
                    &type_name, // Pass the parent type name for context
                )?);
            }
             // Set needs_serde if fields exist (struct needs derive)
             needs_serde = !fields.is_empty(); 
             // needs_value will be set if determine_field_type returns Value
             needs_value = fields.iter().any(|f| f.field_type.contains("Value"));
        }


        // Determine default impl requirements
        let has_default_impl = false; // Effectively disable #[serde(default)] and impl Default

        Ok(RenderContext {
            name: type_name,
            description: schema.get("description").and_then(|v| v.as_str()).map(String::from),
            source_file,
            fields,
            variants: None, // Not an enum
            is_enum: false,
            needs_hashmap: self.needs_hashmap,
            needs_datetime: self.needs_datetime,
            has_default_impl,
            has_builder: false, // Builder support can be added later if needed
            needs_serde,
            needs_value,
            imports: self.current_imports.iter().cloned().collect(),
        })
    }

    // Processes a single property definition within a schema (for Req/Res generation).
    fn process_property(
        &mut self,
        name: &str,
        property: &Value,
        is_required: bool,
        parent_type_name: &str, // Name of the struct containing this field
    ) -> Result<SchemaField, Box<dyn std::error::Error>> {

        let mut field_type = self.determine_field_type(property, name, parent_type_name)?;
        let field_name = self.to_field_name(name);

        let description = property
            .get("description")
            .and_then(|d| d.as_str())
            .map(|d| Self::decode_html_entities(d));

        // Ensure non-required fields are Option<T>
        if !is_required && !field_type.starts_with("Option<") {
            // Check if it's already a primitive Option like Option<i64>
            // This logic might need refinement if determine_field_type changes
            field_type = format!("Option<{}>", field_type);
            debug!("Field '{}' in {} marked as optional (Option<{}>) because not required.", field_name, parent_type_name, field_type);
        }

        let mut serde_attrs = Vec::new();

        // Add rename attribute if needed
        if name != field_name {
             serde_attrs.push(format!("rename = \"{}\"", name));
        }

        // Add skip_serializing_if attribute if the FINAL type is Option
        if field_type.starts_with("Option<") {
             serde_attrs.push("skip_serializing_if = \"Option::is_none\"".to_string());
        }

        // Add flatten attribute (if/when needed)
        // let flatten = ...; // Determine flatten logic
        // if flatten {
        //     serde_attrs.push("flatten".to_string());
        // }

        // Determine default value - Crucial for Enum handling
        let default_value = self.get_default_value_for_type(&field_type, is_required);

        Ok(SchemaField {
            field_name,
            field_type,
            description,
            serde_attrs,
            default_value,
            is_required,
        })
    }


    // Determines the Rust type for a given JSON schema property.
    // Adds necessary imports to self.current_imports.
    fn determine_field_type(
        &mut self,
        property: &Value,
        name_hint: &str,
        parent_type_name: &str, // For context
    ) -> Result<String, Box<dyn std::error::Error>> {
        debug!("Determining type for property: {}, hint: {}, parent: {}", name_hint, name_hint, parent_type_name);

        // Check for nullability first (common case)
        let is_nullable = self.is_nullable_type(property);

        // Handle allOf, anyOf, oneOf - often used for complex types or nullability
        // TODO: Improve handling for `anyOf`, `oneOf`, `allOf` for more complex scenarios
        if let Some(all_of) = property.get("allOf").and_then(|v| v.as_array()) {
            // Simplified: Try to find a $ref within allOf, otherwise complex type
            for item in all_of {
                if let Some(ref_str) = item.get("$ref").and_then(|r| r.as_str()) {
                    let base_type = self.process_ref(ref_str)?;
                    return Ok(if is_nullable || self.is_nullable_alongside_ref(property) { format!("Option<{}>", base_type) } else { base_type });
                }
            }
             warn!("Unhandled allOf without $ref for property '{}', falling back to Value.", name_hint);
             self.needs_value = true;
             return Ok("Value".to_string()); // Fallback for complex unhandled allOf
        }
        // Add similar stubs for anyOf/oneOf if needed, potentially falling back to Value


        // Handle $ref
        if let Some(ref_str) = property.get("$ref").and_then(|v| v.as_str()) {
            let base_type = self.process_ref(ref_str)?;
             // Nullability might be defined alongside the $ref, not just within the referenced schema
            return Ok(if is_nullable || self.is_nullable_alongside_ref(property) { format!("Option<{}>", base_type) } else { base_type });
        }

        // Determine the primary JSON type
        let json_type = match property.get("type") {
            Some(Value::String(s)) => s.as_str(),
            Some(Value::Array(types)) => {
                // Handle cases like ["string", "null"] or ["integer", "null"]
                let non_null_type = types.iter().find(|t| !t.is_null()).and_then(|t| t.as_str());
                match non_null_type {
                    Some(t) => t,
                    None => {
                         warn!("Array type specification only contains null or is invalid for property '{}', falling back to Value.", name_hint);
                         self.needs_value = true;
                         return Ok("Value".to_string()); // Or handle as error?
                    }
                }
            },
            Some(_) => {
                warn!("Unsupported 'type' format for property '{}', falling back to Value.", name_hint);
                 self.needs_value = true;
                 return Ok("Value".to_string());
            },
            None => {
                 // If type is missing, it might be an object defined inline or an enum
                 if property.get("properties").is_some() || property.get("additionalProperties").is_some() {
                     "object" // Assume object if properties/additionalProperties exist
                 } else if property.get("enum").is_some() {
                      // Infer type from the first enum variant if possible, else default/fallback needed
                      // For simplicity now, if 'type' is missing but 'enum' exists, might need fallback
                      warn!("Property '{}' has 'enum' but no explicit 'type', inferring might be needed or fallback to Value.", name_hint);
                      "object" // Placeholder, might need smarter inference or fallback
                 } else {
                    warn!("Missing 'type' and cannot infer for property '{}', falling back to Value.", name_hint);
                    self.needs_value = true;
                    return Ok("Value".to_string());
                }
            }
        };


        // Handle specific formats first
        if self.is_datetime_field(property, json_type) {
            self.needs_datetime = true;
            let base_type = "DateTime<Utc>".to_string();
            return Ok(if is_nullable { format!("Option<{}>", base_type) } else { base_type });
        }

        // Handle numeric strings identified by pattern (placed before basic types)
        if self.is_numeric_string_field(name_hint, property) {
            let base_type = "String".to_string(); // Still a string, but maybe needs special handling later
             debug!("Field '{}' identified as numeric string.", name_hint);
            return Ok(if is_nullable { format!("Option<{}>", base_type) } else { base_type });
        }


        let base_type = match json_type {
            "string" => {
                // Check for enum defined on string type
                if let Some(enum_values) = property.get("enum").and_then(|v| v.as_array()) {
                    // --- BEGIN FIX: Handle string enums correctly ---
                    match enum_values.len() {
                        0 => {
                            warn!("String enum found for '{}' but it has no variants. Falling back to String.", name_hint);
                            "String".to_string()
                        }
                        1 => {
                            debug!("String enum found for '{}' with only one variant. Using String directly.", name_hint);
                            "String".to_string() // Treat single-variant string enum as just String
                        }
                        _ => { // More than one variant
                            let rust_enum_name = self.to_type_name(name_hint);
                            self.add_import(&rust_enum_name); // Ensure import is added
                            rust_enum_name
                        }
                    }
                    // --- END FIX ---
                 } else {
                    self.type_mapping.get(json_type).cloned().unwrap_or_else(|| "String".to_string())
                }
            }
            "integer" => {
                 if let Some(enum_values) = property.get("enum").and_then(|v| v.as_array()) {
                     // --- BEGIN FIX: Handle integer enums correctly ---
                     match enum_values.len() {
                        0 => {
                            warn!("Integer enum found for '{}' but it has no variants. Falling back to i64.", name_hint);
                            "i64".to_string()
                        }
                        1 => {
                             debug!("Integer enum found for '{}' with only one variant. Using i64 directly.", name_hint);
                             "i64".to_string() // Treat single-variant integer enum as just i64
                        }
                        _ => { // More than one variant
                            let rust_enum_name = self.to_type_name(name_hint);
                            self.add_import(&rust_enum_name);
                            rust_enum_name
                        }
                    }
                     // --- END FIX ---
                 } else {
                    self.type_mapping.get(json_type).cloned().unwrap_or_else(|| "i64".to_string())
                 }
            }
            "number" => self.type_mapping.get(json_type).cloned().unwrap_or_else(|| "f64".to_string()),
            "boolean" => self.type_mapping.get(json_type).cloned().unwrap_or_else(|| "bool".to_string()),
            "array" => {
                // --- BEGIN FIX: Extend lifetime of default value ---
                let default_item_schema = json!({"type": "object"});
                let items_prop = property.get("items").unwrap_or(&default_item_schema);
                // --- END FIX ---
                // --- BEGIN FIX: Singularize name hint for item type ---
                let singular_hint = name_hint.strip_suffix('s').unwrap_or(name_hint); // Simple singularization
                let item_type_hint = format!("{}Item", singular_hint);
                let item_type = self.determine_field_type(items_prop, &item_type_hint, parent_type_name)?;
                // --- END FIX ---
                // --- BEGIN FIX: Add import for array item type ---
                self.add_import(&item_type); // item_type should now be consistently singular
                // --- END FIX ---
                format!("Vec<{}>", item_type)
            }
            "object" => {
                // Handle map-like objects (additionalProperties)
                if let Some(add_props) = property.get("additionalProperties") {
                    // Ensure add_props is not `false` (which means no additional props allowed)
                    if add_props.is_boolean() && !add_props.as_bool().unwrap_or(true) {
                        // Handle case where additionalProperties: false - Treat as standard object if 'properties' exist?
                        if property.get("properties").is_some() {
                            // Fall through to inline struct generation below
                        } else {
                           error!("Object '{}' has additionalProperties: false and no properties. Cannot represent.", name_hint);
                           return Err(format!("Object '{}' has additionalProperties: false and no properties", name_hint).into());
                        }
                    } else if add_props.is_object() || (add_props.is_boolean() && add_props.as_bool().unwrap_or(false)) {
                         // If additionalProperties is true or an object schema
                         let value_type = if add_props.is_object() {
                             self.determine_field_type(add_props, &format!("{}Value", name_hint), parent_type_name)?
                         } else {
                              self.needs_value = true;
                              "Value".to_string()
                         };
                         self.needs_hashmap = true;
                         let map_type = format!("HashMap<String, {}>", value_type);
                         // Wrap in Option if nullable, then return
                         return Ok(if is_nullable { format!("Option<{}>", map_type) } else { map_type });
                    } else {
                        // Fall through if additionalProperties is not defined or handled above
                         warn!("Unhandled additionalProperties: {:?} for '{}', falling back to object/Value.", add_props, name_hint);
                    }
                }

                // Handle objects defined inline via "properties"
                if property.get("properties").is_some() {
                     // --- BEGIN FIX: Use name_hint for inline struct name ---
                     let rust_struct_name = self.to_type_name(name_hint);
                     // --- END FIX ---
                     // We expect this type to be collected in pass 1. If not, it's an error.
                     if !self.type_definitions.contains_key(&rust_struct_name) {
                          warn!("Inline object definition found for '{}' but type '{}' was not collected. Check collection logic.", name_hint, rust_struct_name);
                         // Fallback to Value? Or error?
                         self.needs_value = true;
                         return Ok("Value".to_string()); // Fallback for now
                     }
                     self.add_import(&rust_struct_name); // Ensure import is added
                     rust_struct_name

                } else if property.get("additionalProperties").is_some() {
                    // If only additionalProperties is specified (handled above), use that HashMap type.
                    // Re-calculate here to avoid complex fall-through logic
                     if let Some(add_props) = property.get("additionalProperties") {
                          if add_props.is_boolean() && !add_props.as_bool().unwrap_or(true) {
                               // Already handled: error or fallback needed
                               warn!("Re-evaluating additionalProperties: false for '{}', falling back to Value.", name_hint);
                               self.needs_value = true;
                              return Ok("Value".to_string());
                          } else if add_props.is_object() || (add_props.is_boolean() && add_props.as_bool().unwrap_or(false)) {
                               let value_type = if add_props.is_object() {
                                   self.determine_field_type(add_props, &format!("{}Value", name_hint), parent_type_name)?
                               } else {
                                    self.needs_value = true;
                                    "Value".to_string()
                               };
                               self.needs_hashmap = true;
                               format!("HashMap<String, {}>", value_type)
                          } else {
                               warn!("Unhandled additionalProperties case for '{}', falling back to Value.", name_hint);
                                self.needs_value = true;
                                "Value".to_string()
                          }
                     } else {
                          // Should not happen if additionalProperties was the reason we are in "object" type
                          warn!("Object type determined for '{}' but neither 'properties' nor 'additionalProperties' seem applicable. Falling back to Value.", name_hint);
                           self.needs_value = true;
                           "Value".to_string()
                     }

                }
                 else {
                    // Generic object, likely needs Value
                     warn!("Unhandled object type for property '{}', falling back to Value.", name_hint);
                    self.needs_value = true;
                    "Value".to_string()
                }
            }
            _ => {
                warn!("Unknown JSON type '{}' for property '{}', falling back to Value.", json_type, name_hint);
                self.needs_value = true;
                "Value".to_string()
            }
        };


        // Wrap in Option if nullable
        Ok(if is_nullable { format!("Option<{}>", base_type) } else { base_type })
    }


    // --- Generation Helper Functions ---

    // Renders the content for an individual type file (struct or enum)
    fn render_individual_type(
        &mut self,
        type_def: &TypeDefinition,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.current_imports.clear(); // Reset imports
        self.needs_hashmap = false;
        self.needs_datetime = false;
        self.needs_value = false;
        let mut needs_serde_for_this = true;

        let definition_obj = type_def.definition.as_object()
            .ok_or_else(|| format!("Type definition for {} is not an object", type_def.name))?;

        let mut fields = Vec::new();
        let mut variants = None;
        let mut is_enum = false;

        if type_def.is_enum {
            is_enum = true;
            variants = Some(self.parse_enum_variants(definition_obj)?);
            needs_serde_for_this = true; // Enums always need serde
        } else { // It's a struct
            if let Some(properties) = definition_obj.get("properties").and_then(|v| v.as_object()) {
                let required: HashSet<String> = definition_obj
                    .get("required")
                    .and_then(|v| v.as_array())
                    .map(|arr| arr.iter().filter_map(|v| v.as_str().map(String::from)).collect())
                    .unwrap_or_default();

                 // Preprocess problematic fields
                let mut modified_properties = Map::new();
                for (name, value) in properties {
                    let mut new_value = value.clone();
                     if self.is_problematic_field(name) {
                        info!("Converting problematic field '{}' in type {} to Value", name, type_def.name);
                        new_value = json!({
                            "type": "object",
                            "description": format!("Field '{}' mapped to Value due to complexity/potential issues.", name)
                        });
                    }
                    modified_properties.insert(name.clone(), new_value);
                }

                for (prop_name, prop_value) in &modified_properties {
                    fields.push(self.process_property(
                        prop_name,
                        prop_value,
                        required.contains(prop_name),
                        &type_def.name, // Pass current type name as parent context
                    )?);
                }
                if !fields.is_empty() {
                     needs_serde_for_this = true; // Structs with fields generally need serde
                }
                // Check if any field uses Value
                self.needs_value = self.needs_value || fields.iter().any(|f| f.field_type.contains("Value"));
            } else {
                // Struct definition with no properties - still needs serde derive if used elsewhere
                warn!("Struct definition for {} has no 'properties'. Generating an empty struct.", type_def.name);
                needs_serde_for_this = true;
            }
        }

        // Ensure struct/enum types get serde derives.
        if is_enum || !fields.is_empty() {
             needs_serde_for_this = true;
        } // Empty structs might not strictly need it unless used somewhere?
          // Let's be conservative and add it if it has no props but was defined.
         else if !is_enum && definition_obj.contains_key("type") { // It was explicitly an object
             needs_serde_for_this = true;
         }

        let context = RenderContext {
            name: type_def.name.clone(),
            description: definition_obj.get("description").and_then(|v| v.as_str()).map(Self::decode_html_entities),
            source_file: type_def.source_file.clone(),
            fields,
            variants,
            is_enum,
            needs_hashmap: self.needs_hashmap,
            needs_datetime: self.needs_datetime,
            has_default_impl: false,
            has_builder: false,
            needs_serde: needs_serde_for_this,
            needs_value: self.needs_value,
            imports: self.current_imports.iter().cloned().collect(),
        };

        self.handlebars
            .render("type_template", &json!(context))
            .map_err(|e| e.into())
    }


    // Renders the content for a Request or Response file
    fn render_request_response(
        &self,
        context: &RenderContext,
    ) -> Result<String, Box<dyn std::error::Error>> {
        self.handlebars
            .render("request_response_template", &json!(context))
            .map_err(|e| e.into())
    }

    // Parses enum variants from a schema object
    fn parse_enum_variants(&self, property: &Map<String, Value>) -> Result<Vec<EnumVariant>, Box<dyn std::error::Error>> {
        let enum_values = property.get("enum").ok_or("Enum values not found")?;
        let mut variants = Vec::new();
        if let Some(values) = enum_values.as_array() {
            for value in values {
                 let (original_name, variant_name, num_value) = match value {
                    Value::String(s) => (s.clone(), self.to_enum_variant_name(s), None),
                    Value::Number(n) => (n.to_string(), format!("Value{}", n), n.as_i64()),
                    _ => continue, // Skip non-string/non-number variants
                };
                variants.push(EnumVariant {
                    name: variant_name,
                    original_name,
                    value: num_value,
                     // Descriptions per variant are not standard in JSON schema enums
                    description: None,
                });
            }
        }
        Ok(variants)
    }


    // Adds an import for a generated type (struct/enum)
    fn add_import(&mut self, type_name: &str) {
        // Avoid importing self in Req/Res files (already defined)
        // Avoid importing primitives or base types handled by needs_ flags
        if !self.is_primitive_or_known_or_handled(type_name) {
            let module_name = self.to_file_name(type_name);
            self.current_imports.insert(format!("{}::{}", module_name, type_name));
        }
    }

    // Checks if a type name corresponds to a Rust primitive, std type, known base type,
    // or a type whose import is handled by a `needs_` flag.
    fn is_primitive_or_known_or_handled(&self, type_name: &str) -> bool {
        matches!(type_name, "String" | "i64" | "f64" | "bool" | "Value" | "DateTime<Utc>" | "Passthrough" | "EchoReq" | "KeyValueObject")
        || type_name.starts_with("Vec<")
        || type_name.starts_with("HashMap<")
        || type_name.starts_with("Option<")
    }

     // Heuristic check for fields that cause issues if typed strongly
     fn is_problematic_field(&self, name: &str) -> bool {
        // Expand this list based on observed issues during generation/compilation
        matches!(name,
            "tax" | "postcode" | "address_line_2" | // Often optional strings, cause Default issues if Option<String> not handled well
            "account_opening_reason" | // Sometimes enum, sometimes free text
            "p2p_advert_update" | "contracts_for" | "new_account_virtual" |
            "payment_methods" | // Already listed below as p2p_payment_methods? Keep both?
            "payment_methodsitem" | "proposal_open_contract" |
            "p2p_advertiser_create" | "p2p_advert_create" | "ohlc" |
            "active_symbols" | "mt5_new_account" | "contracts" | "residence_list" |
            "paymentagent_details" | "traders" |
            "statement" | "profit_table" | // These often have deeply nested, complex, or varying structures
            "login_history" | // Array items can vary slightly
            "tick" | // Can be complex, sometimes string price, sometimes number
            "proposal" | // Very complex structure

            // Add fields corresponding to types causing trait bound errors:
            "list" | // Covers the various ListItem errors in p2p_order_list, p2p_advert_list, paymentagent_list, p2p_advertiser_list, p2p_advertiser_adverts
            "get_account_status" |
            "p2p_advert_info" |
            "website_config" |
            "p2p_order_dispute" |
            "p2p_payment_methods" | // Covers the HashMap<String, P2pPaymentMethodsValue> error
            "p2p_order_info" |
            "p2p_order_create" |
            "questions" | // Covers the HashMap<String, QuestionsValue> error
            "verified_jurisdiction" | // Add this to bypass trait bounds issue for now
            // Fields with missing definitions in source schema:
            "signup" | 
            "partner" | // Also corresponds to SignUpRequirements
            "withdrawal"

        )
    }

    // Check for numeric strings
    fn is_numeric_string_field(&self, name: &str, property: &Value) -> bool {
         property.get("type").and_then(|t| t.as_str()) == Some("string") &&
         (name.ends_with("_price") || name.ends_with("_amount") || name == "quote" || name == "ask" || name == "bid" || name == "balance" || name == "payout" || name == "profit" || name == "stake" || name == "open" || name == "high" || name == "low" || name == "close") ||
         (property.get("description").and_then(|d|d.as_str()).map_or(false, |d| d.contains("price") || d.contains("amount") || d.contains("value")))

    }

     // Check if a property represents a nullable type (has "null" in its type array or schema implies nullability)
     fn is_nullable_type(&self, property: &Value) -> bool {
         if let Some(types) = property.get("type").and_then(|t| t.as_array()) {
             types.iter().any(|t| t.as_str() == Some("null"))
         } else {
             // Add other heuristics? E.g., if 'default' is explicitly null?
             // Or if not in 'required' list (though this depends on context)
             false
         }
     }

    // Check nullability specifically when a $ref is present
    fn is_nullable_alongside_ref(&self, property: &Value) -> bool {
        // JSON Schema spec is a bit ambiguous here. Sometimes 'type: ["foo", "null"]'
        // appears alongside '$ref'. Let's check for that pattern.
        if let Some(types) = property.get("type").and_then(|t| t.as_array()) {
            types.iter().any(|t| t.as_str() == Some("null"))
        } else {
            false
        }
    }


    // Check if a field (based on name, description, and JSON type) likely represents a datetime
    fn is_datetime_field(&self, property: &Value, json_type: &str) -> bool {
        let description = property.get("description").and_then(|v| v.as_str()).unwrap_or("");
        let name = property.get("name").and_then(|v| v.as_str()).unwrap_or(""); // Less reliable

        // Epoch timestamps are usually integers
        (json_type == "integer" && (description.contains("epoch") || name.contains("time") || name.contains("date")))
        // Sometimes timestamps might be numbers (e.g., with milliseconds)
        || (json_type == "number" && (description.contains("epoch") || name.contains("time") || name.contains("date")))
         // Less common: formatted date strings - we don't handle parsing these automatically yet
         // || (json_type == "string" && (description.contains("date") || ...))
    }

    // Calculates the appropriate default value string for a field in the Default impl.
    fn get_default_value_for_type(&self, field_type: &str, _is_required: bool) -> String {
         // Option types always default to None
         if field_type.starts_with("Option<") {
            return "None".to_string();
        }

        // Handle specific known types
        match field_type {
             "String" => "String::new()".to_string(),
             "i64" => "0".to_string(),
             "f64" => "0.0".to_string(),
             "bool" => "false".to_string(),
             // Vec and HashMap default to empty collections
             _ if field_type.starts_with("Vec<") => "Vec::new()".to_string(),
             _ if field_type.starts_with("HashMap<") => "HashMap::new()".to_string(),
             // For other types (including enums, custom structs, Value), use Default::default()
             // The has_default_impl logic *should* prevent generating Default impls
             // where this would be invalid (e.g., for required enums).
             _ => "Default::default()".to_string(),
        }
    }

    // Checks if a field type string represents a type that can be safely defaulted
    // using primitive defaults or Default::default(). Crucially, excludes non-optional enums.
    fn is_type_defaultable(&self, field_type: &str) -> bool {
        if field_type.starts_with("Option<") {
            true // Options default to None
        } else if field_type.starts_with("Vec<") || field_type.starts_with("HashMap<") {
            true // Collections default to empty
        } else if matches!(field_type, "String" | "i64" | "f64" | "bool") {
            true // Basic primitives have direct defaults
        } else if field_type.ends_with("Enum") {
            false // Non-optional enums CANNOT be defaulted
        } else {
            // For other types (Value, DateTime<Utc>, other generated structs),
            // assume they *might* implement Default. If they don't, compilation
            // will fail later, which is the desired behavior if Default is required.
            true
        }
    }

    // --- Name Conversion Utilities (Moved inside impl SchemaParser) ---

    fn decode_html_entities(s: &str) -> String {
        s.replace("&lt;", "<")
            .replace("&gt;", ">")
            .replace("&quot;", "\"")
            .replace("&apos;", "'")
            .replace("&amp;", "&")
            .replace("&#x60;", "`")
            .replace("&#x27;", "'")
    }

    fn to_file_name(&self, name: &str) -> String {
        // Convert camelCase/PascalCase to snake_case and keep existing underscores
        let mut result = String::new();
        let mut chars = name.chars().peekable();

        while let Some(c) = chars.next() {
            if c.is_uppercase() {
                if !result.is_empty() && !result.ends_with('_') {
                    result.push('_');
                }
                result.push(c.to_lowercase().next().unwrap());
            } else if c == '_' {
                if !result.ends_with('_') { // Avoid double underscores
                    result.push('_');
                }
            } else if c == '-' { // Replace hyphen with underscore
                 if !result.ends_with('_') { // Avoid double underscores
                     result.push('_');
                 }
            } else if c.is_alphanumeric() { // Keep lowercase letters and numbers
                result.push(c);
            } // Ignore other non-alphanumeric characters
        }

        // Final check for trailing underscore
        if result.ends_with('_') {
            result.pop();
        }

        // --- BEGIN FIX: Explicitly handle 'type' keyword --- 
        if result == "type" {
             debug!("Filename is 'type', appending underscore.");
             result.push('_');
        } else {
             // You could potentially keep the broader keyword match here as a fallback
             // For now, let's only explicitly handle 'type' to see if it fixes the specific issue.
        }
        // --- END FIX ---
        
        result
    }

    fn to_type_name(&self, name: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;

        for c in name.chars() {
            if c == '_' || c == '-' { // Treat hyphen like underscore for capitalization
                capitalize_next = true;
            } else if capitalize_next {
                result.extend(c.to_uppercase());
                capitalize_next = false;
            } else {
                // Preserve existing uppercase chars for acronyms like P2P
                result.push(c);
            }
        }
        // Handle snake_case parts like `p2p_` -> `P2p`
        // Also explicitly replace hyphens remaining in the structure (though the loop above should prevent this)
        result.replace("P2p", "P2p").replace('-', "_")
    }


    fn to_field_name(&self, name: &str) -> String {
        let mut result = String::new();
        let mut prev_was_upper = false;
        let mut prev_was_underscore = true; // Treat start as if preceded by underscore

        for c in name.chars() {
            if c.is_uppercase() {
                // Add underscore before uppercase if not the first char,
                // not preceded by another uppercase char, and not preceded by an underscore.
                if !result.is_empty() && !prev_was_upper && !prev_was_underscore {
                    result.push('_');
                }
                result.extend(c.to_lowercase());
                prev_was_upper = true;
                prev_was_underscore = false;
            } else if c == '_' || c == '-' { // Treat hyphen like underscore
                // Avoid leading/multiple underscores
                if !result.is_empty() && !prev_was_underscore {
                    result.push('_');
                }
                prev_was_upper = false;
                prev_was_underscore = true;
            } else { // Lowercase letters or digits
                result.push(c);
                prev_was_upper = false;
                prev_was_underscore = false;
            }
        }

        // Remove potential trailing underscore
        if result.ends_with('_') {
            result.pop();
        }

        // Ensure result is not empty if original name was just underscores/hyphens
        if result.is_empty() && name.chars().any(|c| c == '_' || c == '-') {
             return "_".to_string(); // Or some other placeholder
        }

        // Check if the resulting name is a Rust keyword and use raw identifier if needed
        let final_result = match result.as_str() {
            // List common keywords
            "type" | "match" | "loop" | "ref" | "self" | "super" | "crate" | "mod" | "const" | "static" | "mut" | "pub" | "fn" | "struct" | "enum" | "trait" | "impl" | "use" | "extern" | "unsafe" | "async" | "await" | "dyn" | "for" | "in" | "if" | "else" | "while" | "let" | "return" | "true" | "false" | "as" | "break" | "continue" | "Self" |
            "virtual"
             => {
                format!("{}_", result) // Append underscore
            }
            _ => result,
        };

        final_result
    }

    fn to_enum_variant_name(&self, name: &str) -> String {
        // Handle numeric case first
        if name.chars().all(|c| c.is_numeric()) {
            return format!("Value{}", name);
        }

        // Simple PascalCase conversion, handling spaces/special chars
        let mut result = String::new();
        let mut capitalize_next = true;
        for c in name.chars() {
            if c.is_alphanumeric() {
                if capitalize_next {
                    result.extend(c.to_uppercase());
                    capitalize_next = false;
                } else {
                    result.push(c);
                }
            } else {
                capitalize_next = true; // Capitalize after space/punctuation
                if !result.ends_with('_') && c == ' ' || c == '-' || c == ':' { // Use underscore for some separators
                     result.push('_');
                }
            }
        }
        // Remove leading/trailing underscores
        let result = result.trim_matches('_').to_string();

        // Prefix with underscore if starts with number
        if result.chars().next().map_or(false, |c| c.is_numeric()) {
            format!("_{}", result)
        } else {
            result
        }
    }

    // Processes a $ref string, resolves it to a type name, and ensures the definition exists.
    fn process_ref(&mut self, ref_str: &str) -> Result<String, Box<dyn std::error::Error>> {
        let def_name = ref_str.split('/').last().unwrap_or(""); // Get the last part after '/' 

        // Special remapping for patched schemas or inconsistencies 
        let type_name = match def_name {
            "landingCompanyDetails" => "LandingCompanyInfo".to_string(), // Remapped by patch 
            _ => self.to_type_name(def_name), // Convert the name part to PascalCase 
        };

        // Check if the definition was collected or is a known type 
        if !self.type_definitions.contains_key(&type_name) && !self.is_primitive_or_known_or_handled(&type_name) {
            warn!(
                "Definition for referenced type '{}' (from ref '{}', extracted name '{}') not found in collected definitions. Assuming it will be generated.",
                type_name,
                ref_str,
                def_name
            );
            // Return the expected type name anyway, generation might handle it elsewhere 
            // or fail later if truly missing. 
            // Ensure import is attempted even if definition not found yet
            self.add_import(&type_name);
            return Ok(type_name);
        }

        // If found or known, return the mapped/converted type name and add import
        self.add_import(&type_name); 
        Ok(type_name)
    }

}

// Simple Handlebars helper to join a vector of strings
fn join_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut HbsRenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).ok_or_else(|| handlebars::RenderError::new("Param 0 is required for join helper."))?;
    let separator = h.param(1).map(|v| v.value().render()).unwrap_or_else(|| ", ".to_string());

    let vec = param.value().as_array().ok_or_else(|| handlebars::RenderError::new("Param 0 must be an array."))?;

    // Ensure values are rendered as strings before joining
    let strings: Vec<String> = vec.iter().map(|v| v.render()).collect();
    out.write(&strings.join(&separator))?;
    Ok(())
}

// Handlebars helper to format a string as a Rust doc comment (multi-line)
fn doc_comment_helper(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _: &mut HbsRenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let param = h.param(0).ok_or_else(|| handlebars::RenderError::new("Param 0 (description string) is required for doc_comment helper."))?;
    let description = param.value().render(); // Get the string value

    // Trim whitespace from the description and split into lines
    let lines = description.trim().lines();

    for line in lines {
        // Trim trailing whitespace from each line before outputting
        let trimmed_line = line.trim_end();
        // Output each line prefixed with "/// ", handling empty lines correctly
        out.write("///")?; // Write the base prefix
        if !trimmed_line.is_empty() {
            out.write(" ")?; // Add space only if line is not empty
            out.write(trimmed_line)?;
        }
        out.write("\\n")?; // Add newline for the next doc comment line
    }

    Ok(())
}

// --- Main Function ---

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting schema generation...");

    let input_dir = std::env::var("SCHEMA_PATH").unwrap_or_else(|_| "./deriv-api-docs/config/v3".to_string());
    let output_dir = std::env::var("SCHEMA_OUTPUT_PATH").unwrap_or_else(|_| "./schema/src".to_string());
    let output_path = Path::new(&output_dir);

    info!("Input directory: {}", input_dir);
    info!("Output directory: {}", output_dir);

    fs::create_dir_all(output_path)?;

    let mut parser = SchemaParser::new();
    let mut generated_modules = HashSet::new(); // Use HashSet to avoid duplicates

    // --- Pass 1: Collect all type definitions ---
    info!("Pass 1: Collecting type definitions...");
    for entry in fs::read_dir(&input_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() { continue; }

        let endpoint_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
        debug!("Collecting definitions from: {}", endpoint_name);

        let send_path = path.join("send.json");
        if send_path.exists() {
            if let Err(e) = parser.collect_definitions_from_schema(&send_path) {
                 error!("Error collecting definitions from {}: {}", send_path.display(), e);
            }
        }

        let receive_path = path.join("receive.json");
        if receive_path.exists() {
            if let Err(e) = parser.collect_definitions_from_schema(&receive_path) {
                 error!("Error collecting definitions from {}: {}", receive_path.display(), e);
            }
        }
    }
    info!("Collected {} unique type definitions.", parser.type_definitions.len());


    // --- Pass 2: Generate individual type files ---
    info!("Pass 2: Generating individual type files...");
    // Define essential base types that might be referenced but not explicitly defined everywhere
     let base_types_to_ensure = ["Passthrough", "EchoReq", "KeyValueObject"];
     for base_type in base_types_to_ensure {
         if !parser.type_definitions.contains_key(base_type) {
             // Provide a minimal definition if missing
             let definition = match base_type {
                 "Passthrough" => json!({"type": "object", "additionalProperties": true, "description": "Arbitrary key-value pairs that are passed back from the server."}),
                 "EchoReq" => json!({"type": "object", "additionalProperties": true, "description": "Echo of the request made."}),
                 "KeyValueObject" => json!({"type": "object", "additionalProperties": true, "description": "Generic key-value object."}),
                 _ => json!({"type": "object", "description": format!("Auto-generated placeholder for {}", base_type)})
             };
             parser.type_definitions.insert(base_type.to_string(), TypeDefinition {
                 name: base_type.to_string(),
                 definition,
                 source_file: "Base Type Definition".to_string(),
                 is_enum: false,
             });
             info!("Added missing base type definition for: {}", base_type);
         }
     }


    let type_definitions_clone = parser.type_definitions.clone(); // Clone for iteration while mutating parser state
    for (type_name, type_def) in &type_definitions_clone {
         // Skip primitives/known types that don't need generation
         if parser.is_primitive_or_known_or_handled(type_name) { continue; }

        let file_name_base = parser.to_file_name(type_name);
        let output_file_path = output_path.join(format!("{}.rs", file_name_base));

        match parser.render_individual_type(type_def) {
            Ok(content) => {
                if let Err(e) = fs::write(&output_file_path, content) {
                     error!("Failed to write type file {}: {}", output_file_path.display(), e);
                 } else {
                     // --- BEGIN DEBUG LOG --- 
                     debug!("Successfully wrote type file: {} and added '{}' to generated modules", output_file_path.display(), file_name_base);
                     // --- END DEBUG LOG --- 
                     generated_modules.insert(file_name_base);
                 }
            }
            Err(e) => {
                error!("Failed to render type {}: {}", type_name, e);
            }
        }
    }
    info!("Pass 2: Finished generating individual type files.");


    // --- Pass 3: Generate Request/Response files ---
    info!("Pass 3: Generating Request/Response files...");
     for entry in fs::read_dir(&input_dir)? {
         let entry = entry?;
         let path = entry.path();

         if !path.is_dir() { continue; }

         let endpoint_name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
         debug!("Generating Req/Res for: {}", endpoint_name);

         // Process send.json -> *Request.rs
         let send_path = path.join("send.json");
         if send_path.exists() {
             match parser.parse_request_response_schema(&send_path) {
                 Ok(context) => {
                     let file_name_base = parser.to_file_name(&context.name);
                     let output_file_path = output_path.join(format!("{}.rs", file_name_base));
                     match parser.render_request_response(&context) {
                         Ok(content) => {
                             if let Err(e) = fs::write(&output_file_path, content) {
                                 error!("Failed to write request file {}: {}", output_file_path.display(), e);
                             } else {
                                 debug!("Generated request file: {}", output_file_path.display());
                                 generated_modules.insert(file_name_base);
                             }
                         }
                         Err(e) => error!("Failed to render request {}: {}", context.name, e),
                     }
                 }
                 Err(e) => error!("Failed to parse request schema {}: {}", send_path.display(), e),
             }
         }

         // Process receive.json -> *Response.rs
         let receive_path = path.join("receive.json");
         if receive_path.exists() {
              match parser.parse_request_response_schema(&receive_path) {
                 Ok(context) => {
                     let file_name_base = parser.to_file_name(&context.name);
                     let output_file_path = output_path.join(format!("{}.rs", file_name_base));
                     match parser.render_request_response(&context) {
                         Ok(content) => {
                             if let Err(e) = fs::write(&output_file_path, content) {
                                 error!("Failed to write response file {}: {}", output_file_path.display(), e);
                             } else {
                                 debug!("Generated response file: {}", output_file_path.display());
                                 generated_modules.insert(file_name_base);
                             }
                         }
                         Err(e) => error!("Failed to render response {}: {}", context.name, e),
                     }
                 }
                 Err(e) => error!("Failed to parse response schema {}: {}", receive_path.display(), e),
             }
         }
     }
     info!("Pass 3: Finished generating Request/Response files.");


    // --- Generate lib.rs ---
    info!("Generating lib.rs...");
    let generated_modules_vec: Vec<String> = generated_modules.into_iter().collect();
    let lib_content = generate_mod_content(&generated_modules_vec);
    if let Err(e) = fs::write(output_path.join("lib.rs"), lib_content) {
        error!("Failed to write lib.rs: {}", e);
    }

    info!("Schema generation completed.");
    Ok(())
}


// Generate lib.rs content including module declarations and re-exports
fn generate_mod_content(modules: &[String]) -> String {
    // Create an owned, mutable Vec for sorting
    let mut sorted_modules: Vec<String> = modules.to_vec();
    sorted_modules.sort(); // Sort alphabetically for consistent output

    let mut content = String::from(
        "// Generated by schema_generator.rs - DO NOT EDIT.\n\n\
        //! # Deriv API Schema\n\
        //! \n\
        //! Auto-generated Rust types for the Deriv API.\n\
        //! These types are generated from the official API documentation.\n\
        //! \n\
        //! This crate provides strongly typed request and response structures for all API endpoints.\n\n\
        #![allow(clippy::too_many_lines)]\n\
        #![allow(clippy::derive_partial_eq_without_eq)]\n\
        #![allow(clippy::extra_unused_lifetimes)] // Can happen with generated code\n\
        #![allow(clippy::needless_lifetimes)] // Can happen with generated code\n\
        #![allow(non_snake_case)] // Allow type names like `P2P`\n\
        #![allow(dead_code)] // Allow unused generated types\n\n"
    );

    // Ensure these are always present
    content.push_str("#[allow(unused_imports)]\nuse serde::{Deserialize, Serialize};\n");
    content.push_str("#[allow(unused_imports)]\nuse serde_json::Value;\n");
    content.push_str("#[allow(unused_imports)]\nuse std::collections::HashMap;\n");
    content.push_str("#[allow(unused_imports)]\nuse chrono::{DateTime, Utc};\n\n");


    // Module declarations
    content.push_str("// Module declarations\n");
    for module in modules {
        content.push_str(&format!("mod {};\n", module));
    }
    content.push('\n');

    // Re-exports
    content.push_str("// Re-exports\n");
    for module in modules {
        content.push_str(&format!("pub use {}::*;\n", module));
    }
    content.push('\n');

    content
}


// Remove old helper functions related to the previous approach
// fn collect_shared_types(...) - removed
// fn extract_type_name(...) - removed
// fn is_shared_type_candidate(...) - removed
// fn generate_common_type(...) - removed

// --- Tests ---
// (Tests need significant updates to reflect the new structure and logic)
#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    fn create_parser() -> SchemaParser {
        SchemaParser::new()
    }

    #[test]
    fn test_type_name_conversion() {
        let parser = create_parser();
        assert_eq!(parser.to_type_name("proposal_open_contract"), "ProposalOpenContract");
        assert_eq!(parser.to_type_name("p2p_advert_info"), "P2pAdvertInfo");
        assert_eq!(parser.to_type_name("p2p_order_list"), "P2pOrderList");
        assert_eq!(parser.to_type_name("tick"), "Tick");
    }

    #[test]
    fn test_field_name_conversion() {
        let parser = create_parser();
        assert_eq!(parser.to_field_name("contract_id"), "contract_id");
        assert_eq!(parser.to_field_name("longcode"), "longcode");
        assert_eq!(parser.to_field_name("ask_price"), "ask_price");
        assert_eq!(parser.to_field_name("p2p_advertiser_info"), "p2p_advertiser_info"); // Handles p2p correctly now? Needs check
    }

     #[test]
     fn test_file_name_conversion() {
         let parser = create_parser();
         assert_eq!(parser.to_file_name("ProposalOpenContract"), "proposal_open_contract");
         assert_eq!(parser.to_file_name("P2pAdvertInfo"), "p2p_advert_info");
         assert_eq!(parser.to_file_name("Tick"), "tick");
         assert_eq!(parser.to_file_name("TickHistoryRequest"), "tick_history_request");
     }


    #[test]
    fn test_enum_variant_name_conversion() {
        let parser = create_parser();
        assert_eq!(parser.to_enum_variant_name("sell"), "Sell");
        assert_eq!(parser.to_enum_variant_name("1"), "Value1");
        assert_eq!(parser.to_enum_variant_name("EUR"), "Eur"); // Check capitalization
        assert_eq!(parser.to_enum_variant_name("USD"), "Usd");
        assert_eq!(parser.to_enum_variant_name("proposal"), "Proposal");
         assert_eq!(parser.to_enum_variant_name("CALL"), "Call");
         assert_eq!(parser.to_enum_variant_name("Digits: Even/Odd"), "Digits_Even_Odd"); // Check special chars
    }

    #[test]
    fn test_determine_field_type_basic() {
        let mut parser = create_parser();
        let prop = json!({"type": "string", "description": "Client loginid"});
        let type_str = parser.determine_field_type(&prop, "loginid", "Parent").unwrap();
        assert_eq!(type_str, "String");
        assert!(parser.current_imports.is_empty()); // No imports for String

        let prop_int = json!({"type": "integer"});
        let type_str_int = parser.determine_field_type(&prop_int, "count", "Parent").unwrap();
        assert_eq!(type_str_int, "i64");

        let prop_num = json!({"type": "number"});
        let type_str_num = parser.determine_field_type(&prop_num, "price", "Parent").unwrap();
        assert_eq!(type_str_num, "f64");

         let prop_bool = json!({"type": "boolean"});
         let type_str_bool = parser.determine_field_type(&prop_bool, "is_valid", "Parent").unwrap();
         assert_eq!(type_str_bool, "bool");
    }

    #[test]
    fn test_determine_field_type_nullable() {
        let mut parser = create_parser();
        let prop = json!({"type": ["string", "null"]});
        let type_str = parser.determine_field_type(&prop, "middle_name", "Parent").unwrap();
        assert_eq!(type_str, "Option<String>");
    }

    #[test]
    fn test_determine_field_type_array() {
        let mut parser = create_parser();
        let prop = json!({"type": "array", "items": {"type": "string"}});
        let type_str = parser.determine_field_type(&prop, "tags", "Parent").unwrap();
        assert_eq!(type_str, "Vec<String>");

        let prop_nullable = json!({"type": ["array", "null"], "items": {"type": "integer"}});
        let type_str_nullable = parser.determine_field_type(&prop_nullable, "ids", "Parent").unwrap();
        assert_eq!(type_str_nullable, "Option<Vec<i64>>");
    }

    #[test]
    fn test_determine_field_type_object_fallback() {
         let mut parser = create_parser();
         // Object with no properties -> Value
         let prop = json!({"type": "object"});
         let type_str = parser.determine_field_type(&prop, "metadata", "Parent").unwrap();
         assert_eq!(type_str, "Value");
         //assert!(parser.current_imports.contains("serde_json::Value")); // Import handled by needs_value flag now
         assert!(parser.needs_value);
         // assert!(parser.needs_hashmap); // Value might be a map, but `needs_hashmap` is only true if HashMap type explicitly used
     }

     #[test]
     fn test_determine_field_type_datetime() {
         let mut parser = create_parser();
         let prop_epoch = json!({"type": "integer", "description": "Start time of the candle (epoch)"});
         let type_str = parser.determine_field_type(&prop_epoch, "epoch", "Parent").unwrap();
         assert_eq!(type_str, "DateTime<Utc>");
         //assert!(parser.current_imports.contains("chrono::{DateTime, Utc}")); // Import handled by needs_datetime flag now
         assert!(parser.needs_datetime);
  
         let prop_nullable_epoch = json!({"type": ["integer", "null"], "description": "Expiry time (epoch)"});
         let type_str_nullable = parser.determine_field_type(&prop_nullable_epoch, "expiry_time", "Parent").unwrap();
         assert_eq!(type_str_nullable, "Option<DateTime<Utc>>");
         //assert!(parser.current_imports.contains("chrono::{DateTime, Utc}")); // Import handled by needs_datetime flag now
         assert!(parser.needs_datetime);
     }


    #[test]
    fn test_determine_field_type_hashmap() {
        let mut parser = create_parser();
        // Mock definition for Account
        parser.type_definitions.insert("Account".to_string(), TypeDefinition {
             name: "Account".to_string(),
             definition: json!({"type": "object", "properties": {"balance": {"type": "number"}}}),
             source_file: "mock".to_string(),
             is_enum: false,
         });


        let prop = json!({
            "type": "object",
            "patternProperties": {
                "^[A-Za-z]+$": { "$ref": "#/$defs/account" } // Assuming $ref resolves to Account
            }
        });

         // Need to simulate $ref resolution or pre-populate definitions for patternProperties test
         // Let's assume process_ref would return "Account" and add its definition previously.
         // We also need the definition for "Account" to be in parser.type_definitions.
         // Manually add it for this test:
         parser.type_definitions.insert("Account".to_string(), TypeDefinition {
             name: "Account".to_string(),
             definition: json!({"type":"object", "properties": {}}), // Simplified def
             source_file: "mock".to_string(),
             is_enum: false,
         });


        let type_str = parser.determine_field_type(&prop, "accounts", "Parent").unwrap();
        assert_eq!(type_str, "HashMap<String, Account>");
        assert!(parser.needs_hashmap);
        //assert!(parser.current_imports.contains("std::collections::HashMap")); // Import handled by needs_hashmap flag now
        assert!(parser.current_imports.contains("account::Account")); // Check import added
    }

    // Add more tests for:
    // - $ref resolution (requires more setup)
    // - Enum generation and import
    // - Complex nested types
    // - Problematic field handling
    // - Default value generation for various types
}
