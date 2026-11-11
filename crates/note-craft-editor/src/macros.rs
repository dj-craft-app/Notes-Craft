//! Macros System Module - Système de macros pour l'éditeur
//!
//! Ce module fournit:
//! - Définition de macros utilisateur
//! - Expansion de macros dans le contenu Markdown
//! - Gestion des variables dans les macros
//! - Stockage et récupération des macros

use std::collections::HashMap;

/// Erreurs possibles lors de la gestion des macros
#[derive(Debug, Clone, PartialEq, thiserror::Error)]
pub enum MacroError {
    #[error("Macro '{0}' not found")]
    MacroNotFound(String),
    #[error("Variable '{0}' not found")]
    VariableNotFound(String),
    #[error("Circular reference detected: {0}")]
    CircularReference(String),
    #[error("Invalid macro name: '{0}'")]
    InvalidMacroName(String),
    #[error("Invalid variable name: '{0}'")]
    InvalidVariableName(String),
    #[error("Syntax error in macro definition: {0}")]
    SyntaxError(String),
}

pub type MacroResult<T> = Result<T, MacroError>;

/// Type de contenu d'une macro
#[derive(Debug, Clone, PartialEq)]
pub enum MacroContent {
    /// Contenu statique (texte brut)
    Static(String),
    /// Contenu dynamique avec variables
    Dynamic(String),
}

/// Définition d'une macro
#[derive(Debug, Clone)]
pub struct Macro {
    /// Nom de la macro
    pub name: String,
    /// Contenu de la macro
    pub content: MacroContent,
    /// Variables utilisées dans la macro
    pub variables: Vec<String>,
    /// Description de la macro
    pub description: Option<String>,
}

impl Macro {
    pub fn new(name: &str, content: MacroContent) -> Self {
        let variables = extract_variables(&content);
        Self {
            name: name.to_string(),
            content,
            variables,
            description: None,
        }
    }

    pub fn with_description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    /// Vérifie si le nom est valide
    pub fn is_valid_name(name: &str) -> bool {
        if name.is_empty() {
            return false;
        }
        // Un nom valide commence par une lettre ou underscore, suivi de lettres, chiffres ou underscores
        let chars: Vec<char> = name.chars().collect();
        if !chars[0].is_alphabetic() && chars[0] != '_' {
            return false;
        }
        chars
            .iter()
            .all(|&c| c.is_alphanumeric() || c == '_' || c == '-')
    }

    /// Expand la macro avec les variables fournies
    pub fn expand(&self, variables: &HashMap<String, String>) -> MacroResult<String> {
        match &self.content {
            MacroContent::Static(content) => Ok(content.clone()),
            MacroContent::Dynamic(content) => {
                let mut result = content.clone();

                // Remplacer les variables dans l'ordre de longueur décroissante
                // pour éviter les remplacements partiels
                let mut sorted_vars: Vec<&String> = self.variables.iter().collect();
                sorted_vars.sort_by_key(|s| std::cmp::Reverse(s.len()));

                for var in sorted_vars {
                    if let Some(value) = variables.get(var) {
                        let placeholder = format!("${}", var);
                        result = result.replace(&placeholder, value);
                    } else {
                        return Err(MacroError::VariableNotFound(var.clone()));
                    }
                }

                Ok(result)
            }
        }
    }
}

/// Extrait les variables d'un contenu de macro
fn extract_variables(content: &MacroContent) -> Vec<String> {
    let text = match content {
        MacroContent::Static(s) => s.as_str(),
        MacroContent::Dynamic(s) => s.as_str(),
    };

    let mut variables = Vec::new();
    let mut chars = text.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '$' {
            // Début d'une variable
            let mut var_name = String::new();
            let mut next_char = chars.next();

            // Lire jusqu'à la fin de la variable (non-alphanumérique)
            while let Some(ch) = next_char {
                if ch.is_alphanumeric() || ch == '_' {
                    var_name.push(ch);
                    next_char = chars.next();
                } else {
                    // On a lu un caractère de trop
                    break;
                }
            }

            if !var_name.is_empty() {
                variables.push(var_name);
            }
        }
    }

    // Supprimer les doublons
    variables.sort();
    variables.dedup();
    variables
}

