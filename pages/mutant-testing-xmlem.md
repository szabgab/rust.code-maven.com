---
title: Mutation testing the xmlem Rust crate
timestamp: 2026-06-23T21:30:01
author: szabgab
published: true
show_related: true
description:
tags:
    - mutants
    - xmlem
    - testing
---

As I am working on a project that generates HTML pages I wanted to prettify / beautify the files.
So far the best crate I found is the [xmlem](https://crates.io/crates/xmlem), however the generated
file are "too pretty". That is, every HTML element is on it own line. I wanted to have an option
to keep the result a bit tighter. So I opened an [issue](https://github.com/xmlem/xmlem/issues/14)
and the author basically gave his (her?) blessing for someone to work on it.

I thought to give it a try, but first I had to make myself familiar with the code and with the tests.

Side note: by the time I started to write this blog post someone has already committed the change.
So maybe I won't have to do anything here. On the other hand I could contribute some of my expertise
to the project (GitHub Actions, testing etc.)

As I looked at the tests I noticed that in many cases the tests seem to only print the generated
HTML/XML and don't seem to verify the result. There were several `println!` calls instead of `assert_eq!` calls.

I figured it would be an interesting thing to see the test coverage report and the mutation testing report.


## SHA

I cloned the repository and got this sha: `89b9f9fa2f729f6e95ec79638c977dfecc3b6ec1`.
I keep it here so you can check it out too to reproduce this result.

## Test coverage report

Using [cargo-tarpaulin](https://crates.io/crates/cargo-tarpaulin)

```
cargo install cargo-tarpaulin
```

```
$ cargo tarpaulin --out Html --out Stdout

|| Uncovered Lines:
|| src/display.rs: 45-47, 50-52, 55-57, 60-62, 65-67, 135, 139, 293-294, 298, 342-343, 347, 392, 492-495, 497, 537
|| src/document.rs: 25, 49, 51-52, 57, 59-60, 73-74, 77-79, 82-84, 88-92, 99-107, 112-114, 118-125, 129, 175-177, 180, 182-190, 192-193, 197-198, 200-202, 204, 208-209, 214, 217-218, 220-224, 226, 228-229, 231, 234-237, 239, 242-244, 248-253, 257-258, 261-262, 265-269, 272-275, 281, 284-286, 294-297, 301, 318-321, 325-327, 357, 364, 369, 384, 422, 425, 431-433, 450-451, 466-468, 480, 487-489, 512, 527-529, 535-540, 542-544, 556, 563-564, 566-567, 569-570, 572, 574, 576-577, 601, 625-631, 633-634, 636-638, 645-650
|| src/element.rs: 93, 96, 119, 163, 168, 170, 206, 211, 220, 279-282, 284-287, 289, 440, 486, 549-550, 554-555, 558-560, 564-565, 568, 575-576, 578-579, 581, 583, 585-586, 589, 592-593, 595-596, 598, 601-602, 604, 606-607, 610, 617-619, 633
|| src/key.rs: 23-25, 33-35, 43-45, 68, 74-77, 88-91, 95-98, 102-105, 109-112, 116-123
|| src/select.rs: 48, 54-55, 69, 73, 78-79, 104, 108, 139-140, 143-144, 150, 154-155, 158, 162-165, 171-174, 180, 188-189, 192-193, 205, 209-210, 217, 221, 229, 237, 241, 245, 250-252, 256, 261-262, 264-265, 269, 273, 276, 280-281, 284-285, 288-290, 293, 295, 299, 301, 305, 316, 322-323, 346, 355-360, 364-366
|| src/value.rs: 27, 34, 41, 48, 52-55
|| Tested/Total Lines:
|| src/display.rs: 182/212
|| src/document.rs: 145/325
|| src/element.rs: 134/188
|| src/key.rs: 14/52
|| src/select.rs: 37/112
|| src/value.rs: 12/20
||
57.65% coverage, 524/909 lines covered
```


## Mutation testing

Using [cargo-mutants](https://crates.io/crates/cargo-mutants)

For documentation see the [mutants](https://mutants.rs/) web site.

```
cargo install cargo-mutants
```

```
$ cargo mutants
Found 337 mutants to test
ok       Unmutated baseline in 8s build + 0s test
 INFO Auto-set test timeout to 20s
MISSED   src/display.rs:46:9: replace Config::indent -> Self with Default::default() in 0s build + 0s test
MISSED   src/display.rs:51:9: replace Config::end_pad -> Self with Default::default() in 0s build + 0s test
MISSED   src/display.rs:56:9: replace Config::max_line_length -> Self with Default::default() in 0s build + 0s test
MISSED   src/display.rs:61:9: replace Config::entity_mode -> Self with Default::default() in 0s build + 0s test
MISSED   src/display.rs:66:9: replace Config::indent_text_nodes -> Self with Default::default() in 0s build + 0s test
MISSED   src/display.rs:128:9: replace <impl Print<Config, State<'_>> for Declaration>::print -> std::io::Result<()> with Ok(()) in 0s build + 0s test
MISSED   src/display.rs:222:9: replace + with * in fmt_attrs in 0s build + 0s test
MISSED   src/display.rs:221:9: replace + with * in fmt_attrs in 0s build + 0s test
MISSED   src/display.rs:223:53: replace + with * in fmt_attrs in 0s build + 0s test
MISSED   src/display.rs:223:43: replace + with * in fmt_attrs in 0s build + 0s test
MISSED   src/display.rs:223:17: replace + with * in fmt_attrs in 0s build + 0s test
MISSED   src/display.rs:226:56: replace > with == in fmt_attrs in 0s build + 0s test
MISSED   src/display.rs:226:56: replace > with < in fmt_attrs in 0s build + 0s test
MISSED   src/display.rs:226:56: replace > with >= in fmt_attrs in 0s build + 0s test
MISSED   src/display.rs:288:25: replace + with * in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:287:25: replace + with * in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 1s test
MISSED   src/display.rs:289:69: replace + with * in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 1s test
MISSED   src/display.rs:289:59: replace + with * in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:289:33: replace + with * in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:291:72: replace > with == in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:291:72: replace > with < in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:291:72: replace > with >= in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:333:28: replace match guard !attrs.is_empty() with false in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:337:21: replace + with * in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:336:21: replace + with - in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:336:21: replace + with * in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:338:65: replace + with - in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 1s test
MISSED   src/display.rs:338:65: replace + with * in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 1s test
MISSED   src/display.rs:338:55: replace + with * in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:338:29: replace + with * in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:340:53: replace && with || in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 1s test
MISSED   src/display.rs:340:68: replace > with == in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:340:68: replace > with < in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:340:68: replace > with >= in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 1s test
MISSED   src/display.rs:354:60: replace && with || in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:354:46: replace || with && in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:354:49: delete ! in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:360:49: delete ! in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:379:38: replace || with && in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:379:41: delete ! in <impl Print<Config, State<'_>> for ElementValue>::print in 0s build + 0s test
MISSED   src/display.rs:412:41: replace && with || in <impl Print<Config, State<'_>> for NodeValue>::print in 0s build + 0s test
MISSED   src/display.rs:436:41: replace && with || in <impl Print<Config, State<'_>> for NodeValue>::print in 0s build + 0s test
MISSED   src/display.rs:442:41: replace && with || in <impl Print<Config, State<'_>> for NodeValue>::print in 0s build + 0s test
MISSED   src/display.rs:482:53: replace || with && in process_entities in 0s build + 0s test
MISSED   src/display.rs:486:28: replace || with && in process_entities in 0s build + 0s test
MISSED   src/display.rs:492:49: replace match guard !is_text with true in process_entities in 0s build + 0s test
MISSED   src/display.rs:492:49: replace match guard !is_text with false in process_entities in 0s build + 0s test
MISSED   src/display.rs:493:48: replace match guard !is_text with true in process_entities in 0s build + 3s test
MISSED   src/display.rs:493:48: replace match guard !is_text with false in process_entities in 0s build + 0s test
MISSED   src/display.rs:500:28: replace match guard !ch.is_ascii_whitespace() && ch.is_ascii_control() with false in process_entities in 0s build + 0s test
MISSED   src/display.rs:492:49: delete ! in process_entities in 0s build + 0s test
MISSED   src/display.rs:493:48: delete ! in process_entities in 0s build + 0s test
MISSED   src/display.rs:508:25: replace && with || in process_entities in 0s build + 0s test
MISSED   src/display.rs:508:32: replace == with != in process_entities in 0s build + 0s test
MISSED   src/display.rs:510:33: replace && with || in process_entities in 0s build + 0s test
MISSED   src/display.rs:511:61: replace || with && in process_entities in 0s build + 0s test
MISSED   src/document.rs:83:8: delete ! in ord_elem in 0s build + 0s test
MISSED   src/document.rs:91:16: delete ! in ord_elem in 0s build + 0s test
MISSED   src/document.rs:106:12: delete ! in ord_elem in 0s build + 0s test
MISSED   src/document.rs:124:12: delete ! in ord_elem in 0s build + 0s test
MISSED   src/document.rs:176:9: replace Document::sort_nodes -> Vec<Node> with vec![] in 0s build + 1s test
MISSED   src/document.rs:176:29: replace < with == in Document::sort_nodes in 0s build + 0s test
MISSED   src/document.rs:176:29: replace < with > in Document::sort_nodes in 0s build + 0s test
MISSED   src/document.rs:176:29: replace < with <= in Document::sort_nodes in 0s build + 0s test
MISSED   src/document.rs:208:21: replace && with || in Document::sort_nodes in 0s build + 0s test
MISSED   src/document.rs:208:24: delete ! in Document::sort_nodes in 0s build + 0s test
MISSED   src/document.rs:243:9: replace Document::sort_attrs with () in 0s build + 0s test
MISSED   src/document.rs:249:9: replace Document::sort with () in 0s build + 0s test
MISSED   src/document.rs:258:9: replace Document::set_declaration with () in 0s build + 0s test
MISSED   src/document.rs:262:9: replace Document::declaration -> Option<&Declaration> with None in 0s build + 0s test
MISSED   src/document.rs:266:9: replace Document::set_doctype with () in 0s build + 0s test
MISSED   src/document.rs:295:9: replace Document::doctype -> Option<&str> with None in 0s build + 0s test
MISSED   src/document.rs:295:9: replace Document::doctype -> Option<&str> with Some("") in 0s build + 0s test
MISSED   src/document.rs:295:9: replace Document::doctype -> Option<&str> with Some("xyzzy") in 0s build + 0s test
MISSED   src/document.rs:319:9: replace Document::to_string_pretty_with_config -> String with String::new() in 0s build + 0s test
MISSED   src/document.rs:319:9: replace Document::to_string_pretty_with_config -> String with "xyzzy".into() in 0s build + 0s test
MISSED   src/document.rs:363:25: delete match arm Some(XML_VERSION_1_0) in Document::from_reader in 0s build + 0s test
MISSED   src/document.rs:366:25: delete match arm Some(XML_VERSION_1_1) in Document::from_reader in 0s build + 0s test
MISSED   src/document.rs:626:9: replace <impl fmt::Display for ReadError>::fmt -> fmt::Result with Ok(Default::default()) in 2s build + 2s test
MISSED   src/document.rs:646:9: replace <impl Error for ReadError>::source -> Option<&(dyn Error +'static)> with None in 0s build + 0s test
MISSED   src/element.rs:214:18: replace >= with < in Element::append_new_element_after in 0s build + 0s test
MISSED   src/element.rs:559:9: replace Element::display -> String with String::new() in 0s build + 0s test
MISSED   src/element.rs:559:9: replace Element::display -> String with "xyzzy".into() in 0s build + 1s test
MISSED   src/element.rs:576:9: replace Element::next_sibling_element -> Option<Element> with None in 0s build + 1s test
MISSED   src/element.rs:581:29: replace == with != in Element::next_sibling_element in 0s build + 0s test
MISSED   src/element.rs:583:15: replace += with -= in Element::next_sibling_element in 0s build + 0s test
MISSED   src/element.rs:583:15: replace += with *= in Element::next_sibling_element in 0s build + 0s test
MISSED   src/element.rs:585:18: replace < with == in Element::next_sibling_element in 0s build + 0s test
MISSED   src/element.rs:585:18: replace < with > in Element::next_sibling_element in 0s build + 0s test
MISSED   src/element.rs:585:18: replace < with <= in Element::next_sibling_element in 0s build + 0s test
MISSED   src/element.rs:593:9: replace Element::prev_sibling_element -> Option<Element> with None in 0s build + 1s test
MISSED   src/element.rs:598:29: replace == with != in Element::prev_sibling_element in 0s build + 0s test
MISSED   src/element.rs:601:18: replace == with != in Element::prev_sibling_element in 0s build + 0s test
MISSED   src/element.rs:604:15: replace -= with += in Element::prev_sibling_element in 0s build + 0s test
MISSED   src/element.rs:604:15: replace -= with /= in Element::prev_sibling_element in 0s build + 0s test
MISSED   src/element.rs:606:18: replace == with != in Element::prev_sibling_element in 0s build + 0s test
MISSED   src/element.rs:618:9: replace Element::query_selector_all -> Vec<Element> with vec![] in 0s build + 0s test
TIMEOUT  src/element.rs:644:15: replace += with *= in walk_tree in 0s build + 20s test
MISSED   src/key.rs:24:9: replace CDataSection::as_str -> &'d str with "" in 0s build + 0s test
MISSED   src/key.rs:24:9: replace CDataSection::as_str -> &'d str with "xyzzy" in 0s build + 0s test
MISSED   src/key.rs:34:9: replace ProcessingInstruction::as_str -> &'d str with "" in 0s build + 0s test
MISSED   src/key.rs:34:9: replace ProcessingInstruction::as_str -> &'d str with "xyzzy" in 0s build + 0s test
MISSED   src/key.rs:44:9: replace Comment::as_str -> &'d str with "" in 0s build + 0s test
MISSED   src/key.rs:44:9: replace Comment::as_str -> &'d str with "xyzzy" in 0s build + 0s test
MISSED   src/key.rs:75:9: replace Node::as_text -> Option<Text> with None in 0s build + 0s test
MISSED   src/key.rs:76:13: delete match arm Node::Text(e) in Node::as_text in 0s build + 0s test
MISSED   src/key.rs:89:9: replace Node::as_document_type -> Option<DocumentType> with None in 0s build + 0s test
MISSED   src/key.rs:90:13: delete match arm Node::DocumentType(e) in Node::as_document_type in 0s build + 0s test
MISSED   src/key.rs:96:9: replace Node::as_cdata -> Option<CDataSection> with None in 0s build + 0s test
MISSED   src/key.rs:97:13: delete match arm Node::CDataSection(e) in Node::as_cdata in 0s build + 0s test
MISSED   src/key.rs:103:9: replace Node::as_comment -> Option<Comment> with None in 0s build + 0s test
MISSED   src/key.rs:104:13: delete match arm Node::Comment(e) in Node::as_comment in 0s build + 0s test
MISSED   src/key.rs:110:9: replace Node::as_processing_instruction -> Option<ProcessingInstruction> with None in 0s build + 0s test
MISSED   src/key.rs:111:13: delete match arm Node::ProcessingInstruction(e) in Node::as_processing_instruction in 0s build + 0s test
MISSED   src/key.rs:117:9: replace Node::to_ordinal -> u8 with 0 in 0s build + 0s test
MISSED   src/key.rs:117:9: replace Node::to_ordinal -> u8 with 1 in 0s build + 0s test
MISSED   src/select.rs:42:5: replace hash -> u32 with 0 in 0s build + 0s test
MISSED   src/select.rs:42:5: replace hash -> u32 with 1 in 0s build + 0s test
MISSED   src/select.rs:49:9: replace <impl Borrow<String> for PrehashedString>::borrow -> &String with Box::leak(Box::new(String::new())) in 0s build + 0s test
MISSED   src/select.rs:49:9: replace <impl Borrow<String> for PrehashedString>::borrow -> &String with Box::leak(Box::new("xyzzy".into())) in 0s build + 0s test
MISSED   src/select.rs:55:9: replace <impl PrecomputedHash for PrehashedString>::precomputed_hash -> u32 with 0 in 0s build + 0s test
MISSED   src/select.rs:55:9: replace <impl PrecomputedHash for PrehashedString>::precomputed_hash -> u32 with 1 in 0s build + 0s test
MISSED   src/select.rs:73:9: replace <impl cssparser::ToCss for Value>::to_css -> std::fmt::Result with Ok(Default::default()) in 0s build + 0s test
MISSED   src/select.rs:79:9: replace <impl PrecomputedHash for Value>::precomputed_hash -> u32 with 0 in 0s build + 0s test
MISSED   src/select.rs:79:9: replace <impl PrecomputedHash for Value>::precomputed_hash -> u32 with 1 in 0s build + 0s test
MISSED   src/select.rs:105:9: replace <impl NonTSPseudoClass for Value>::is_active_or_hover -> bool with true in 0s build + 0s test
MISSED   src/select.rs:109:9: replace <impl NonTSPseudoClass for Value>::is_user_action_state -> bool with true in 0s build + 0s test
MISSED   src/select.rs:144:9: replace <impl selectors::Element for ElementRef<'_>>::parent_element -> Option<Self> with None in 0s build + 0s test
MISSED   src/select.rs:151:9: replace <impl selectors::Element for ElementRef<'_>>::parent_node_is_shadow_root -> bool with true in 0s build + 0s test
MISSED   src/select.rs:159:9: replace <impl selectors::Element for ElementRef<'_>>::is_pseudo_element -> bool with true in 0s build + 0s test
MISSED   src/select.rs:163:9: replace <impl selectors::Element for ElementRef<'_>>::prev_sibling_element -> Option<Self> with None in 0s build + 0s test
MISSED   src/select.rs:172:9: replace <impl selectors::Element for ElementRef<'_>>::next_sibling_element -> Option<Self> with None in 0s build + 0s test
MISSED   src/select.rs:181:9: replace <impl selectors::Element for ElementRef<'_>>::is_html_element_in_html_document -> bool with true in 0s build + 0s test
MISSED   src/select.rs:185:9: replace <impl selectors::Element for ElementRef<'_>>::has_local_name -> bool with true in 0s build + 0s test
MISSED   src/select.rs:189:9: replace <impl selectors::Element for ElementRef<'_>>::has_namespace -> bool with true in 0s build + 0s test
MISSED   src/select.rs:189:9: replace <impl selectors::Element for ElementRef<'_>>::has_namespace -> bool with false in 0s build + 0s test
MISSED   src/select.rs:189:44: replace == with != in <impl selectors::Element for ElementRef<'_>>::has_namespace in 0s build + 0s test
MISSED   src/select.rs:193:9: replace <impl selectors::Element for ElementRef<'_>>::is_same_type -> bool with true in 0s build + 0s test
MISSED   src/select.rs:193:9: replace <impl selectors::Element for ElementRef<'_>>::is_same_type -> bool with false in 0s build + 1s test
MISSED   src/select.rs:193:42: replace == with != in <impl selectors::Element for ElementRef<'_>>::is_same_type in 0s build + 0s test
MISSED   src/select.rs:202:9: replace <impl selectors::Element for ElementRef<'_>>::attr_matches -> bool with true in 0s build + 0s test
MISSED   src/select.rs:206:50: replace match guard ns.as_str() == "" with true in <impl selectors::Element for ElementRef<'_>>::attr_matches in 0s build + 0s test
MISSED   src/select.rs:226:9: replace <impl selectors::Element for ElementRef<'_>>::match_non_ts_pseudo_class -> bool with true in 0s build + 1s test
MISSED   src/select.rs:234:9: replace <impl selectors::Element for ElementRef<'_>>::match_pseudo_element -> bool with true in 0s build + 0s test
MISSED   src/select.rs:238:9: replace <impl selectors::Element for ElementRef<'_>>::is_link -> bool with true in 0s build + 0s test
MISSED   src/select.rs:242:9: replace <impl selectors::Element for ElementRef<'_>>::is_html_slot_element -> bool with true in 0s build + 0s test
MISSED   src/select.rs:250:9: replace <impl selectors::Element for ElementRef<'_>>::has_id -> bool with true in 0s build + 0s test
MISSED   src/select.rs:250:9: replace <impl selectors::Element for ElementRef<'_>>::has_id -> bool with false in 0s build + 0s test
MISSED   src/select.rs:261:9: replace <impl selectors::Element for ElementRef<'_>>::has_class -> bool with true in 0s build + 0s test
MISSED   src/select.rs:261:9: replace <impl selectors::Element for ElementRef<'_>>::has_class -> bool with false in 0s build + 0s test
MISSED   src/select.rs:273:9: replace <impl selectors::Element for ElementRef<'_>>::imported_part -> Option<<Self::Impl as SelectorImpl>::Identifier> with Some(Default::default()) in 0s build + 0s test
MISSED   src/select.rs:277:9: replace <impl selectors::Element for ElementRef<'_>>::is_part -> bool with true in 0s build + 0s test
MISSED   src/select.rs:281:9: replace <impl selectors::Element for ElementRef<'_>>::is_empty -> bool with true in 0s build + 0s test
MISSED   src/select.rs:281:9: replace <impl selectors::Element for ElementRef<'_>>::is_empty -> bool with false in 0s build + 0s test
MISSED   src/select.rs:285:9: replace <impl selectors::Element for ElementRef<'_>>::is_root -> bool with true in 0s build + 0s test
MISSED   src/select.rs:285:9: replace <impl selectors::Element for ElementRef<'_>>::is_root -> bool with false in 0s build + 0s test
MISSED   src/select.rs:285:30: replace == with != in <impl selectors::Element for ElementRef<'_>>::is_root in 0s build + 0s test
MISSED   src/select.rs:289:9: replace <impl selectors::Element for ElementRef<'_>>::first_element_child -> Option<Self> with None in 0s build + 0s test
MISSED   src/select.rs:302:9: replace <impl selectors::Element for ElementRef<'_>>::has_custom_state -> bool with true in 0s build + 0s test
MISSED   src/select.rs:306:9: replace <impl selectors::Element for ElementRef<'_>>::add_element_unique_hashes -> bool with true in 0s build + 0s test
MISSED   src/select.rs:321:9: replace <impl Parser<'i> for TheParser>::parse_non_ts_pseudo_class -> Result<Value, ParseError<'i, SelectorParseErrorKind<'i>>> with Ok(Default::default()) in 0s build + 0s test
MISSED   src/select.rs:353:9: replace Selector::matches -> bool with true in 0s build + 0s test
MISSED   src/value.rs:53:9: replace NodeValue::as_document_type -> Option<&str> with None in 0s build + 0s test
MISSED   src/value.rs:53:9: replace NodeValue::as_document_type -> Option<&str> with Some("") in 0s build + 0s test
MISSED   src/value.rs:53:9: replace NodeValue::as_document_type -> Option<&str> with Some("xyzzy") in 0s build + 0s test
MISSED   src/value.rs:54:13: delete match arm NodeValue::DocumentType(e) in NodeValue::as_document_type in 0s build + 0s test
337 mutants tested in 6m: 165 missed, 118 caught, 53 unviable, 1 timeouts
```

The 165 missed mutants indicate that there are lots of places the code could be changed and the test would not catch the regression.

## Convert `println!` to `assert_eq!`

As I mentioned, there were many test cases where at the end of the test the content of the generated document was printed,
but not verified. So I changed all those cases. I don't know much about this crate so I could not personally verify
the correctness of the results. So first I changed the `println` to compare the result to an empty string. This made the test
fail printing the actual result. Then I took the current actual result and set it as the expected string.

This will ensure that changes to the code don't lead to unplanned changes in the output.

Later someone who wants to manually verify the correctness of the tests can do so.

Here is an example of the changes I made. (I'll link to them if they are merged into the code-base.)

```
-        println!("{:#}", doc);
+        assert_eq!("<root>\n  potato\n</root>\n", format!("{:#}", doc));
```

## Running mutation testing again

```
cargo mutants
...
337 mutants tested in 5m: 141 missed, 142 caught, 53 unviable, 1 timeouts
```

There are still 141 missed cases, but that's already 24 less than what we had earlier.


## Conclusion

Given that the test coverage is "only" 57.65% this might not be such a great example showing that "high test coverage might not mean the code is really tested",
but still converting those `println!` macros to `assert_eq!` macros reduced the missed cases by 24.

We need to increase the test coverage in order to make sure the code does not accidently breaks in one place when some changes are introduced in another place.
At the same time we also need to monitor the reports from the mutant tests to make sure the tests indeed verify the behavior.

Unrelated note: What would be really nice if the two reports could be combined and it would emphasize the statements that were covered by the test coverage,
but still don't fail the tests if they are changed.

