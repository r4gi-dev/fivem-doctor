use crate::diagnostic::{Diagnostic, Severity};
use full_moon::ast::{FunctionCall, NumericFor, Repeat, While};
use full_moon::node::Node;
use full_moon::visitors::Visitor;
use std::collections::HashSet;

pub fn analyze_lua(ast: &full_moon::ast::Ast, file: &str) -> Vec<Diagnostic> {
    let mut visitor = LuaRuleVisitor::new(file);

    visitor.visit_ast(ast);

    visitor.diagnostics
}

struct LuaRuleVisitor<'a> {
    file: &'a str,
    loop_depth: usize,
    diagnostics: Vec<Diagnostic>,

    /// Events registered through RegisterNetEvent.
    registered_network_events: HashSet<String>,

    /// Events for which F004 has already been emitted.
    reported_network_events: HashSet<String>,
}

impl<'a> LuaRuleVisitor<'a> {
    fn new(file: &'a str) -> Self {
        Self {
            file,
            loop_depth: 0,
            diagnostics: Vec::new(),
            registered_network_events: HashSet::new(),
            reported_network_events: HashSet::new(),
        }
    }

    fn check_function_call(&mut self, call: &FunctionCall) {
        let call_text = call.to_string();
        let function_name = function_name(&call_text);

        match function_name {
            Some("print") => {
                let mut diagnostic = Diagnostic::new(
                    "F006",
                    Severity::Info,
                    "Debug output using print() was found.",
                    self.file,
                )
                .with_suggestion(
                    "Remove debug output or replace it with an appropriate logging mechanism.",
                );

                if let Some(position) = call.start_position() {
                    diagnostic =
                        diagnostic.with_location(position.line(), position.character() + 1);
                }

                self.diagnostics.push(diagnostic);
            }

            Some("Wait") if self.loop_depth > 0 && is_zero_wait(&call_text) => {
                let mut diagnostic = Diagnostic::new(
                    "F005",
                    Severity::Warning,
                    "Wait(0) was found inside a loop.",
                    self.file,
                )
                .with_explanation(
                    "A continuously running Wait(0) loop executes every frame and can become expensive when the loop body performs non-trivial work.",
                )
                .with_suggestion(
                    "Use a larger Wait interval when per-frame execution is not required.",
                );

                if let Some(position) = call.start_position() {
                    diagnostic =
                        diagnostic.with_location(position.line(), position.character() + 1);
                }

                self.diagnostics.push(diagnostic);
            }

            Some("RegisterNetEvent") => {
                self.check_register_net_event(&call_text, call);
            }

            Some("AddEventHandler") => {
                self.check_network_event_handler(&call_text, call);
            }

            Some("TriggerServerEvent") => {
                self.check_f008(&call_text, call);
            }

            _ => {}
        }
    }

    fn check_register_net_event(&mut self, call_text: &str, call: &FunctionCall) {
        let Some(event_name) = first_string_argument(call_text) else {
            return;
        };

        self.registered_network_events.insert(event_name.clone());

        /*
         * FiveM also permits:
         *
         * RegisterNetEvent('event:name', function(...)
         *     ...
         * end)
         *
         * Handle this form directly as well.
         */
        if contains_inline_handler(call_text)
            && handler_has_arguments(call_text)
            && !has_obvious_validation(call_text)
        {
            self.report_f004(call, &event_name);
        }
    }

    fn check_network_event_handler(&mut self, call_text: &str, call: &FunctionCall) {
        let Some(event_name) = first_string_argument(call_text) else {
            return;
        };

        /*
         * Only treat AddEventHandler as a network-event handler when the
         * corresponding event was previously registered with RegisterNetEvent.
         */
        if !self.registered_network_events.contains(&event_name) {
            return;
        }

        if !handler_has_arguments(call_text) {
            return;
        }

        if has_obvious_validation(call_text) {
            return;
        }

        self.report_f004(call, &event_name);
        self.check_f007(call_text, call);
    }

    fn report_f004(&mut self, call: &FunctionCall, event_name: &str) {
        if !self.reported_network_events.insert(event_name.to_owned()) {
            return;
        }

        let mut diagnostic = Diagnostic::new(
            "F004",
            Severity::Warning,
            "Potentially unsafe network event handler detected.",
            self.file,
        )
        .with_explanation(
            "This network event can be triggered by a client and receives client-controlled arguments without an obvious authorization or input validation check.",
        )
        .with_suggestion(
            "Validate the event source, permissions, and all client-controlled arguments before performing privileged operations.",
        );

        if let Some(position) = call.start_position() {
            diagnostic = diagnostic.with_location(position.line(), position.character() + 1);
        }

        self.diagnostics.push(diagnostic);
    }
    fn check_f007(&mut self, call_text: &str, call: &FunctionCall) {
        if !contains_privileged_operation(call_text) {
            return;
        }

        if has_obvious_authorization(call_text) {
            return;
        }

        let mut diagnostic = Diagnostic::new(
            "F007",
            Severity::Warning,
            "Potentially privileged network event without an obvious authorization check.",
            self.file,
        )
        .with_explanation(
            "This network event performs a potentially privileged operation, but no obvious authorization check was found.",
        )
        .with_suggestion(
            "Validate the player's permissions before performing privileged operations.",
        );

        if let Some(position) = call.start_position() {
            diagnostic = diagnostic.with_location(position.line(), position.character() + 1);
        }

        self.diagnostics.push(diagnostic);
    }
    fn check_f008(&mut self, call_text: &str, call: &FunctionCall) {
        if !has_server_event_arguments(call_text) {
            return;
        }

        let mut diagnostic = Diagnostic::new(
            "F008",
            Severity::Warning,
            "Client-controlled value is passed to a server network event.",
            self.file,
        )
        .with_explanation(
            "Values passed through TriggerServerEvent can be controlled or modified by the client and must not be trusted by the server.",
        )
        .with_suggestion(
            "Validate all client-controlled arguments on the server before using them in privileged or state-changing operations.",
        );

        if let Some(position) = call.start_position() {
            diagnostic = diagnostic.with_location(position.line(), position.character() + 1);
        }

        self.diagnostics.push(diagnostic);
    }
}