/// Gestionnaire des macros
#[derive(Default)]
pub struct MacroManager {
    /// Macros définies
    macros: HashMap<String, Macro>,
    /// Variables globales
    global_variables: HashMap<String, String>,
    /// Ordre de résolution pour détecter les références circulaires (réservé pour futur usage)
    #[allow(dead_code)]
    resolution_stack: Vec<String>,
}

impl MacroManager {
    pub fn new() -> Self {
        Self::default()
    }

    /// Ajoute une nouvelle macro
    pub fn add_macro(&mut self, name: &str, content: MacroContent) -> MacroResult<()> {
        if !Macro::is_valid_name(name) {
            return Err(MacroError::InvalidMacroName(name.to_string()));
        }

        let macro_def = Macro::new(name, content);
        self.macros.insert(name.to_string(), macro_def);
        Ok(())
    }

    /// Ajoute une macro avec description
    pub fn add_macro_with_description(
        &mut self,
        name: &str,
        content: MacroContent,
        description: &str,
    ) -> MacroResult<()> {
        if !Macro::is_valid_name(name) {
            return Err(MacroError::InvalidMacroName(name.to_string()));
        }

        let macro_def = Macro::new(name, content).with_description(description);
        self.macros.insert(name.to_string(), macro_def);
        Ok(())
    }

    /// Supprime une macro
    pub fn remove_macro(&mut self, name: &str) -> MacroResult<()> {
        if self.macros.remove(name).is_none() {
            return Err(MacroError::MacroNotFound(name.to_string()));
        }
        Ok(())
    }

    /// Récupère une macro par son nom
    pub fn get_macro(&self, name: &str) -> Option<&Macro> {
        self.macros.get(name)
    }

    /// Liste toutes les macros
    pub fn list_macros(&self) -> Vec<&Macro> {
        self.macros.values().collect()
    }

    /// Ajoute une variable globale
    pub fn set_variable(&mut self, name: &str, value: &str) -> MacroResult<()> {
        if !is_valid_variable_name(name) {
            return Err(MacroError::InvalidVariableName(name.to_string()));
        }
        self.global_variables
            .insert(name.to_string(), value.to_string());
        Ok(())
    }

    /// Supprime une variable globale
    pub fn remove_variable(&mut self, name: &str) -> MacroResult<()> {
        if self.global_variables.remove(name).is_none() {
            return Err(MacroError::VariableNotFound(name.to_string()));
        }
        Ok(())
    }

    /// Récupère une variable globale
    pub fn get_variable(&self, name: &str) -> Option<&String> {
        self.global_variables.get(name)
    }

    /// Liste toutes les variables globales
    pub fn list_variables(&self) -> Vec<(&String, &String)> {
        self.global_variables.iter().collect()
    }

    /// Expand une macro avec ses variables locales et les variables globales
    pub fn expand_macro(
        &self,
        name: &str,
        local_variables: Option<HashMap<String, String>>,
    ) -> MacroResult<String> {
        let macro_def = self
            .macros
            .get(name)
            .ok_or_else(|| MacroError::MacroNotFound(name.to_string()))?;

        // Préparer les variables : locales + globales
        let mut all_vars = self.global_variables.clone();
        if let Some(local) = local_variables {
            all_vars.extend(local);
        }

        macro_def.expand(&all_vars)
    }

    /// Expand toutes les macros dans un texte
    pub fn expand_all_in_text(&self, text: &str) -> MacroResult<String> {
        self.expand_all_in_text_with_vars(text, None)
    }

