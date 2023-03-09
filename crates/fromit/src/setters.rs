use derivit_core::setter::FieldSetter;

use super::*;

pub(crate) fn generate_setters(
  opts: &StructOpts,
  final_generics: &FinalGenerics,
) -> syn::Result<proc_macro2::TokenStream> {
  if opts.setters.ignore {
    return Ok(quote!());
  }
  let mut setters = Vec::new();
  let setters_prefix = opts
    .setters
    .prefix
    .as_ref()
    .cloned()
    .unwrap_or_else(|| format_ident!("set"));

  let mut ctr = 0;
  if let Some(extra) = &opts.extra {
    for field in extra.fields.values() {
      if field.setter.ignore {
        continue;
      }

      let src_name = field
        .name
        .clone()
        .unwrap_or_else(|| format_ident!("{}", ctr));
      ctr += usize::from(field.named);

      let field_name = field.name.as_ref().unwrap_or(&src_name);
      let vis = field.setter.vis.as_ref().unwrap_or_else(|| {
        opts
          .setters
          .vis_all
          .as_ref()
          .unwrap_or_else(|| field.vis.as_ref().unwrap_or(&opts.vis))
      });
      let fn_name = field
        .setter
        .rename
        .clone()
        .unwrap_or_else(|| format_ident!("{}_{}", setters_prefix, field_name));

      setters.push(FieldSetter {
        field_name: field_name.clone(),
        field_ty: field.src_ty.clone(),
        style: field.setter.style.unwrap_or(opts.setters.style),
        vis: vis.clone(),
        fn_name,
        bound: field.setter.bound.bound.clone(),
      });
    }
  }

  for (src_name, field) in opts.fields.iter() {
    if field.skip.is_some() || field.setter.ignore {
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
    let fn_name = field
      .setter
      .rename
      .clone()
      .unwrap_or_else(|| format_ident!("{}_{}", setters_prefix, field_name));

    let field_ty = field.typ.as_ref().unwrap_or(&field.src_ty);
    setters.push(FieldSetter {
      field_name: field_name.clone(),
      field_ty: field_ty.clone(),
      style: field.setter.style.unwrap_or(opts.setters.style),
      vis: vis.clone(),
      fn_name,
      bound: field.setter.bound.bound.clone(),
    });
  }

  let name = &opts.name;
  let impl_generics = &final_generics.impl_generics;
  let self_ty_generics = &final_generics.ty_generics;
  let where_clause = &final_generics.where_clause;
  Ok(quote! {
    impl #impl_generics #name #self_ty_generics #where_clause {
      #(#setters)*
    }
  })
}