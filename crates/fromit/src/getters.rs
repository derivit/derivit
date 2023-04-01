use super::*;

pub(crate) fn generate_getters(
  opts: &StructOpts,
  final_generics: &FinalGenerics,
) -> syn::Result<proc_macro2::TokenStream> {
  if opts.getters.ignore {
    return Ok(quote!());
  }
  let mut ctr = 0;
  let mut getters = Vec::new();
  if let Some(extra) = &opts.extra {
    for field in extra.fields.values() {
      if field.getter.ignore {
        continue;
      }

      let src_name = field
        .name
        .clone()
        .unwrap_or_else(|| format_ident!("{}", ctr));
      ctr += usize::from(field.named);

      let field_name = field.name.as_ref().unwrap_or(&src_name);
      let vis = field.getter.vis.as_ref().unwrap_or_else(|| {
        opts
          .getters
          .vis_all
          .as_ref()
          .unwrap_or_else(|| field.vis.as_ref().unwrap_or(&opts.vis))
      });
      let fn_name = field.getter.rename.clone().unwrap_or_else(|| {
        if let Some(p) = &opts.getters.prefix {
          format_ident!("{}_{}", p, field_name)
        } else {
          field_name.clone()
        }
      });

      let style = field.getter.style.unwrap_or(opts.getters.style);
      let field_ty = &field.src_ty;

      getters.push(FieldGetter {
        field_name: field_name.clone(),
        field_ty: field_ty.clone(),
        style,
        vis: vis.clone(),
        fn_name,
        converter: field.getter.result.clone(),
      });
    }
  }

  for (src_name, field) in opts.fields.iter() {
    if field.skip.is_some() || field.getter.ignore {
      continue;
    }

    let src_name = field.rename.clone().unwrap_or_else(|| {
      if field.named {
        format_ident!("{}", src_name)
      } else {
        format_ident!("{}", ctr)
      }
    });
    ctr += usize::from(field.named);
    let field_name = field.rename.as_ref().unwrap_or(&src_name);
    let vis = field.getter.vis.as_ref().unwrap_or_else(|| {
      opts
        .getters
        .vis_all
        .as_ref()
        .unwrap_or_else(|| field.vis.as_ref().unwrap_or(&opts.vis))
    });
    let fn_name = field.getter.rename.clone().unwrap_or_else(|| {
      if let Some(p) = &opts.getters.prefix {
        format_ident!("{}_{}", p, field_name)
      } else {
        field_name.clone()
      }
    });

    let style = field.getter.style.unwrap_or(opts.getters.style);
    let field_ty = field.typ.as_ref().unwrap_or(&field.src_ty);
    getters.push(FieldGetter {
      field_name: field_name.clone(),
      field_ty: field_ty.clone(),
      style,
      vis: vis.clone(),
      fn_name,
      converter: field.getter.result.clone(),
    });
  }

  let name = &opts.name;
  let impl_generics = &final_generics.impl_generics;
  let self_ty_generics = &final_generics.ty_generics;
  let where_clause = &final_generics.where_clause;
  Ok(quote! {
    impl #impl_generics #name #self_ty_generics #where_clause {
      #(#getters)*
    }
  })
}