    /// Expand toutes les macros dans un texte avec des variables locales
    pub fn expand_all_in_text_with_vars(
        &self,
        text: &str,
        _local_variables: Option<HashMap<String, String>>,
    ) -> MacroResult<String> {
        let mut result = text.to_string();
        let mut max_iterations = 100; // Prévenir les boucles infinies

        // Trouver et expandir les macros de manière itérative
        while max_iterations > 0 {
            // Trouver toutes les invocations de macros
            let macro_calls = self.find_macro_calls(&result);

            if macro_calls.is_empty() {
                break;
            }

            // Prendre la première macro (par position)
            let first_call = &macro_calls[0];
            let macro_name = first_call.0.clone();
            let start_pos = first_call.1;
            let end_pos = first_call.2;
            let macro_def = self
                .macros
                .get(&macro_name)
                .ok_or_else(|| MacroError::MacroNotFound(macro_name.clone()))?;

            // Extraire les arguments/options si présent
            let args = if end_pos > start_pos + macro_name.len() + 1 {
                // Il y a des arguments après le nom
                let args_str = &result[start_pos + macro_name.len() + 1..end_pos];
                parse_macro_arguments(args_str)
            } else {
                HashMap::new()
            };

            // Expandir la macro
            let expanded = macro_def.expand(&args)?;

            // Remplacer dans le texte
            result = format!(
                "{}{}{}",
                &result[..start_pos],
                expanded,
                &result[end_pos + 2..] // +2 pour sauter les "}}"
            );

            max_iterations -= 1;
        }

        if max_iterations == 0 {
            return Err(MacroError::CircularReference(
                "Maximum recursion depth exceeded".to_string(),
            ));
        }

        Ok(result)
    }

    /// Trouve tous les appels de macros dans un texte
    fn find_macro_calls(&self, text: &str) -> Vec<(String, usize, usize)> {
        let mut calls = Vec::new();
        let chars = text.chars().collect::<Vec<char>>();
        let mut i = 0;

        while i < chars.len() - 1 {
            // Chercher "{{"
            if chars[i] == '{' && chars[i + 1] == '{' {
                // Trouver le "}}" correspondant
                let mut j = i + 2;
                while j < chars.len() - 1 {
                    if chars[j] == '}' && chars[j + 1] == '}' {
                        // Extraire le nom de la macro
                        let macro_content: String = chars[i + 2..j].iter().collect();
                        let parts: Vec<&str> = macro_content.split_whitespace().collect();
                        if !parts.is_empty() {
                            let macro_name = parts[0].to_string();
                            if self.macros.contains_key(&macro_name) {
                                calls.push((macro_name, i, j));
                            }
                        }
                        i = j + 2;
                        break;
                    }
                    j += 1;
                }
            }
            i += 1;
        }

        calls
    }
}

/// Vérifie si un nom de variable est valide
fn is_valid_variable_name(name: &str) -> bool {
    if name.is_empty() {
        return false;
    }
    let chars: Vec<char> = name.chars().collect();
    if !chars[0].is_alphabetic() && chars[0] != '_' {
        return false;
    }
    chars.iter().all(|&c| c.is_alphanumeric() || c == '_')
}

/// Parse les arguments d'une macro
fn parse_macro_arguments(args_str: &str) -> HashMap<String, String> {
    let mut args = HashMap::new();

    for arg in args_str.split(',') {
        let parts: Vec<&str> = arg.split('=').map(|s| s.trim()).collect();
        if parts.len() == 2 {
            args.insert(parts[0].to_string(), parts[1].to_string());
        } else if parts.len() == 1 {
            // Argument sans valeur (flag)
            args.insert(parts[0].to_string(), "true".to_string());
        }
    }

    args
}

/// Système de macros par défaut avec des macros utiles pré-définies
pub fn create_default_macro_manager() -> MacroManager {
    let mut manager = MacroManager::new();

    // Ajouter des macros utiles par défaut
    manager
        .add_macro("date", MacroContent::Static("{{current-date}}".to_string()))
        .ok();

    manager
        .add_macro(
            "timestamp",
            MacroContent::Static("{{current-timestamp}}".to_string()),
        )
        .ok();

    manager
        .add_macro(
            "signature",
            MacroContent::Dynamic("Best regards,\n$name".to_string()),
        )
        .ok();

    manager
        .add_macro(
            "header",
            MacroContent::Dynamic("# $title\n\nBy: $author\nDate: $date".to_string()),
        )
        .ok();

    manager
}

