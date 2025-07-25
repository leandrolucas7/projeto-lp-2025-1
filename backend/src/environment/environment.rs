use crate::ir::ast::FuncSignature;
use crate::ir::ast::Function;
use crate::ir::ast::Name;
use crate::ir::ast::ValueConstructor;
use crate::{show, show_counter};
use std::collections::HashMap;
use std::collections::LinkedList;
use std::fmt::Debug;
use std::fmt::format;
use std::sync::atomic::{AtomicUsize, Ordering};
static NEXT_ENVIRONMENT_ID: AtomicUsize = AtomicUsize::new(0);

#[derive(Clone, Debug)]
pub struct Scope<A: Clone + Debug> {
    pub variables: HashMap<Name, (bool, A)>,
    pub functions: HashMap<FuncSignature, Function>,
    pub adts: HashMap<Name, Vec<ValueConstructor>>,
}

impl<A: Clone + Debug> Scope<A> {
    fn new() -> Scope<A> {
        Scope {
            variables: HashMap::new(),
            functions: HashMap::new(),
            adts: HashMap::new(),
        }
    }

    fn map_variable(&mut self, var: Name, mutable: bool, value: A) -> () {
        self.variables.insert(var, (mutable, value));
        return ();
    }

    fn map_function(&mut self, function: Function) -> () {
        let func_signature = FuncSignature::from_func(&function);
        self.functions.insert(func_signature, function);
        return ();
    }

    fn map_adt(&mut self, name: Name, adt: Vec<ValueConstructor>) -> () {
        self.adts.insert(name.clone(), adt);
        return ();
    }

    fn lookup_var(&self, var: &Name) -> Option<(bool, A)> {
        self.variables
            .get(var)
            .map(|(mutable, value)| (*mutable, value.clone()))
    }

    fn lookup_function(&self, func_signature: &FuncSignature) -> Option<&Function> {
        self.functions.get(func_signature)
    }

    fn lookup_function_by_name(&self, name: &Name) -> Option<&Function> {
        self.functions.iter().find_map(|(signature, function)| {
            if &signature.name == name {
                Some(function)
            } else {
                None
            }
        })
    }

    fn lookup_adt(&self, name: &Name) -> Option<&Vec<ValueConstructor>> {
        self.adts.get(name)
    }
}

#[derive(Clone, Debug)]
pub struct Environment<A: Clone + Debug> {
    pub id: usize,
    pub stack_len: usize,
    pub current_func: FuncSignature,
    pub output: Vec<String>,
    pub globals: Scope<A>,
    pub stack: LinkedList<Scope<A>>,
}

impl<A: Clone + Debug> Environment<A> {
    pub fn new() -> Environment<A> {
        let id = NEXT_ENVIRONMENT_ID.fetch_add(1, Ordering::Relaxed);
        show_counter_env();
        show_env(format!("New Env: {}", id));
        Environment {
            id,
            stack_len: 0,
            current_func: FuncSignature::new(),
            output: Vec::new(),
            globals: Scope::new(),
            stack: LinkedList::new(),
        }
    }

    pub fn get_globals(&self) -> Scope<A> {
        return self.globals.clone();
    }

    pub fn get_stack(&self) -> LinkedList<Scope<A>> {
        return self.stack.clone();
    }

    pub fn get_current_scope(&self) -> &Scope<A> {
        self.stack.front().unwrap_or(&self.globals)
    }

    pub fn set_stack(&mut self, stack: LinkedList<Scope<A>>) {
        self.stack_len = stack.len();
        self.stack = stack;
    }

    pub fn set_global_functions(&mut self, global_functions: HashMap<FuncSignature, Function>) {
        self.globals.functions = global_functions;
    }

    //pub fn set_stack

    pub fn set_current_func(&mut self, func_signature: &FuncSignature) {
        self.current_func = func_signature.clone();
    }

    pub fn insert_output_line(&mut self, line: &str) {
        self.output.push(line.to_string());
    }

    pub fn get_output(&mut self) -> Vec<String> {
        return self.output.clone();
    }

    //Maybe bug is happening bc this takes ownership

    pub fn map_variable(&mut self, var: Name, mutable: bool, value: A) -> () {
        let var_name = var.clone();
        match self.stack.front_mut() {
            None => self.globals.map_variable(var, mutable, value),
            Some(top) => top.map_variable(var, mutable, value),
        }
        //show_counter_env();
        //show_env(format!("Variable {} mapped to Env {}", var_name, self.id));
        //show_env(format!("{:?}", self));
    }

