use crate::domain::{Sheet, CellAddress, CellValue, Formula, CellRange};
use std::collections::HashMap;

/// Service for evaluating formulas in sheets
pub struct FormulaEvaluator;

impl FormulaEvaluator {
    pub fn new() -> Self {
        Self
    }
    
    /// Evaluate a formula in the context of a sheet
    pub fn evaluate_formula(&self, formula: &Formula, sheet: &Sheet) -> Result<CellValue, String> {
        // If we have a cached value that's still valid, return it
        if let Some(cached) = &formula.cache {
            // In a real implementation, we'd check if dependencies have changed
            // For now, we'll just return the cached value
            return Ok(cached.clone());
        }
        
        // Parse and evaluate the formula expression
        self.evaluate_expression(&formula.expression, sheet)
    }
    
    /// Evaluate a formula expression
    fn evaluate_expression(&self, expression: &str, sheet: &Sheet) -> Result<CellValue, String> {
        // Remove leading '=' if present
        let expr = if expression.starts_with('=') {
            &expression[1..]
        } else {
            expression
        };
        
        // Parse the expression
        self.parse_and_evaluate(expr, sheet)
    }
    
    /// Parse and evaluate an expression
    fn parse_and_evaluate(&self, expr: &str, sheet: &Sheet) -> Result<CellValue, String> {
        // This is a simplified implementation
        // A real implementation would need a proper parser
        
        // Handle basic functions
        if expr.starts_with("SUM(") && expr.ends_with(')') {
            let range_str = &expr[4..expr.len()-1];
            return self.evaluate_sum(range_str, sheet);
        }
        
        if expr.starts_with("AVERAGE(") && expr.ends_with(')') {
            let range_str = &expr[8..expr.len()-1];
            return self.evaluate_average(range_str, sheet);
        }
        
        // Handle financial functions
        if expr.starts_with("PMT(") && expr.ends_with(')') {
            let args_str = &expr[4..expr.len()-1];
            return self.evaluate_pmt(args_str, sheet);
        }
        
        if expr.starts_with("FV(") && expr.ends_with(')') {
            let args_str = &expr[3..expr.len()-1];
            return self.evaluate_fv(args_str, sheet);
        }
        
        if expr.starts_with("NPV(") && expr.ends_with(')') {
            let args_str = &expr[4..expr.len()-1];
            return self.evaluate_npv(args_str, sheet);
        }
        
        if expr.starts_with("IRR(") && expr.ends_with(')') {
            let args_str = &expr[4..expr.len()-1];
            return self.evaluate_irr(args_str, sheet);
        }
        
        // Handle cell references
        if self.is_cell_reference(expr) {
            return self.get_cell_value(expr, sheet);
        }
        
        // Handle numbers
        if let Ok(number) = expr.parse::<f64>() {
            return Ok(CellValue::Number(number));
        }
        
        // Handle text (everything else)
        Ok(CellValue::Text(expr.to_string()))
    }
    
    /// Check if a string is a cell reference
    fn is_cell_reference(&self, s: &str) -> bool {
        // Simple check for cell references like A1, B2, etc.
        // This is a very basic implementation
        s.len() >= 2 && 
        s.chars().take_while(|c| c.is_alphabetic()).count() > 0 &&
        s.chars().skip_while(|c| c.is_alphabetic()).all(|c| c.is_numeric())
    }
    
    /// Get the value of a cell by reference
    fn get_cell_value(&self, cell_ref: &str, sheet: &Sheet) -> Result<CellValue, String> {
        let address = self.parse_cell_reference(cell_ref)?;
        if let Some(cell) = sheet.get_cell(&address) {
            Ok(cell.value.clone())
        } else {
            Ok(CellValue::Empty)
        }
    }
    
