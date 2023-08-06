use heck::ToPascalCase;
use proc_macro::{Span, TokenStream};
use syn::{
    parse_macro_input, spanned::Spanned, Data, DataStruct, DeriveInput, Fields, Ident, Type,
};

struct StructField {
    pub name: Ident,
    pub ty: Type,
    pub pascal: Ident,
}

/// Creates an Iden enum, makes the struct public
#[proc_macro_attribute]
pub fn entity(_args: TokenStream, input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let import_name = Ident::new("sea_query", input.span());

    // get all struct fields
    let fields = match &input.data {
        Data::Struct(DataStruct {
            fields: Fields::Named(fields),
            ..
        }) => &fields.named,
        _ => panic!("#[entity] can only be used on structs"),
    };

    // get all field names
    let fields: Vec<StructField> = fields
        .iter()
        .map(|field| {
            let ident = &field.ident;
            let string = ident
                .as_ref()
                .expect("#[enum_def] can only be used on structs with named fields")
                .to_string();
            let as_pascal = string.to_pascal_case();
            StructField {
                name: ident.as_ref().unwrap().clone(),
                ty: field.ty.clone(),
                pascal: Ident::new(as_pascal.as_str(), ident.span()),
            }
        })
        .collect();

    let struct_name = input.ident.clone();

    let enum_name = Ident::new("Iden", Span::call_site().into());

    let table_name = input.ident.to_string().to_lowercase();
    let table_name2 = table_name.clone();

    let default_names = fields.iter().map(|f| &f.name);
    let default_names2 = default_names.clone();
    let default_names3 = default_names.clone();

    let types = fields.iter().map(|f| &f.ty);

    let pascal_def_names = fields.iter().map(|f| &f.pascal);
    let pascal_def_names2 = pascal_def_names.clone();
    let pascal_def_names3 = pascal_def_names.clone();

    TokenStream::from(quote::quote! {
        #[derive(Debug, sqlx::FromRow)]
        #[allow(dead_code)]
        pub struct #struct_name {
            #(pub #default_names: #types,)*
        }
        // #input

        #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, strum::EnumIter)]
        pub enum #enum_name {
            Table,
            #(#pascal_def_names,)*
        }

        impl #import_name::Iden for #enum_name {
            fn unquoted(&self, s: &mut dyn sea_query::Write) {
                write!(s, "{}", match self {
                    #enum_name::Table => #table_name2.to_string(),
                    #(#enum_name::#pascal_def_names2 => stringify!(#default_names2).to_string()),*
                }).unwrap();
            }
        }

        impl Into<String> for #enum_name {
            fn into(self) -> String {
                match self {
                    #enum_name::Table => #table_name.to_string().to_lowercase(),
                    #(#enum_name::#pascal_def_names3 => stringify!(#default_names3).to_string().to_lowercase()),*
                }
            }
        }
    })
}
