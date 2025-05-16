use std::path::Path;
use std::fs;
use std::convert::Infallible;
use std::collections::HashMap;

use cssparser::{
    CowRcStr,
    ParserState,
    Parser,
    ParseError,
    BasicParseErrorKind,
    SourceLocation,
    match_ignore_ascii_case };
use lightningcss::{
    error::PrinterError,
    printer::Printer,
    declaration::DeclarationBlock,
    stylesheet::{ ParserOptions, PrinterOptions },
    bundler::{ Bundler, FileProvider },
    rules::{style::StyleRule, CssRule, CssRuleList, Location},
    selector::Component,
    visitor::{ Visitor, Visit, VisitTypes },
    visit_types,
    vendor_prefix::VendorPrefix,
    traits::{AtRuleParser, ToCss}
};

fn main(){
    let fs = FileProvider::new();
    let mut arp = ExtendAtRuleParser;
    let mut bundler = Bundler::new_with_at_rule_parser(&fs, None, ParserOptions::default(), &mut arp);
    let mut stylesheet = bundler.bundle(Path::new("assets/main.css")).unwrap();
        let mut style_rules = HashMap::new();
    stylesheet.visit(&mut StyleRuleCollector {rules: &mut style_rules}).unwrap();
    println!("cargo:warning={:?}", style_rules);
    stylesheet.visit(&mut Extender { rules: &style_rules }).unwrap();
        
    let res = stylesheet.to_css(PrinterOptions::default()).unwrap();
    let dest_path = Path::new("assets/bundled.css");
    fs::write(&dest_path, res.code.as_bytes()).unwrap();
}

enum Prelude {
  Extend(Vec<String>),
}

#[derive(Debug, Clone)]
struct ExtendRule {
  names: Vec<String>,
  loc: SourceLocation,
}

#[derive(Debug, Clone)]
enum AtRule {
  Extend(ExtendRule),
}

#[derive(Debug, Clone)]
struct ExtendAtRuleParser;
impl<'i> AtRuleParser<'i> for ExtendAtRuleParser {
  type Prelude = Prelude;
  type Error = Infallible;
  type AtRule = AtRule;

  fn parse_prelude<'t>(
    &mut self,
    name: CowRcStr<'i>,
    input: &mut Parser<'i, 't>,
    _options: &ParserOptions<'_, 'i>,
  ) -> Result<Self::Prelude, ParseError<'i, Self::Error>> {
    match_ignore_ascii_case! {&*name,
      "extends" => {
        let mut names = Vec::new();
        loop {
          if let Ok(name) = input.try_parse(|input| input.expect_ident_cloned()) {
            names.push(name.as_ref().into());
          } else {
            break
          }
        }

        Ok(Prelude::Extend(names))
      },
      _ => Err(input.new_error(BasicParseErrorKind::AtRuleInvalid(name)))
    }
  }

    fn rule_without_block(
        &mut self,
        prelude: Self::Prelude,
        start: &ParserState,
        _options: &ParserOptions<'_, 'i>,
        _is_nested: bool,
    ) -> Result<Self::AtRule, ()> {
        let loc = start.source_location();
        match prelude {
            Prelude::Extend(names) => Ok(AtRule::Extend(ExtendRule { names, loc })),
        }
    }
}


struct StyleRuleCollector<'i, 'a> {
    rules: &'a mut HashMap<String, DeclarationBlock<'i>>,
}

impl<'i, 'a> Visitor<'i, AtRule> for StyleRuleCollector<'i, 'a> {
    type Error = Infallible;
    
    fn visit_types(&self) -> VisitTypes {
        visit_types!(RULES)
    }
    
    fn visit_rule(&mut self, rule: &mut lightningcss::rules::CssRule<'i, AtRule>) -> Result<(), Self::Error> {
        match rule {
            CssRule::Style(rule) => {
                for selector in rule.selectors.0.iter() {
                    if selector.len() != 1 {
                        continue; // TODO
                    }
                    for component in selector.iter_raw_match_order() {
                        match component {
                            Component::Class(name) => {
                                self.rules.insert(name.0.to_string(), rule.declarations.clone());
                            }
                            _ => {}
                        }
                    }
                }
            }
            _ => {}
        }       
        rule.visit_children(self)
    }
}

struct Extender<'a, 'i> {
  rules: &'a HashMap<String, DeclarationBlock<'i>>,
}
impl<'a, 'i> Visitor<'i, AtRule> for Extender<'a, 'i> {
    type Error = Infallible;

    fn visit_types(&self) -> VisitTypes {
        visit_types!(RULES)
    }
    
    fn visit_rule(&mut self, rule: &mut CssRule<'i, AtRule>) -> Result<(), Self::Error> {
        match rule {
            CssRule::Custom(AtRule::Extend(extend)) => {
                let mut declarations = DeclarationBlock::new();
                println!("cargo:warning={:?}", extend.names);
                for name in &extend.names {
                    let Some(extended) = self.rules.get(name) else {
                        continue;
                    };
                    declarations
                        .important_declarations
                        .extend(extended.important_declarations.iter().cloned());
                    declarations.declarations.extend(extended.declarations.iter().cloned());
                }
                *rule = CssRule::Style(StyleRule {
                    selectors: Component::Nesting.into(),
                    vendor_prefix: VendorPrefix::None,
                    declarations,
                    rules: CssRuleList(vec![]),
                    loc: Location {
                        source_index: 0,
                        line: extend.loc.line,
                        column: extend.loc.column,
                    },
                })
            },
            _ => {}
        }
        rule.visit_children(self)
    }
}

impl<'i, V: Visitor<'i, AtRule>> Visit<'i, AtRule, V> for AtRule {
  const CHILD_TYPES: VisitTypes = VisitTypes::empty();

  fn visit_children(&mut self, _: &mut V) -> Result<(), V::Error> {
    Ok(())
  }
}

impl ToCss for AtRule {
  fn to_css<W: std::fmt::Write>(&self, _dest: &mut Printer<W>) -> Result<(), PrinterError> {
    match self {
      AtRule::Extend(_) => Ok(()),
    }
  }
}
