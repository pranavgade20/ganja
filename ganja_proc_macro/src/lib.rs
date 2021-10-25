use proc_macro::{Group, Ident, TokenStream, TokenTree};

fn replace_ident(ident: Ident) -> Option<TokenTree> {
    let ident_str = ident.to_string();

    let new_str = match ident_str.as_str() {
        "गंज" => "ganja",
        "चूक" => "Err",
        "ठीक" => "Ok",
        "माळ" => "String",
        "कोश" => "HashMap",
        "Default" => "Default",
        "पर्याय" => "Option",
        "काही" => "Some",
        "नाही" => "None",
        "निकाल" => "Result",
        "छापा" => "println",
        "विराम" => "break",
        "async" => "async",
        "प्रतीक्षा" => "await",
        "वळसा" => "loop",
        "हलवा" => "move",
        "खोका" => "crate",
        "अगम्य_कोड" => "unreachable_code",
        "जसा" => "as",
        "सतत" => "const",
        "गुण" => "trait",
        "असुरक्षित" => "unsafe",
        "मधे" => "in",
        "पासून" => "from",
        "चलनशक्तिविषयक" => "dyn",
        "उघडा" => "unwrap",
        "défaut" => "default",
        "संदर्भ_म्हणून" => "as_ref",
        "पुउ" => "io",
        "बाह्य" => "extern",
        "खोटे" => "false",
        "कार्य" => "fn",
        "वरिष्ठ" => "super",
        "घाला" => "insert",
        "मिळवा" => "get",
        "अनुमत" => "allow",
        "घाबरा" => "panic",
        "भाग" => "mod",
        "परिवर्तनिय" => "mut",
        "नवा" => "new",
        "कुठे" => "where",
        "च्या_साठी" => "for",
        "मिळवा_किंवा_घाला" => "get_or_insert_with",
        "मुख्य" => "main",
        "सार्वजनिक" => "pub",
        "परता" => "return",
        "अंमलबजावणी" => "impl",
        "संदर्भ" => "ref",
        "जुळ" => "match",
        "जर" => "if",
        "इतर" => "else",
        "स्वत:" => "self",
        "द्या" => "let",
        "स्थिर" => "static",
        "रचना" => "struct",
        "अपेक्षा" => "expect",
        "असताना" => "while",
        "वापरा" => "use",
        "मध्ये" => "into",
        "खरे" => "true",
        "गणना" => "enum",

        _ => &ident_str,
    };

    let new_ident = Ident::new(new_str, ident.span());
    Some(TokenTree::Ident(new_ident))
}

fn replace_tree(tok: TokenTree, out: &mut Vec<TokenTree>) {
    match tok {
        TokenTree::Group(group) => {
            let mut group_elem = Vec::new();
            replace_stream(group.stream(), &mut group_elem);
            let mut new_stream = TokenStream::new();
            new_stream.extend(group_elem);
            out.push(TokenTree::Group(Group::new(group.delimiter(), new_stream)));
        }
        TokenTree::Ident(ident) => {
            if let Some(ident) = replace_ident(ident) {
                out.push(ident);
            }
        }
        TokenTree::Punct(..) | TokenTree::Literal(..) => {
            out.push(tok);
        }
    }
}

fn replace_stream(ts: TokenStream, out: &mut Vec<TokenTree>) {
    for tok in ts {
        replace_tree(tok, out)
    }
}

#[proc_macro]
pub fn ganja(item: TokenStream) -> TokenStream {
    let mut returned = Vec::new();
    replace_stream(item, &mut returned);
    let mut out = TokenStream::new();
    out.extend(returned);
    out
}