    /// Parse a cell reference like "A1" into a CellAddress
    fn parse_cell_reference(&self, cell_ref: &str) -> Result<CellAddress, String> {
        // This is a simplified implementation
        let chars: Vec<char> = cell_ref.chars().collect();
        
        // Find where letters end and numbers begin
        let letter_end = chars.iter().position(|c| c.is_numeric()).unwrap_or(chars.len());
        
        let column_letters = &chars[..letter_end];
        let row_numbers = &chars[letter_end..];
        
        // Convert column letters to number (A=1, B=2, ..., Z=26, AA=27, etc.)
        let mut column = 0u32;
        for &letter in column_letters {
            if letter.is_ascii_alphabetic() {
                column = column * 26 + (letter.to_ascii_uppercase() as u32 - 'A' as u32 + 1);
            } else {
                return Err(format!("Invalid column letter: {}", letter));
            }
        }
        column -= 1; // Convert to 0-based indexing
        
        // Convert row numbers to number
        let row_str: String = row_numbers.iter().collect();
        let row = row_str.parse::<u32>().map_err(|_| format!("Invalid row number: {}", row_str))? - 1; // Convert to 0-based indexing
        
        Ok(CellAddress::new(row, column))
    }
    
    /// Evaluate a SUM function
    fn evaluate_sum(&self, range_str: &str, sheet: &Sheet) -> Result<CellValue, String> {
        let range = self.parse_cell_range(range_str)?;
        let mut sum = 0.0;
        
        for address in range.iter() {
            if let Some(cell) = sheet.get_cell(&address) {
                if let CellValue::Number(value) = cell.value {
                    sum += value;
                }
            }
        }
        
        Ok(CellValue::Number(sum))
    }
    
    /// Evaluate an AVERAGE function
    fn evaluate_average(&self, range_str: &str, sheet: &Sheet) -> Result<CellValue, String> {
        let range = self.parse_cell_range(range_str)?;
        let mut sum = 0.0;
        let mut count = 0u32;
        
        for address in range.iter() {
            if let Some(cell) = sheet.get_cell(&address) {
                if let CellValue::Number(value) = cell.value {
                    sum += value;
                    count += 1;
                }
            }
        }
        
        if count > 0 {
            Ok(CellValue::Number(sum / count as f64))
        } else {
            Ok(CellValue::Number(0.0))
        }
    }
    
    /// Parse a cell range like "A1:B2" into a CellRange
    fn parse_cell_range(&self, range_str: &str) -> Result<CellRange, String> {
        let parts: Vec<&str> = range_str.split(':').collect();
        if parts.len() != 2 {
            return Err(format!("Invalid range format: {}", range_str));
        }
        
        let start = self.parse_cell_reference(parts[0])?;
        let end = self.parse_cell_reference(parts[1])?;
        
        Ok(CellRange::new(start, end))
    }
    
    /// Parse comma-separated arguments, handling cell references
    fn parse_function_args(&self, args_str: &str, sheet: &Sheet) -> Result<Vec<CellValue>, String> {
        let mut args = Vec::new();
        let mut current_arg = String::new();
        let mut in_quotes = false;
        let mut chars = args_str.chars().peekable();
        
        while let Some(ch) = chars.next() {
            match ch {
                '"' if !in_quotes => {
                    in_quotes = true;
                    current_arg.push(ch);
                }
                '"' if in_quotes => {
                    in_quotes = false;
                    current_arg.push(ch);
                }
                ',' if !in_quotes => {
                    // Process the current argument
                    let value = self.parse_argument_value(&current_arg.trim(), sheet)?;
                    args.push(value);
                    current_arg.clear();
                }
                _ => {
                    current_arg.push(ch);
                }
            }
        }
        
        // Process the last argument
        if !current_arg.trim().is_empty() {
            let value = self.parse_argument_value(&current_arg.trim(), sheet)?;
            args.push(value);
        }
        
        Ok(args)
    }
    
    /// Parse a single argument value, handling cell references and literals
    fn parse_argument_value(&self, arg: &str, sheet: &Sheet) -> Result<CellValue, String> {
        let trimmed = arg.trim();
        
        // Handle quoted strings
        if trimmed.starts_with('"') && trimmed.ends_with('"') && trimmed.len() >= 2 {
            return Ok(CellValue::Text(trimmed[1..trimmed.len()-1].to_string()));
        }
        
        // Handle cell references
        if self.is_cell_reference(trimmed) {
            return self.get_cell_value(trimmed, sheet);
        }
        
        // Handle numbers
        if let Ok(number) = trimmed.parse::<f64>() {
            return Ok(CellValue::Number(number));
        }
        
        // Handle boolean values
        match trimmed.to_lowercase().as_str() {
            "true" => return Ok(CellValue::Boolean(true)),
            "false" => return Ok(CellValue::Boolean(false)),
            _ => {}
        }
        
        // Treat as text if none of the above
        Ok(CellValue::Text(trimmed.to_string()))
    }
    