    pub fn create_variable(
        &mut self,
        var: Name,
        mutable: bool,
        value: A,
    ) -> Result<String, String> {
        show_counter_env();
        show_env(format!("Trying to create variable ..."));
        let current_scope = self.stack.front_mut().unwrap_or(&mut self.globals);

        for (name, _) in &current_scope.variables {
            if var == name.to_string() {
                show_env(format!(
                    "Variable '{}' was declared multiple times in function '{}'",
                    var, self.current_func
                ));
                return Err(format!(
                    "Variable '{}' was declared multiple times in function '{}'",
                    var, self.current_func
                ));
            }
        }
        current_scope.map_variable(var.clone(), mutable, value);
        show_env(format!("Variable '{}' was successfully created", var));
        show_env(format!("{:?}", self));
        return Ok("Variable successfully created".to_string());
    }

    pub fn change_variable_value(&mut self, var: Name, value: A) -> Result<String, String> {
        show_counter_env();
        show_env(format!(
            "Trying to assign value '{:?}' to variable '{}'...",
            value, var
        ));
        for scope in self.stack.iter_mut() {
            for (scope_var, (mutable, _)) in &scope.variables {
                if scope_var.to_string() == var {
                    if !mutable {
                        show_env(format!("Assignment failed because variable is immutable"));
                        return Err(format!(
                            "Variable `{}` cannot be assigned to a value because it is immutable",
                            var
                        ));
                    } else {
                        scope.map_variable(var.clone(), true, value);
                        show_env(format!("Assignment was successfull"));
                        show_env(format!("{:?}", self));
                        return Ok(format!("Assingnment of variable {} was successfull", var));
                    }
                }
            }
        }
        show_env(format!(
            "Assingment failed because variable wasn't declared"
        ));
        return Err(format!(
            "Variable '{}' was never declared in function '{}'",
            var, self.current_func
        ));
    }

    //Maybe bug is happening bc this takes ownership
    pub fn map_function(&mut self, function: Function) -> () {
        let func_name = function.name.clone();
        match self.stack.front_mut() {
            None => self.globals.map_function(function),
            Some(top) => top.map_function(function),
        }
        show_counter_env();
        show_env(format!("Function {} mapped to Env {}", func_name, self.id));
        show_env(format!("{:?}", self));
    }

    pub fn map_adt(&mut self, name: Name, cons: Vec<ValueConstructor>) -> () {
        match self.stack.front_mut() {
            None => self.globals.map_adt(name, cons),
            Some(top) => top.map_adt(name, cons),
        }
    }

    pub fn lookup(&self, var: &Name) -> Option<(bool, A)> {
        for scope in self.stack.iter() {
            if let Some(value) = scope.lookup_var(var) {
                return Some(value);
            }
        }
        self.globals.lookup_var(var)
    }

    //pub fn lookup_functions_by_name(&self, func_name: Name) -> Vec<&Function> {
    //    let mut results = Vec::new();
    //
    //    for scope in self.stack.iter() {
    //        for (signature, func) in scope.functions.iter() {
    //            if signature.name == func_name {
    //                results.push(func);
    //            }
    //        }
    //    }
    //
    //    for (signature, func) in self.globals.functions.iter() {
    //        if signature.name == func_name {
    //            results.push(func);
    //        }
    //    }
    //    results
    //}

    pub fn lookup_function(&self, func_signature: &FuncSignature) -> Option<&Function> {
        for scope in self.stack.iter() {
            if let Some(func) = scope.lookup_function(func_signature) {
                return Some(func);
            }
        }
        self.globals.lookup_function(func_signature)
    }

    pub fn lookup_var_or_func(&self, name: &Name) -> Option<FuncOrVar<A>> {
        for scope in self.stack.iter() {
            if let Some(value) = scope.lookup_var(name) {
                return Some(FuncOrVar::Var(value));
            }
            if let Some(func) = scope.lookup_function_by_name(name) {
                return Some(FuncOrVar::Func(func.clone()));
            }
        }
        if let Some(value) = self.globals.lookup_var(name) {
            return Some(FuncOrVar::Var(value));
        }
        if let Some(func) = self.globals.lookup_function_by_name(name) {
            return Some(FuncOrVar::Func(func.clone()));
        }
        return None;
    }

    pub fn lookup_adt(&self, name: &Name) -> Option<&Vec<ValueConstructor>> {
        for scope in self.stack.iter() {
            if let Some(cons) = scope.lookup_adt(name) {
                return Some(cons);
            }
        }
        self.globals.lookup_adt(name)
    }

