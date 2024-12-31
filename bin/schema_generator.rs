use derive_builder::Builder;
use handlebars::Handlebars;
use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use serde_json::{json, Map, Value};
use std::collections::{HashMap, HashSet};
use std::fs;
use std::path::Path;

const SCHEMA_TEMPLATE: &str = r#"
// Generated automatically by schema_generator.rs - DO NOT EDIT.
// Source: {{source_file}}

use serde::{Deserialize, Serialize};
use serde_json::Value;
{{#if needs_hashmap}}use std::collections::HashMap;{{/if}}
{{#if needs_datetime}}use chrono::{DateTime, Utc};{{/if}}

{{#if description}}
/// {{{description}}}
{{/if}}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
{{#if default_impl}}#[serde(default)]{{/if}}
pub struct {{name}} {
    {{#each fields}}
    {{#if description}}
    /// {{{description}}}
    {{/if}}
    #[serde({{~#if rename}}rename = "{{rename}}"{{/if}}{{~#if skip_serializing_if}}, skip_serializing_if = "Option::is_none"{{/if}}{{~#if flatten}}, flatten{{/if}})]
    pub {{field_name}}: {{{field_type}}},
    {{/each}}
}

{{#if has_default_impl}}
impl Default for {{name}} {
    fn default() -> Self {
        Self {
            {{#each fields}}
            {{field_name}}: {{default_value}},
            {{/each}}
        }
    }
}
{{/if}}

{{#if has_builder}}
impl {{name}} {
    /// Creates a new builder for {{name}}
    pub fn builder() -> {{name}}Builder {
        {{name}}Builder::default()
    }
}
{{/if}}

{{#each sub_types}}
{{{this}}}
{{/each}}

{{#each enums}}
/// {{{description}}}
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum {{name}} {
    {{#each variants}}
    {{#if description}}
    /// {{{description}}}
    {{/if}}
    {{name}}{{#if value}} = {{value}}{{/if}},
    {{/each}}
}

impl {{name}} {
    pub fn as_str(&self) -> &'static str {
        match self {
            {{#each variants}}
            Self::{{name}} => "{{original_name}}",
            {{/each}}
        }
    }

    pub fn from_str(s: &str) -> Option<Self> {
        match s {
            {{#each variants}}
            "{{original_name}}" => Some(Self::{{name}}),
            {{/each}}
            _ => None,
        }
    }
}
{{/each}}
"#;

#[derive(Debug, Clone, Serialize, Builder)]
struct SchemaField {
    field_name: String,
    field_type: String,
    description: Option<String>,
    rename: Option<String>,
    skip_serializing_if: bool,
    flatten: bool,
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

#[derive(Debug, Clone, Serialize)]
struct SchemaType {
    name: String,
    description: Option<String>,
    source_file: String,
    fields: Vec<SchemaField>,
    sub_types: Vec<String>,
    enums: Vec<EnumType>,
    needs_hashmap: bool,
    needs_datetime: bool,
    has_default_impl: bool,
    has_builder: bool,
    default_impl: bool,
}

impl SchemaType {
    fn new(name: String, description: Option<String>, source_file: String) -> Self {
        Self {
            name,
            description,
            source_file,
            fields: Vec::new(),
            sub_types: Vec::new(),
            enums: Vec::new(),
            needs_hashmap: false,
            needs_datetime: false,
            has_default_impl: false,
            has_builder: false,
            default_impl: false,
        }
    }
}

#[derive(Debug)]
struct SchemaParser {
    handlebars: Handlebars<'static>,
    seen_types: HashSet<String>,
    type_mapping: HashMap<String, String>,
}

impl SchemaParser {
    fn new() -> Self {
        let mut handlebars = Handlebars::new();
        handlebars
            .register_template_string("schema", SCHEMA_TEMPLATE)
            .expect("Failed to register schema template");

        let mut type_mapping = HashMap::new();
        type_mapping.insert("string".to_string(), "String".to_string());
        type_mapping.insert("integer".to_string(), "i64".to_string());
        type_mapping.insert("number".to_string(), "f64".to_string());
        type_mapping.insert("boolean".to_string(), "bool".to_string());
        type_mapping.insert("object".to_string(), "Value".to_string());
        type_mapping.insert("array".to_string(), "Vec<T>".to_string());

        Self {
            handlebars,
            seen_types: HashSet::new(),
            type_mapping,
        }
    }

    fn parse_schema(
        &mut self,
        schema_path: &Path,
    ) -> Result<SchemaType, Box<dyn std::error::Error>> {
        let content = fs::read_to_string(schema_path)?;
        let schema: Value = serde_json::from_str(&content)?;

        let schema_name = schema_path
            .file_stem()
            .and_then(|n| n.to_str())
            .ok_or("Invalid schema filename")?;

        // Get the parent directory name (endpoint name)
        let endpoint_name = schema_path
            .parent()
            .and_then(|p| p.file_name())
            .and_then(|n| n.to_str())
            .ok_or("Invalid schema path")?;

        // Determine if this is a send.json or receive.json file and adjust the name accordingly
        let type_name = match schema_name {
            "send" => format!("{}Request", self.to_type_name(endpoint_name)),
            "receive" => format!("{}Response", self.to_type_name(endpoint_name)),
            _ => self.to_type_name(schema_name),
        };

        let mut schema_type = SchemaType::new(
            type_name,
            schema
                .get("description")
                .and_then(|v| v.as_str())
                .map(String::from),
            schema_path.to_string_lossy().to_string(),
        );

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

            self.process_properties(&mut schema_type, properties, &required)?;
        }

        Ok(schema_type)
    }

    fn process_properties(
        &mut self,
        schema_type: &mut SchemaType,
        properties: &Map<String, Value>,
        required: &HashSet<String>,
    ) -> Result<(), Box<dyn std::error::Error>> {
        for (property_name, property_value) in properties {
            let field = self.process_property(
                property_name,
                property_value,
                required.contains(property_name),
                schema_type,
            )?;

            schema_type.fields.push(field);

            if property_value.get("type") == Some(&json!("object")) {
                schema_type.needs_hashmap = true;
            }

            if self.is_datetime_field(property_value) {
                schema_type.needs_datetime = true;
            }
        }

        Ok(())
    }

    fn process_property(
        &mut self,
        name: &str,
        property: &Value,
        is_required: bool,
        parent_type: &mut SchemaType,
    ) -> Result<SchemaField, Box<dyn std::error::Error>> {
        let mut field_type = self.determine_field_type(property, name, parent_type)?;

        // Decode any HTML entities in the field type
        field_type = Self::decode_html_entities(&field_type);

        let is_pattern_property = property.get("patternProperties").is_some();
        let is_nullable = self.is_nullable_type(property);

        if is_nullable && !is_pattern_property {
            field_type = format!("Option<{}>", field_type);
        }

        let builder = SchemaFieldBuilder::default()
            .field_name(self.to_field_name(name))
            .field_type(field_type)
            .description(
                property
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|d| Self::decode_html_entities(&d.replace('\n', "\n/// "))),
            )
            .rename(Some(name.to_string()))
            .skip_serializing_if(!is_required || is_nullable)
            .flatten(is_pattern_property)
            .default_value(self.get_default_value(property))
            .is_required(is_required)
            .build()?;

        Ok(builder)
    }

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
                if !result.ends_with('_') {
                    result.push('_');
                }
            } else {
                result.push(c);
            }
        }

        result.to_lowercase()
    }

    fn determine_field_type(
        &mut self,
        property: &Value,
        name: &str,
        parent_type: &mut SchemaType,
    ) -> Result<String, Box<dyn std::error::Error>> {
        if let Some(enum_values) = property.get("enum") {
            return self.handle_enum_type(name, property, enum_values, parent_type);
        }

        let type_value = property.get("type");
        match type_value.and_then(|t| t.as_str()) {
            Some("array") => self.handle_array_type(property, name, parent_type),
            Some("object") => self.handle_object_type(property, name, parent_type),
            Some(type_name) => Ok(self
                .type_mapping
                .get(type_name)
                .cloned()
                .unwrap_or_else(|| "Value".to_string())),
            None => Ok("Value".to_string()),
        }
    }

    fn handle_enum_type(
        &mut self,
        name: &str,
        property: &Value,
        enum_values: &Value,
        parent_type: &mut SchemaType,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let enum_name = format!("{}Enum", self.to_type_name(name));

        if !self.seen_types.contains(&enum_name) {
            self.seen_types.insert(enum_name.clone());

            let mut variants = Vec::new();
            if let Some(values) = enum_values.as_array() {
                for (i, value) in values.iter().enumerate() {
                    let variant = match value {
                        Value::String(s) => EnumVariant {
                            name: self.to_enum_variant_name(s),
                            original_name: s.clone(),
                            value: None,
                            description: None,
                        },
                        Value::Number(n) => EnumVariant {
                            name: format!("Value{}", n),
                            original_name: n.to_string(),
                            value: n.as_i64(),
                            description: None,
                        },
                        _ => continue,
                    };
                    variants.push(variant);
                }
            }

            parent_type.enums.push(EnumType {
                name: enum_name.clone(),
                description: property
                    .get("description")
                    .and_then(|v| v.as_str())
                    .map(|d| Self::decode_html_entities(d)),
                variants,
            });
        }

        Ok(enum_name)
    }

    fn handle_array_type(
        &mut self,
        property: &Value,
        name: &str,
        parent_type: &mut SchemaType,
    ) -> Result<String, Box<dyn std::error::Error>> {
        let items = property.get("items").unwrap_or(&Value::Null);
        let item_type = if items.is_null() {
            "Value".to_string()
        } else {
            self.determine_field_type(items, &format!("{}Item", name), parent_type)?
        };

        Ok(format!("Vec<{}>", item_type))
    }

    fn handle_object_type(
        &mut self,
        property: &Value,
        name: &str,
        parent_type: &mut SchemaType,
    ) -> Result<String, Box<dyn std::error::Error>> {
        // Handle pattern properties (like in balance.accounts)
        if let Some(pattern_props) = property.get("patternProperties") {
            let first_value = pattern_props
                .as_object()
                .and_then(|obj| obj.values().next())
                .ok_or("Invalid pattern properties")?;

            let value_type =
                self.determine_field_type(first_value, &format!("{}Value", name), parent_type)?;

            return Ok(format!("HashMap<String, {}>", value_type));
        }

        // Handle regular object type
        let type_name = self.to_type_name(name);
        if !self.seen_types.contains(&type_name) {
            self.seen_types.insert(type_name.clone());

            if let Some(properties) = property.get("properties").and_then(|v| v.as_object()) {
                let mut sub_type = SchemaType::new(
                    type_name.clone(),
                    property
                        .get("description")
                        .and_then(|v| v.as_str())
                        .map(String::from),
                    String::new(),
                );

                let required: HashSet<String> = property
                    .get("required")
                    .and_then(|v| v.as_array())
                    .map(|arr| {
                        arr.iter()
                            .filter_map(|v| v.as_str())
                            .map(String::from)
                            .collect()
                    })
                    .unwrap_or_default();

                self.process_properties(&mut sub_type, properties, &required)?;

                let sub_type_str = self.render_type(&sub_type)?;
                parent_type.sub_types.push(sub_type_str);
            }
        }

        Ok(type_name)
    }

    fn is_datetime_field(&self, property: &Value) -> bool {
        let description = property
            .get("description")
            .and_then(|v| v.as_str())
            .unwrap_or("");

        let type_is_integer = property
            .get("type")
            .and_then(|v| v.as_str())
            .map(|t| t == "integer")
            .unwrap_or(false);

        type_is_integer
            && (description.contains("epoch")
                || property
                    .get("name")
                    .and_then(|v| v.as_str())
                    .map(|name| name.contains("time") || name.contains("date"))
                    .unwrap_or(false))
    }

    fn is_nullable_type(&self, property: &Value) -> bool {
        if let Some(types) = property.get("type").and_then(|t| t.as_array()) {
            types.iter().any(|t| t.as_str() == Some("null"))
        } else {
            false
        }
    }

    fn get_default_value(&self, property: &Value) -> String {
        match property.get("type").and_then(|t| t.as_str()) {
            Some("string") => "String::new()".to_string(),
            Some("integer") => "0".to_string(),
            Some("number") => "0.0".to_string(),
            Some("boolean") => "false".to_string(),
            Some("array") => "Vec::new()".to_string(),
            Some("object") => "HashMap::new()".to_string(),
            _ => "Default::default()".to_string(),
        }
    }

    fn to_type_name(&self, name: &str) -> String {
        let mut result = String::new();
        let mut capitalize_next = true;

        for c in name.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.extend(c.to_uppercase());
                capitalize_next = false;
            } else {
                result.extend(c.to_lowercase());
            }
        }

        result
    }

    fn to_field_name(&self, name: &str) -> String {
        let mut result = String::new();
        let mut prev_was_num = false;
        let mut prev_was_upper = false;

        for (i, c) in name.chars().enumerate() {
            if c.is_uppercase() {
                if i > 0 && !prev_was_upper && !result.ends_with('_') {
                    result.push('_');
                }
                result.extend(c.to_lowercase());
                prev_was_upper = true;
                prev_was_num = false;
            } else if c.is_numeric() {
                if i > 0 && !prev_was_num && !result.ends_with('_') {
                    result.push('_');
                }
                result.push(c);
                prev_was_num = true;
                prev_was_upper = false;
            } else if c == '_' {
                if !result.ends_with('_') {
                    result.push('_');
                }
                prev_was_num = false;
                prev_was_upper = false;
            } else {
                result.push(c);
                prev_was_num = false;
                prev_was_upper = false;
            }
        }

        // Special case: handle p2p -> p2p (not p_2_p)
        result = result.replace("p_2_p", "p2p");

        result
    }

    fn to_enum_variant_name(&self, name: &str) -> String {
        // Handle numeric case first
        if name.chars().all(|c| c.is_numeric()) {
            return format!("Value{}", name);
        }

        let sanitized = name
            .replace(['\'', '"', '-', '.'], "")
            .replace(['(', ')', ' '], "_");

        let mut result = String::new();
        let mut capitalize_next = true;

        for c in sanitized.chars() {
            if c == '_' {
                capitalize_next = true;
                result.push(c);
            } else if capitalize_next {
                result.extend(c.to_uppercase());
                capitalize_next = false;
            } else {
                result.extend(c.to_string().chars());
            }
        }

        // Remove trailing underscore if any
        if result.ends_with('_') {
            result.pop();
        }

        // For non-numeric case, if it starts with a number, prefix with underscore
        if !name.chars().all(|c| c.is_numeric())
            && result.chars().next().unwrap_or('_').is_numeric()
        {
            result.insert(0, '_');
        }

        result
    }

    fn render_type(&self, schema_type: &SchemaType) -> Result<String, Box<dyn std::error::Error>> {
        // First, register a helper that decodes HTML entities
        let mut handlebars = Handlebars::new();
        handlebars.register_template_string("schema", SCHEMA_TEMPLATE)?;
        handlebars.register_helper(
            "decode_html",
            Box::new(
                |h: &handlebars::Helper,
                 _: &handlebars::Handlebars,
                 _: &handlebars::Context,
                 _: &mut handlebars::RenderContext,
                 out: &mut dyn handlebars::Output| {
                    let param = h.param(0).and_then(|v| v.value().as_str()).unwrap_or("");
                    out.write(&Self::decode_html_entities(param)).map_err(|e| {
                        handlebars::RenderError::new(format!("Failed to write output: {}", e))
                    })
                },
            ),
        );

        // Then render the template
        handlebars
            .render("schema", &json!(schema_type))
            .map_err(|e| e.into())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    info!("Starting schema generation");

    let input_dir =
        std::env::var("SCHEMA_PATH").unwrap_or_else(|_| "./deriv-api-docs/config/v3".to_string());
    let output_dir =
        std::env::var("SCHEMA_OUTPUT_PATH").unwrap_or_else(|_| "./schema/src".to_string());

    info!("Input directory: {}", input_dir);
    info!("Output directory: {}", output_dir);

    fs::create_dir_all(&output_dir)?;

    let mut parser = SchemaParser::new();
    let mut generated_modules = Vec::new();

    for entry in fs::read_dir(&input_dir)? {
        let entry = entry?;
        let path = entry.path();

        if !path.is_dir() {
            continue;
        }

        let schema_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        debug!("Processing schema: {}", schema_name);

        // Process send.json
        if let Ok(schema) = parser.parse_schema(&path.join("send.json")) {
            let request_name = format!("{}Request", parser.to_type_name(&schema_name));
            let content = parser.render_type(&schema)?;
            let file_name = parser.to_file_name(&request_name);
            let output_path = Path::new(&output_dir).join(format!("{}.rs", file_name));

            fs::write(&output_path, content)?;
            generated_modules.push(file_name);

            debug!("Generated request schema: {:?}", output_path);
        }

        // Process receive.json
        if let Ok(schema) = parser.parse_schema(&path.join("receive.json")) {
            let response_name = format!("{}Response", parser.to_type_name(&schema_name));
            let content = parser.render_type(&schema)?;
            let file_name = parser.to_file_name(&response_name);
            let output_path = Path::new(&output_dir).join(format!("{}.rs", file_name));

            fs::write(&output_path, content)?;
            generated_modules.push(file_name);

            debug!("Generated response schema: {:?}", output_path);
        }
    }

    // Generate mod.rs
    let mod_content = generate_mod_content(&generated_modules);
    fs::write(Path::new(&output_dir).join("mod.rs"), mod_content)?;

    info!("Schema generation completed successfully");
    Ok(())
}

fn generate_mod_content(modules: &[String]) -> String {
    let mut content = String::from("// Generated by schema_generator.rs - DO NOT EDIT.\n\n");

    for module in modules {
        content.push_str(&format!("mod {};\n", module));
        content.push_str(&format!("pub use {}::*;\n", module));
    }

    content
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_type_name_conversion() {
        let parser = SchemaParser::new();
        // Test actual API type names
        assert_eq!(
            parser.to_type_name("proposal_open_contract"),
            "ProposalOpenContract"
        );
        assert_eq!(parser.to_type_name("msg_type"), "MsgType");
        assert_eq!(parser.to_type_name("tick_stream"), "TickStream");
    }

    #[test]
    fn test_field_name_conversion() {
        let parser = SchemaParser::new();
        // Test actual API field names
        assert_eq!(
            parser.to_field_name("proposal_open_contract"),
            "proposal_open_contract"
        );
        assert_eq!(parser.to_field_name("msg_type"), "msg_type");
        assert_eq!(parser.to_field_name("contract_id"), "contract_id");
    }

    #[test]
    fn test_enum_variant_name_conversion() {
        let parser = SchemaParser::new();
        // Test actual enum variants from the API
        assert_eq!(parser.to_enum_variant_name("1"), "Value1"); // For proposal_open_contract
        assert_eq!(parser.to_enum_variant_name("open"), "Open"); // For status enum
        assert_eq!(parser.to_enum_variant_name("won"), "Won"); // For status enum
        assert_eq!(parser.to_enum_variant_name("lost"), "Lost"); // For status enum
        assert_eq!(parser.to_enum_variant_name("cancelled"), "Cancelled"); // For status enum
    }

    #[test]
    fn test_is_datetime_field() {
        let parser = SchemaParser::new();
        // Test actual datetime fields from the API
        let expiry_date = json!({
            "type": "integer",
            "description": "Expiry date (epoch) of the Contract"
        });
        assert!(parser.is_datetime_field(&expiry_date));

        let tick_count = json!({
            "type": "integer",
            "description": "Only for tick trades, number of ticks"
        });
        assert!(!parser.is_datetime_field(&tick_count));
    }
}