    /// Convert a CellValue to a number
    fn to_number(&self, value: &CellValue) -> Result<f64, String> {
        match value {
            CellValue::Number(n) => Ok(*n),
            CellValue::Text(s) => {
                s.parse::<f64>().map_err(|_| format!("Cannot convert '{}' to number", s))
            }
            CellValue::Boolean(b) => Ok(if *b { 1.0 } else { 0.0 }),
            CellValue::Empty => Ok(0.0),
            CellValue::Error(e) => Err(e.clone()),
            CellValue::DateTime(_) => Err("Cannot convert DateTime to number".to_string()),
        }
    }
    
    /// Convert a CellValue to a vector of numbers (for functions like NPV)
    fn to_vector(&self, value: &CellValue, sheet: &Sheet) -> Result<Vec<f64>, String> {
        match value {
            CellValue::Number(n) => Ok(vec![*n]),
            CellValue::Text(s) => {
                // Try to parse as a cell range
                if s.contains(':') {
                    let range = self.parse_cell_range(s)?;
                    let mut values = Vec::new();
                    for address in range.iter() {
                        if let Some(cell) = sheet.get_cell(&address) {
                            if let CellValue::Number(value) = cell.value {
                                values.push(value);
                            }
                        }
                    }
                    Ok(values)
                } else if self.is_cell_reference(s) {
                    // Single cell reference
                    let value = self.get_cell_value(s, sheet)?;
                    Ok(vec![self.to_number(&value)?])
                } else {
                    // Try to parse as a number
                    let n = s.parse::<f64>().map_err(|_| format!("Cannot convert '{}' to number", s))?;
                    Ok(vec![n])
                }
            }
            _ => Err("Cannot convert value to vector".to_string()),
        }
    }
    
    /// Financial Functions Section
    fn evaluate_pmt(&self, args_str: &str, sheet: &Sheet) -> Result<CellValue, String> {
        // Parse comma-separated arguments (rate, nper, pv, [fv], [type])
        let args = self.parse_function_args(args_str, sheet)?;
        if args.len() < 3 || args.len() > 5 {
            return Err("PMT requires 3-5 arguments: rate, nper, pv, [fv], [type]".to_string());
        }
        
        let rate = self.to_number(&args[0])?;
        let nper = self.to_number(&args[1])?;
        let pv = self.to_number(&args[2])?;
        let fv = if args.len() > 3 { self.to_number(&args[3])? } else { 0.0 };
        let payment_type = if args.len() > 4 { self.to_number(&args[4])? } else { 0.0 };
        
        // PMT calculation: pmt = (rate * (pv * (1 + rate)^nper + fv)) / ((1 + rate * payment_type) * ((1 + rate)^nper - 1))
        let pmt = self.calculate_pmt(rate, nper, pv, fv, payment_type);
        Ok(CellValue::Number(pmt))
    }
    
    fn calculate_pmt(&self, rate: f64, nper: f64, pv: f64, fv: f64, payment_type: f64) -> f64 {
        if rate == 0.0 {
            return -(pv + fv) / nper;
        }
        
        let payment_at_beginning = payment_type != 0.0;
        let rate_factor = (1.0 + rate).powf(nper);
        
        let numerator = rate * (pv * rate_factor + fv);
        let denominator = (1.0 + if payment_at_beginning { rate } else { 0.0 }) * (rate_factor - 1.0);
        
        -numerator / denominator
    }
    