/// Macro Builder pour une construction plus fluide
pub struct MacroBuilder {
    name: String,
    content: MacroContent,
    description: Option<String>,
}

impl MacroBuilder {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            content: MacroContent::Static(String::new()),
            description: None,
        }
    }

    pub fn static_content(mut self, content: &str) -> Self {
        self.content = MacroContent::Static(content.to_string());
        self
    }

    pub fn dynamic_content(mut self, content: &str) -> Self {
        self.content = MacroContent::Dynamic(content.to_string());
        self
    }

    pub fn description(mut self, desc: &str) -> Self {
        self.description = Some(desc.to_string());
        self
    }

    pub fn build(self) -> Macro {
        let mut macro_def = Macro::new(&self.name, self.content);
        if let Some(desc) = self.description {
            macro_def = macro_def.with_description(&desc);
        }
        macro_def
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_macro_new() {
        let macro_def = Macro::new("test", MacroContent::Static("Hello World".to_string()));
        assert_eq!(macro_def.name, "test");
        assert!(macro_def.variables.is_empty());
    }

    #[test]
    fn test_macro_with_variables() {
        let content = MacroContent::Dynamic("Hello $name, welcome to $place!".to_string());
        let macro_def = Macro::new("greet", content);

        assert_eq!(macro_def.variables.len(), 2);
        assert!(macro_def.variables.contains(&"name".to_string()));
        assert!(macro_def.variables.contains(&"place".to_string()));
    }

    #[test]
    fn test_macro_expand_static() {
        let macro_def = Macro::new("static", MacroContent::Static("Static content".to_string()));
        let vars = HashMap::<String, String>::new();
        let result = macro_def.expand(&vars).unwrap();
        assert_eq!(result, "Static content");
    }

    #[test]
    fn test_macro_expand_dynamic() {
        let content = MacroContent::Dynamic("Hello $name!".to_string());
        let macro_def = Macro::new("greet", content);

        let vars = HashMap::from([("name".to_string(), "Alice".to_string())]);

        let result = macro_def.expand(&vars).unwrap();
        assert_eq!(result, "Hello Alice!");
    }

    #[test]
    fn test_macro_expand_missing_variable() {
        let content = MacroContent::Dynamic("Hello $name!".to_string());
        let macro_def = Macro::new("greet", content);

        let vars = HashMap::<String, String>::new();
        let result = macro_def.expand(&vars);

        assert!(matches!(result, Err(MacroError::VariableNotFound(_))));
    }

    #[test]
    fn test_macro_manager_add_remove() {
        let mut manager = MacroManager::new();

        manager
            .add_macro("test", MacroContent::Static("content".to_string()))
            .unwrap();
        assert!(manager.get_macro("test").is_some());

        manager.remove_macro("test").unwrap();
        assert!(manager.get_macro("test").is_none());
    }

    #[test]
    fn test_macro_manager_remove_nonexistent() {
        let mut manager = MacroManager::new();
        let result = manager.remove_macro("nonexistent");
        assert!(matches!(result, Err(MacroError::MacroNotFound(_))));
    }

    #[test]
    fn test_macro_manager_variables() {
        let mut manager = MacroManager::new();

        manager.set_variable("author", "John Doe").unwrap();
        assert_eq!(
            manager.get_variable("author"),
            Some(&"John Doe".to_string())
        );

        manager.remove_variable("author").unwrap();
        assert!(manager.get_variable("author").is_none());
    }

    #[test]
    fn test_macro_manager_expand_with_globals() {
        let mut manager = MacroManager::new();
        manager
            .add_macro("greet", MacroContent::Dynamic("Hello $name!".to_string()))
            .unwrap();
        manager.set_variable("name", "Bob").unwrap();

        let result = manager.expand_macro("greet", None).unwrap();
        assert_eq!(result, "Hello Bob!");
    }

    #[test]
    fn test_macro_manager_expand_with_locals() {
        let mut manager = MacroManager::new();
        manager
            .add_macro("greet", MacroContent::Dynamic("Hello $name!".to_string()))
            .unwrap();
        manager.set_variable("name", "Bob").unwrap();

        let local_vars = HashMap::from([("name".to_string(), "Alice".to_string())]);

        let result = manager.expand_macro("greet", Some(local_vars)).unwrap();
        assert_eq!(result, "Hello Alice!");
    }

    #[test]
    fn test_macro_manager_expand_nonexistent() {
        let manager = MacroManager::new();
        let result = manager.expand_macro("nonexistent", None);
        assert!(matches!(result, Err(MacroError::MacroNotFound(_))));
    }

    #[test]
    fn test_invalid_macro_name() {
        assert!(!Macro::is_valid_name(""));
        assert!(!Macro::is_valid_name("123invalid"));
        assert!(!Macro::is_valid_name("has spaces"));
        assert!(Macro::is_valid_name("valid_name"));
        assert!(Macro::is_valid_name("_valid"));
        assert!(Macro::is_valid_name("CamelCase"));
    }

    #[test]
    fn test_extract_variables() {
        let content = MacroContent::Dynamic("Hello $name, welcome to $place!".to_string());
        let vars = extract_variables(&content);
        assert_eq!(vars.len(), 2);
        assert!(vars.contains(&"name".to_string()));
        assert!(vars.contains(&"place".to_string()));
    }

    #[test]
    fn test_extract_variables_no_duplicates() {
        let content = MacroContent::Dynamic("Hello $name, $name!".to_string());
        let vars = extract_variables(&content);
        assert_eq!(vars.len(), 1);
        assert!(vars.contains(&"name".to_string()));
    }

    #[test]
    fn test_macro_builder() {
        let macro_def = MacroBuilder::new("header")
            .dynamic_content("# $title\n\nBy: $author")
            .description("Document header")
            .build();

        assert_eq!(macro_def.name, "header");
        assert_eq!(macro_def.variables.len(), 2);
        assert_eq!(macro_def.description, Some("Document header".to_string()));
    }

    #[test]
    fn test_default_macro_manager() {
        let manager = create_default_macro_manager();
        assert!(manager.get_macro("date").is_some());
        assert!(manager.get_macro("timestamp").is_some());
        assert!(manager.get_macro("signature").is_some());
        assert!(manager.get_macro("header").is_some());
    }

    #[test]
    fn test_list_macros() {
        let mut manager = MacroManager::new();
        manager
            .add_macro("macro1", MacroContent::Static("content1".to_string()))
            .unwrap();
        manager
            .add_macro("macro2", MacroContent::Static("content2".to_string()))
            .unwrap();

        let macros = manager.list_macros();
        assert_eq!(macros.len(), 2);
    }

    #[test]
    fn test_list_variables() {
        let mut manager = MacroManager::new();
        manager.set_variable("var1", "value1").unwrap();
        manager.set_variable("var2", "value2").unwrap();

        let vars = manager.list_variables();
        assert_eq!(vars.len(), 2);
    }

    #[test]
    fn test_invalid_variable_name() {
        assert!(!is_valid_variable_name(""));
        assert!(!is_valid_variable_name("123invalid"));
        assert!(!is_valid_variable_name("has-dash"));
        assert!(is_valid_variable_name("valid_name"));
        assert!(is_valid_variable_name("_valid"));
    }

    #[test]
    fn test_parse_macro_arguments() {
        let args = parse_macro_arguments("name=John, age=30, active");
        assert_eq!(args.len(), 3);
        assert_eq!(args.get("name"), Some(&"John".to_string()));
        assert_eq!(args.get("age"), Some(&"30".to_string()));
        assert_eq!(args.get("active"), Some(&"true".to_string()));
    }

    #[test]
    fn test_macro_expand_multiple_vars() {
        let content =
            MacroContent::Dynamic("$greeting $name, you have $count messages".to_string());
        let macro_def = Macro::new("message", content);

        let mut vars = HashMap::new();
        vars.insert("greeting".to_string(), "Hello".to_string());
        vars.insert("name".to_string(), "Alice".to_string());
        vars.insert("count".to_string(), "5".to_string());

        let result = macro_def.expand(&vars).unwrap();
        assert_eq!(result, "Hello Alice, you have 5 messages");
    }
}
