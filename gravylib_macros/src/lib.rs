use proc_macro::*;
use quote::quote;

#[proc_macro]
pub fn shader(name: TokenStream) -> TokenStream {
    let const_case = name.to_string()
        .replace([' ', '-'], "_")
        .to_uppercase();
    let snake_case = const_case.to_lowercase();
    let _pascal_case = snake_case
        .split('_')
        .map(|s| {
            let mut c = s.chars();
            match c.next() {
                None => String::new(),
                Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
            }
        })
        .collect::<String>();
    let function = proc_macro2::Ident::new(&snake_case, proc_macro2::Span::call_site());
    let module = proc_macro2::Ident::new(&snake_case, proc_macro2::Span::call_site());
    let constant = proc_macro2::Ident::new(&const_case, proc_macro2::Span::call_site());
    let code = quote! {
        mod #module;

        #[spirv(fragment)]
        pub fn #function(
            #[spirv(frag_coord)] in_frag_coord: Vec4,
            #[spirv(push_constant)] constants: &Constants,
            output: &mut Vec4,
        ) {
            let frag_coord = vec2(in_frag_coord.x, in_frag_coord.y);
            *output = #module::#function(constants, frag_coord);
        }

        #[cfg(not(target_arch = "spirv"))]
        #[allow(dead_code)]
        pub const #constant: &RawShader = &RawShader {
            shader_type: ShaderType::Pixel,
            crate_name: env!("CARGO_CRATE_NAME"),
            crate_path: env!("CARGO_MANIFEST_DIR"),
            entry_point: stringify!(#function),
        };
    };

    code.into()
}