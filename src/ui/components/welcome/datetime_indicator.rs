//! A datetime indicator for the welcome screen, including
//! a digital ASCII-art clock, and a date teller.

/// A list of ASCII art digits. To get a digit from here,
/// index it directly with the digit wanted - for example,
/// `ASCII_ART_DIGITS[0] = /* ASCII art digit for 0 */`.
pub static ASCII_ART_DIGITS: [&str; 10] = [
	r#"    /‾‾‾‾‾‾‾/
   / /‾‾‾/ / 
  / /   / /  
 / /   / /   
/  ‾‾‾‾ /    
‾‾‾‾‾‾‾‾     "#,
	r#"     /‾‾‾‾/  
     ‾‾/ /   
      / /    
     / /     
    / /      
    ‾‾       "#,
	r#"    /‾‾‾‾‾‾‾/
    ‾‾‾‾‾/ / 
  /‾‾‾‾‾‾ /  
 / /‾‾‾‾‾‾   
/  ‾‾‾‾ /    
‾‾‾‾‾‾‾‾     "#,
	r#"    /‾‾‾‾‾‾‾/
    ‾‾‾‾‾/ / 
     /‾‾  /  
     ‾‾/ /   
/‾‾‾‾‾  /    
‾‾‾‾‾‾‾‾     "#,
	r#"    /‾/   /‾/
   / /   / / 
  /  ‾‾‾  /  
  ‾‾‾‾‾/ /   
      / /    
      ‾‾     "#,
	r#"    /‾‾‾‾‾‾‾/
   / /‾‾‾‾‾  
  /  ‾‾‾‾‾/  
  ‾‾‾‾‾/ /   
/‾‾‾‾‾‾ /    
‾‾‾‾‾‾‾‾     "#,
	r#"    /‾‾‾‾‾‾‾/
   / /‾‾‾‾‾  
  /  ‾‾‾‾‾/  
 / /‾‾‾/ /   
/  ‾‾‾‾ /    
‾‾‾‾‾‾‾‾     "#,
	r#"  /‾‾‾‾‾‾‾/  
  ‾‾‾‾‾/ /   
    /‾   ‾/  
    ‾/ /‾    
    / /      
    ‾‾       "#,
	r#"    /‾‾‾‾‾‾‾/
   / /‾‾‾/ / 
  /  ‾‾‾  /  
 / /‾‾‾/ /   
/  ‾‾‾‾ /    
‾‾‾‾‾‾‾‾     "#,
	r#"    /‾‾‾‾‾‾‾/
   / /‾‾‾/ / 
  /  ‾‾‾  /  
  ‾‾‾‾‾/ /   
/‾‾‾‾‾‾ /    
‾‾‾‾‾‾‾‾    "#,
];