    pub fn scoped_function(&self) -> bool {
        !self.stack.is_empty()
    }

    pub fn push(&mut self) -> () {
        self.stack.push_front(Scope::new());
        self.stack_len += 1;

        show_counter_env();
        show_env(format!("Env {} pushed:", self.id));
        show_env(format!("{:?}", self));
    }

    pub fn pop(&mut self) -> () {
        self.stack.pop_front();
        self.stack_len -= 1;

        show_counter_env();
        show_env(format!("Env {} popped:", self.id));
        show_env(format!("{:?}", self));
    }

    pub fn get_all_variables(&self) -> Vec<(Name, (bool, A))> {
        let mut vars = Vec::new();

        // First get variables from local scopes (in reverse order to respect shadowing)
        for scope in self.stack.iter() {
            for (name, value) in &scope.variables {
                if !vars.iter().any(|(n, _)| n == name) {
                    vars.push((name.clone(), value.clone()));
                }
            }
        }

        // Then get variables from global scope (if not already found)
        for (name, value) in &self.globals.variables {
            if !vars.iter().any(|(n, _)| n == name) {
                vars.push((name.clone(), value.clone()));
            }
        }

        vars
    }

    // The type checker ensures that each function is defined only once
    pub fn get_all_functions(&self) -> HashMap<FuncSignature, Function> {
        let mut all_functions = HashMap::new();
        for (func_signature, func) in &self.globals.functions {
            all_functions.insert(func_signature.clone(), func.clone());
        }
        // It is necessary to traverse the scope stack from bottom to top
        // so that functions defined in inner scopes can shadow those defined in outer scopes.
        for scope in self.stack.iter().rev() {
            for (func_signature, func) in &scope.functions {
                all_functions.insert(func_signature.clone(), func.clone());
            }
        }
        all_functions
    }
}

fn show_env(texto: String) {
    show(texto, "env.txt");
}

fn show_counter_env() {
    show_counter("env.txt");
}

pub enum FuncOrVar<A: Clone + Debug> {
    Func(Function),
    Var((bool, A)),
}

/*
#[cfg(test)]
mod tests {
    use super::*;
    use crate::ir::ast::Type;

    #[test]
    fn test_variable_scoping() {
        let mut env: Environment<i32> = Environment::new();

        // Test global scope
        env.map_variable("x".to_string(), true, 32);
        assert_eq!(Some((true, 32)), env.lookup(&"x".to_string()));

        // Test nested scopes
        env.push(); // scope 1
        env.map_variable("y".to_string(), true, 23);
        env.map_variable("x".to_string(), true, 55); // shadows global x

        env.push(); // scope 2
        env.map_variable("z".to_string(), true, 44);

        // Variables from all scopes should be accessible
        assert_eq!(Some((true, 55)), env.lookup(&"x".to_string())); // from scope 1
        assert_eq!(Some((true, 23)), env.lookup(&"y".to_string())); // from scope 1
        assert_eq!(Some((true, 44)), env.lookup(&"z".to_string())); // from scope 2

        // Pop scope 2
        env.pop();
        assert_eq!(Some((true, 55)), env.lookup(&"x".to_string())); // still in scope 1
        assert_eq!(Some((true, 23)), env.lookup(&"y".to_string())); // still in scope 1
        assert_eq!(None, env.lookup(&"z".to_string())); // z is gone

        // Pop scope 1
        env.pop();
        assert_eq!(Some((true, 32)), env.lookup(&"x".to_string())); // back to global x
        assert_eq!(None, env.lookup(&"y".to_string())); // y is gone
    }

    #[test]
    fn test_function_scoping() {
        let mut env: Environment<i32> = Environment::new();

        let global_func = Function {
            name: "global".to_string(),
            kind: Type::TVoid,
            params: Vec::new(),
            body: None,
        };

        let local_func = Function {
            name: "local".to_string(),
            kind: Type::TVoid,
            params: Vec::new(),
            body: None,
        };

        // Test function scoping
        env.map_function(global_func.clone());
        assert!(env.lookup_function(&"global".to_string()).is_some());

        env.push();
        env.map_function(local_func.clone());

        assert!(env.lookup_function(&"global".to_string()).is_some()); // can see global
        assert!(env.lookup_function(&"local".to_string()).is_some()); // can see local

        env.pop();
        assert!(env.lookup_function(&"global".to_string()).is_some()); // global still visible
        assert!(env.lookup_function(&"local".to_string()).is_none()); // local gone
    }
}
*/