    /// Evaluate FV (Future Value) function
    fn evaluate_fv(&self, args_str: &str, sheet: &Sheet) -> Result<CellValue, String> {
        // Parse comma-separated arguments (rate, nper, pmt, [pv], [type])
        let args = self.parse_function_args(args_str, sheet)?;
        if args.len() < 3 || args.len() > 5 {
            return Err("FV requires 3-5 arguments: rate, nper, pmt, [pv], [type]".to_string());
        }
        
        let rate = self.to_number(&args[0])?;
        let nper = self.to_number(&args[1])?;
        let pmt = self.to_number(&args[2])?;
        let pv = if args.len() > 3 { self.to_number(&args[3])? } else { 0.0 };
        let payment_type = if args.len() > 4 { self.to_number(&args[4])? } else { 0.0 };
        
        // FV calculation
        let fv = self.calculate_fv(rate, nper, pmt, pv, payment_type);
        Ok(CellValue::Number(fv))
    }
    
    fn calculate_fv(&self, rate: f64, nper: f64, pmt: f64, pv: f64, payment_type: f64) -> f64 {
        if rate == 0.0 {
            return -pv - pmt * nper;
        }
        
        let payment_at_beginning = payment_type != 0.0;
        let rate_factor = (1.0 + rate).powf(nper);
        
        let fv_pv = pv * rate_factor;
        let fv_pmt = if payment_at_beginning {
            pmt * (1.0 + rate) * (rate_factor - 1.0) / rate
        } else {
            pmt * (rate_factor - 1.0) / rate
        };
        
        -(fv_pv + fv_pmt)
    }
    
    /// Evaluate NPV (Net Present Value) function
    fn evaluate_npv(&self, args_str: &str, sheet: &Sheet) -> Result<CellValue, String> {
        // Parse comma-separated arguments (rate, value1, [value2], ...)
        let args = self.parse_function_args(args_str, sheet)?;
        if args.len() < 2 {
            return Err("NPV requires at least 2 arguments: rate, value1, [value2], ...".to_string());
        }
        
        let rate = self.to_number(&args[0])?;
        if rate == -1.0 {
            return Err("Rate cannot be -100%".to_string());
        }
        
        let mut npv = 0.0;
        for (i, arg) in args[1..].iter().enumerate() {
            let value = self.to_number(arg)?;
            npv += value / (1.0 + rate).powf((i + 1) as f64);
        }
        
        Ok(CellValue::Number(npv))
    }
    
    /// Evaluate IRR (Internal Rate of Return) function
    fn evaluate_irr(&self, args_str: &str, sheet: &Sheet) -> Result<CellValue, String> {
        // Parse comma-separated arguments (values, [guess])
        let args = self.parse_function_args(args_str, sheet)?;
        if args.is_empty() {
            return Err("IRR requires at least 1 argument: values, [guess]".to_string());
        }
        
        // Convert all arguments to a vector of cash flows
        let mut cash_flows = Vec::new();
        for arg in &args {
            let values = self.to_vector(arg, sheet)?;
            cash_flows.extend(values);
        }
        
        if cash_flows.is_empty() {
            return Err("No cash flows provided".to_string());
        }
        
        // IRR calculation using Newton-Raphson method
        let irr = self.calculate_irr(&cash_flows)?;
        Ok(CellValue::Number(irr))
    }
    
    fn calculate_irr(&self, cash_flows: &[f64]) -> Result<f64, String> {
        if cash_flows.is_empty() {
            return Err("No cash flows provided".to_string());
        }
        
        // Initial guess
        let mut rate = 0.1; // 10%
        let tolerance = 1e-10;
        let max_iterations = 100;
        
        for _ in 0..max_iterations {
            let mut npv = 0.0;
            let mut npv_derivative = 0.0;
            
            for (i, &cash_flow) in cash_flows.iter().enumerate() {
                let time = i as f64;
                let discount_factor = (1.0 + rate).powf(time);
                
                npv += cash_flow / discount_factor;
                
                if i > 0 {
                    npv_derivative -= cash_flow * time / (discount_factor * (1.0 + rate));
                }
            }
            
            if npv_derivative.abs() < tolerance {
                break;
            }
            
            let rate_change = npv / npv_derivative;
            rate -= rate_change;
            
            if rate_change.abs() < tolerance {
                break;
            }
        }
        
        Ok(rate)
    }
}