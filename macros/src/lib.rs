use proc_macro::{self, TokenStream};
use quote::quote;

#[proc_macro]
pub fn init_fonts(input: TokenStream) -> TokenStream {
    let input = input.to_string();
    let mut tokens = input.split(",").collect::<Vec<&str>>();
    let folder = tokens.remove(0).trim().trim_matches('"');

    let files = &input[input.find('[').unwrap() + 1..input.find(']').unwrap()];
    let files = files.split(',').collect::<Vec<&str>>();

    let mut result = "(vec![".to_string();
    for font in &files {
        if font.trim().trim_matches('"').is_empty() {
            continue;
        }
        let path = format!("{}/{}", folder, font.trim().trim_matches('"'));
        // let path = format!("{}/{}", folder, font.);
        result.push_str(&quote! {
            fontdue::Font::from_bytes(include_bytes!(#path) as &[u8], Default::default()).unwrap(),
        }
        .to_string());
    }
    result.push_str("], HashMap::from([");

    for (i, font) in files.iter().enumerate() {
        if font.trim().trim_matches('"').is_empty() {
            continue;
        }
        let path = format!("({}.to_string(),{}),", font.trim(), i);
        result.push_str(&path);
    }

    result.push_str("]))");
    result.parse().unwrap()
}
