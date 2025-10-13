// This script builds README.md from files/README.md.template

use std::collections::{
  BTreeMap,
  BTreeSet,
};
use std::fs::{
  self,
  File,
};
use std::io::{
  self,
  Read,
  Write,
  Error,
  ErrorKind,
};

fn slice_until_template_or_end(buf: &str) -> (&str, &str) {
  for (i, c) in buf.char_indices() {
    if c == '{' {
      return (&buf[0..i], &buf[i..])
    }
  }

  (buf, &buf[buf.len()..])
}

fn slice_template_name(buf: &str) -> Result<(&str, &str), &'static str> {
  // XXX: Improvement: assert buf[0] == '{'

  for (i, c) in buf.char_indices().skip(1) {
    if c == '}' {
      return Ok((&buf[1..i], &buf[i+1..]))
    }
  }

  Err("missing closing brace")
}

fn output_solutions_table<'a>(output: &mut File) -> io::Result<()> {
  let current_dir = std::env::current_dir()?;

  let mut any_solutions = BTreeSet::new();
  let mut langs = BTreeSet::new();
  let mut solutions = BTreeMap::new();

  for entry in fs::read_dir(current_dir)? {
    let entry = entry?;
    let path = entry.path();
    
    if !path.is_file() {
      continue
    }

    let file_name_os = entry.file_name();
    let file_name = String::from(file_name_os.to_str().unwrap());
  
    if true {
      let mut prefix = "";
      let mut suffix = "";
      let mut pset = "";
      let mut lang = "";

      if file_name.starts_with("PE-") {
        prefix = "PE-";
        pset = "PE";
      }
      else
      if file_name.starts_with("UVA-") {
        prefix = "UVA-";
        pset = "UVA";
      }
      else
      if file_name.starts_with("AOC-") {
        prefix = "AOC-";
        pset = "AOC";
      }

      if file_name.ends_with(".cpp") {
        suffix = ".cpp";
        lang = "C++";
      }
      else
      if file_name.ends_with(".jl") {
        suffix = ".jl";
        lang = "Julia";
      }
      else
      if file_name.ends_with(".go") {
        suffix = ".go";
        lang = "Go";
      }
      else
      if file_name.ends_with(".py") {
        suffix = ".py";
        lang = "Python";
      }
      else
      if file_name.ends_with(".rs") {
        suffix = ".rs";
        lang = "Rust";
      }

      if pset.len() > 0 && lang.len() > 0 && prefix.len() > 0 && suffix.len() > 0 {
        let designator = String::from(file_name.strip_prefix(prefix).unwrap().strip_suffix(suffix).unwrap());

        any_solutions.insert((pset, designator.clone()));
        langs.insert(lang);
        solutions.insert((pset, designator.clone(), lang), file_name.clone());
      }
    }
  }

  output.write_all("<table>\n".as_bytes())?;
  output.write_all("<thead>\n".as_bytes())?;
  output.write_all("<tr>\n".as_bytes())?;
  for lang in &langs {
    output.write_all("<th>".as_bytes())?;
    output.write_all(lang.as_bytes())?;
    output.write_all("</th>\n".as_bytes())?;
  }
  output.write_all("</tr>\n".as_bytes())?;
  output.write_all("</thead>\n".as_bytes())?;
  output.write_all("<tbody>\n".as_bytes())?;
  for (pset, designator) in &any_solutions {
    output.write_all("<tr>\n".as_bytes())?;
    for lang in &langs {
      output.write_all("<td>".as_bytes())?;
      if let Some(path) = solutions.get(&(pset, designator.to_string(), lang)) {
        let fragment = format!("<a href=\"./{}\">{}</a>", path, path);
        output.write_all(fragment.as_bytes())?;
      }
      output.write_all("</td>\n".as_bytes())?;
    }
    output.write_all("</tr>\n".as_bytes())?;
  }
  output.write_all("</tbody>\n".as_bytes())?;
  output.write_all("</table>\n".as_bytes())?;

  Ok(())
}

fn output_evaluated_template<'a>(buf: &'a str, output: &mut File) -> io::Result<&'a str> {
  let (template_name, rest) = slice_template_name(buf)
    .map_err(|error| Error::new(ErrorKind::Other, error))?;

  if template_name == "SOLUTIONS_TABLE" {
    output_solutions_table(output)?
  } else {
    return Err(
      Error::new(ErrorKind::Other, format!("invalid template name {}", template_name))
    );
  }

  Ok(rest)
}

fn output_next_evaluated_template<'a>(buf: &'a str, output: &mut File) -> io::Result<&'a str> {
  let (content, mut rest) = slice_until_template_or_end(buf);

  if content.len() > 0 {
    output.write_all(content.as_bytes())?;
  }

  if rest.len() > 0 {
    rest = output_evaluated_template(rest, output)?;
  }

  Ok(rest)
}

fn output_with_evaluated_templates(buf: &str, output: &mut File) -> io::Result<()> {
  let mut rest = buf;
  while rest.len() > 0 {
    rest = output_next_evaluated_template(rest, output)?;
  }

  Ok(())
}

fn make_readme() -> io::Result<()> {
  let mut readme_template = File::open("./files/README.md.template")?;
  let mut buf = String::new();

  readme_template.read_to_string(&mut buf)?;

  let mut readme_output = File::create("./README.md")?;
  output_with_evaluated_templates(buf.as_str(), &mut readme_output)?;
  
  Ok(())
}

fn main() -> io::Result<()> {
  make_readme()?;

  Ok(())
}