impl Visitor for LuaRuleVisitor<'_> {
    fn visit_function_call(&mut self, node: &FunctionCall) {
        self.check_function_call(node);
    }

    fn visit_while(&mut self, _node: &While) {
        self.loop_depth += 1;
    }

    fn visit_while_end(&mut self, _node: &While) {
        self.loop_depth = self.loop_depth.saturating_sub(1);
    }

    fn visit_repeat(&mut self, _node: &Repeat) {
        self.loop_depth += 1;
    }

    fn visit_repeat_end(&mut self, _node: &Repeat) {
        self.loop_depth = self.loop_depth.saturating_sub(1);
    }

    fn visit_numeric_for(&mut self, _node: &NumericFor) {
        self.loop_depth += 1;
    }

    fn visit_numeric_for_end(&mut self, _node: &NumericFor) {
        self.loop_depth = self.loop_depth.saturating_sub(1);
    }
}

fn function_name(call_text: &str) -> Option<&str> {
    let text = call_text.trim_start();

    let end = text
        .find(|character: char| character == '(' || character.is_whitespace())
        .unwrap_or(text.len());

    let name = &text[..end];

    if name.is_empty() { None } else { Some(name) }
}

fn is_zero_wait(call_text: &str) -> bool {
    let normalized: String = call_text.chars().filter(|c| !c.is_whitespace()).collect();

    normalized.starts_with("Wait(0)")
}

fn first_string_argument(call_text: &str) -> Option<String> {
    let open_paren = call_text.find('(')?;
    let args = &call_text[open_paren + 1..];

    let mut chars = args.chars().peekable();

    while let Some(character) = chars.next() {
        if character != '\'' && character != '"' {
            continue;
        }

        let quote = character;
        let mut value = String::new();

        while let Some(character) = chars.next() {
            if character == quote {
                return Some(value);
            }

            value.push(character);
        }

        return None;
    }

    None
}

fn contains_inline_handler(call_text: &str) -> bool {
    call_text.contains("function(") || call_text.contains("function (")
}

fn handler_has_arguments(call_text: &str) -> bool {
    let Some(function_start) = call_text.find("function") else {
        return false;
    };

    let after_function = &call_text[function_start + "function".len()..];

    let Some(open_paren) = after_function.find('(') else {
        return false;
    };

    let after_open = &after_function[open_paren + 1..];

    let Some(close_paren) = after_open.find(')') else {
        return false;
    };

    !after_open[..close_paren].trim().is_empty()
}

fn has_obvious_validation(call_text: &str) -> bool {
    let normalized: String = call_text.chars().filter(|c| !c.is_whitespace()).collect();

    let has_permission_check = [
        "IsPlayerAceAllowed(",
        "HasPermission(",
        ":HasPermission(",
        "hasPermission(",
    ]
    .iter()
    .any(|pattern| normalized.contains(pattern));

    let has_type_check = normalized.contains("type(");

    let has_source_guard = ["source==", "source~=", "source>", "source<", "notsource"]
        .iter()
        .any(|pattern| normalized.contains(pattern));

    let has_argument_validation =
        has_type_check && (normalized.contains("~=") || normalized.contains("=="));

    has_permission_check || has_source_guard || has_argument_validation
}

fn contains_privileged_operation(call_text: &str) -> bool {
    let normalized: String = call_text.chars().filter(|c| !c.is_whitespace()).collect();

    [
        ".Functions.AddMoney(",
        ".Functions.RemoveMoney(",
        ".Functions.SetMoney(",
        "ExecuteCommand(",
        "DropPlayer(",
    ]
    .iter()
    .any(|pattern| normalized.contains(pattern))
}

fn has_obvious_authorization(call_text: &str) -> bool {
    let normalized: String = call_text.chars().filter(|c| !c.is_whitespace()).collect();

    let has_permission_check = [
        "QBCore.Functions.HasPermission(",
        "IsPlayerAceAllowed(",
        "HasPermission(",
        ":HasPermission(",
        "hasPermission(",
    ]
    .iter()
    .any(|pattern| normalized.contains(pattern));

    let has_source_guard = ["source==", "source~=", "source>", "source<", "notsource"]
        .iter()
        .any(|pattern| normalized.contains(pattern));

    has_permission_check || has_source_guard
}

fn has_server_event_arguments(call_text: &str) -> bool {
    let Some(open_paren) = call_text.find('(') else {
        return false;
    };

    let args = &call_text[open_paren + 1..];

    let mut depth = 0usize;
    let mut quote = None;

    for character in args.chars() {
        match quote {
            Some(active_quote) => {
                if character == active_quote {
                    quote = None;
                }
            }

            None if character == '\'' || character == '"' => {
                quote = Some(character);
            }

            None if character == '(' || character == '{' || character == '[' => {
                depth += 1;
            }

            None if character == ')' || character == '}' || character == ']' => {
                if depth > 0 {
                    depth -= 1;
                } else {
                    break;
                }
            }

            None if character == ',' && depth == 0 => {
                return true;
            }

            None if !character.is_whitespace() => {
                return true;
            }

            None => {}
        }
    }

    false
}
