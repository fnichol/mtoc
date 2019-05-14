// Copyright 2019 Fletcher Nichol and/or applicable contributors.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license (see <LICENSE-MIT or
// http://opensource.org/licenses/MIT>, at your option. This file may not be copied, modified, or
// distributed except according to those terms.

use regex::Regex;

lazy_static::lazy_static! {
    static ref HTML_TAG_RE: Regex = Regex::new("</?[^>]+>").unwrap();

    // Regular expression representing characters, symbols, and punctuation marks to strip out of
    // the given text
    static ref INVALID_CHARS_RE: Regex = Regex::new(concat!(
        "[",
        // Normal ASCII characters
        r#"|$&`~=\\/@+*!?({\[\]})<>.,;:'"^%#"#,
        // CJK punctuation symbols (see: https://git.io/fjWDK)
        r#"[。？！，、；：“”【】（）〔〕［］﹃﹄“ ”‘’﹁﹂—…－～《》〈〉「」]"#,
        "]"
    ))
    .unwrap();
}

pub(crate) fn titleize<T: AsRef<str>>(text: T) -> String {
    HTML_TAG_RE
        .replace_all(text.as_ref(), "")
        .split_whitespace()
        .collect::<Vec<_>>()
        .join(" ")
}

pub(crate) fn slugify<T: AsRef<str>>(text: T) -> String {
    let slug = text.as_ref().to_lowercase().trim().replace(" ", "-");
    let slug = HTML_TAG_RE.replace_all(&slug, "");
    // TODO: strip emphasis/strong markings
    let slug = INVALID_CHARS_RE.replace_all(&slug, "");

    slug.to_string()
}

#[cfg(test)]
mod tests {
    use super::{slugify, titleize};

    macro_rules! test {
        (
            $name:ident, $title:expr, $text_exp:expr, $anchor_exp:expr
        ) => {
            #[test]
            fn $name() {
                let titleize = titleize($title);
                let slugify = slugify($title);

                assert_eq!(
                    $text_exp, titleize,
                    "titleize({:?}) != {:?} (got: {:?})",
                    $title, $text_exp, titleize
                );
                assert_eq!(
                    $anchor_exp, slugify,
                    "slugify({:?}) != {:?} (got: {:?})",
                    $title, $anchor_exp, slugify
                );
            }
        };
    }

    mod fnichol {
        use super::*;

        // Adapted from: https://github.com/jonschlinkert/markdown-toc/issues/80
        test!(
            umlauts,
            "Frachtaufträge",
            "Frachtaufträge",
            "frachtaufträge"
        );

        // Adapted from: https://github.com/jonschlinkert/markdown-toc/issues/134
        test!(c_sharp, "C#", "C#", "c");

        // TODO: determine whether or not to support some, but not all HTML tags, as in this
        // example
        //
        // Adapted from: https://github.com/ekalinin/github-markdown-toc/issues/40
        // test!(
        //     sub_sub_link,
        //     "On <sub><sub>[Go to Top](#top)</sub></sub>",
        //     "On",
        //     "on-go-to-top"
        // );

        test!(
            lowercase_diacritics,
            "Okay Åô Then",
            "Okay Åô Then",
            "okay-åô-then"
        );
    }

    // Tests were adapted from the https://github.com/jonschlinkert/markdown-toc test suite and
    // altered/updated/modified where appropriate.
    //
    // Reference:
    // https://github.com/jonschlinkert/markdown-toc/blob/4ba79a2948f2a2ca472a103916a0530e6acb26f9/test/test.js
    mod jonschlinkert {
        use super::*;

        test!(
            strip_forward_slashes_in_slugs,
            "Some/Article",
            "Some/Article",
            "somearticle"
        );
        test!(
            strip_backticks_in_slugs,
            "Some`Article`",
            "Some`Article`",
            "somearticle"
        );
        test!(
            strip_cjk_punctuation_in_slugs,
            "存在，【中文】；《标点》、符号！的标题？",
            "存在，【中文】；《标点》、符号！的标题？",
            "存在中文标点符号的标题"
        );
        test!(
            strip_ampersands_in_slugs,
            "Foo & Bar",
            "Foo & Bar",
            "foo--bar"
        );
        test!(
            dont_escape_cjk_characters_in_slugs_1,
            "中文",
            "中文",
            "中文"
        );
        test!(
            escape_cjk_characters_in_slugs_2,
            "かんじ",
            "かんじ",
            "かんじ"
        );
        test!(
            dont_escape_cjk_characters_in_slugs_3,
            "한자",
            "한자",
            "한자"
        );
        test!(strip_html_tags_from_headings_1, "<test>Foo", "Foo", "foo");
        test!(strip_html_tags_from_headings_2, "<test> Foo", "Foo", "-foo");
        test!(
            strip_html_tags_from_headings_3,
            "<test> Foo ",
            "Foo",
            "-foo"
        );
        test!(
            strip_html_tags_from_headings_4,
            "<div> Foo </div>",
            "Foo",
            "-foo-"
        );
        test!(
            strip_html_tags_from_headings_5,
            " Foo <test>",
            "Foo",
            "foo-"
        );
        test!(
            condense_spaces_in_text,
            "Some    Article",
            "Some Article",
            "some----article"
        );
        test!(
            replace_spaces_in_links_with_dashes_1,
            "Foo - bar",
            "Foo - bar",
            "foo---bar"
        );
        test!(
            replace_spaces_in_links_with_dashes_2,
            "Foo- - -bar",
            "Foo- - -bar",
            "foo-----bar"
        );
        test!(
            replace_spaces_in_links_with_dashes_3,
            "Foo---bar",
            "Foo---bar",
            "foo---bar"
        );
        test!(
            replace_spaces_in_links_with_dashes_4,
            "Foo- - -bar",
            "Foo- - -bar",
            "foo-----bar"
        );
        test!(
            replace_spaces_in_links_with_dashes_5,
            "Foo- -   -bar",
            "Foo- - -bar",
            "foo-------bar"
        );
    }

    // Tests were adapted from the https://github.com/sebdah/markdown-toc test suite and
    // altered/updated/modified where appropriate.
    //
    // Reference:
    // https://github.com/sebdah/markdown-toc/blob/3bb461875c34a519e84499bdc32798f297c4cd3c/toc/slugify_test.go
    mod sebdah {
        use super::*;

        test!(
            applies_lower_case,
            "MysTrInghEre",
            "MysTrInghEre",
            "mystringhere"
        );
        test!(
            replace_space_with_dash,
            "Some ex ample",
            "Some ex ample",
            "some-ex-ample"
        );
        test!(
            drop_parens,
            "Header (something)",
            "Header (something)",
            "header-something"
        );
        test!(
            drop_brackets,
            "Header [something]",
            "Header [something]",
            "header-something"
        );
        test!(
            drop_braces,
            "Header {something}",
            "Header {something}",
            "header-something"
        );
        test!(
            drop_backslash,
            r#"Header "something""#,
            r#"Header "something""#,
            "header-something"
        );
        test!(
            drop_single_quote,
            "Header 'something'",
            "Header 'something'",
            "header-something"
        );
        test!(
            drop_backtick,
            "Header `something`",
            "Header `something`",
            "header-something"
        );
        test!(
            drop_period,
            "Header .something.",
            "Header .something.",
            "header-something"
        );
        test!(
            drop_exclamation_mark,
            "Header !something!",
            "Header !something!",
            "header-something"
        );
        test!(
            drop_tilde,
            "Header ~something~",
            "Header ~something~",
            "header-something"
        );
        test!(
            drop_ampersand,
            "Header &something&",
            "Header &something&",
            "header-something"
        );
        test!(
            drop_percent,
            "Header %something%",
            "Header %something%",
            "header-something"
        );
        test!(
            drop_circumflex,
            "Header ^something^",
            "Header ^something^",
            "header-something"
        );
        test!(
            drop_asterisk,
            "Header *something*",
            "Header *something*",
            "header-something"
        );
        test!(
            drop_hash,
            "Header #something#",
            "Header #something#",
            "header-something"
        );
        test!(
            drop_at,
            "Header @something@",
            "Header @something@",
            "header-something"
        );
        test!(
            drop_pipe,
            "Header |something|",
            "Header |something|",
            "header-something"
        );
    }

    // Tests were adapted from the https://github.com/naokazuterada/MarkdownTOC test suite and
    // altered/updated/modified where appropriate.
    //
    // https://github.com/naokazuterada/MarkdownTOC/blob/37f7d3af179250116e06e09c58ce83c91b7d3d16/tests/default.py
    // https://github.com/naokazuterada/MarkdownTOC/blob/37f7d3af179250116e06e09c58ce83c91b7d3d16/tests/github_flavored_markdown.py
    mod naokazuterada {
        use super::*;

        test!(
            spaces_in_atx_heading,
            "      Heading 1",
            "Heading 1",
            "heading-1"
        );
        test!(replace_chars_1, "Heading ! 0", "Heading ! 0", "heading--0");
        test!(replace_chars_2, "Heading # 1", "Heading # 1", "heading--1");
        test!(
            replace_chars_3,
            "Heading !! 2",
            "Heading !! 2",
            "heading--2"
        );
        test!(
            replace_chars_4,
            "Heading &and&and& 3",
            "Heading &and&and& 3",
            "heading-andand-3"
        );
        // TODO: support escaped HTML output as GitHub appears to support this
        //
        // test!(
        //     replace_chars_5,
        //     "&lt;element1>",
        //     "&lt;element1>",
        //     "element1"
        // );
        // test!(
        //     replace_chars_6,
        //     "&#60;element2>",
        //     "&#60;element2>",
        //     "element2"
        // );
        test!(
            no_escape_in_code_with_links_1,
            "`function(param, [optional])`",
            "`function(param, [optional])`",
            "functionparam-optional"
        );
        test!(
            no_escape_in_code_with_links_2,
            "(a static function) `greet([name])` (original, right?)",
            "(a static function) `greet([name])` (original, right?)",
            "a-static-function-greetname-original-right"
        );
        test!(
            no_escape_in_code_with_links_3,
            "`add(keys, command[, args][, context])`",
            "`add(keys, command[, args][, context])`",
            "addkeys-command-args-context"
        );
        test!(
            no_escape_in_code_with_links_4,
            "`get_context(key[, operator][, operand][, match_all])`",
            "`get_context(key[, operator][, operand][, match_all])`",
            "get_contextkey-operator-operand-match_all"
        );
        test!(
            underscores_asterisks_head_1,
            "_x test 1",
            "_x test 1",
            "_x-test-1"
        );
        // TODO: support emphasis/strong sequences in the title text which would more fully support
        // GitHub's extended Markdown format
        //
        // test!(
        //     underscores_asterisks_head_2,
        //     "_x_ test 2",
        //     "_x_ test 2",
        //     "x-test-2"
        // );
        test!(
            underscores_asterisks_head_3,
            "*x* test 3",
            "*x* test 3",
            "x-test-3"
        );
        test!(
            underscores_asterisks_head_4,
            "_x _ test 4",
            "_x _ test 4",
            "_x-_-test-4"
        );
        test!(
            underscores_asterisks_head_5,
            "*x * test 5",
            "*x * test 5",
            "x--test-5"
        );
        test!(
            underscores_asterisks_head_6,
            "_ x_ test 6",
            "_ x_ test 6",
            "_-x_-test-6"
        );
        test!(
            underscores_asterisks_head_7,
            "* x* test 7",
            "* x* test 7",
            "-x-test-7"
        );
        // TODO: support emphasis/strong sequences in the title text which would more fully support
        // GitHub's extended Markdown format
        //
        // test!(
        //     underscores_asterisks_head_8,
        //     "__x__ test 8",
        //     "__x__ test 8",
        //     "x-test-8"
        // );
        test!(
            underscores_asterisks_head_9,
            "**x** test 9",
            "**x** test 9",
            "x-test-9"
        );
        test!(
            underscores_asterisks_head_10,
            "__x __ test 10",
            "__x __ test 10",
            "__x-__-test-10"
        );
        test!(
            underscores_asterisks_head_11,
            "**x ** test 11",
            "**x ** test 11",
            "x--test-11"
        );
        test!(
            underscores_asterisks_head_12,
            "__ x__ test 12",
            "__ x__ test 12",
            "__-x__-test-12"
        );
        test!(
            underscores_asterisks_head_13,
            "** x** test 13",
            "** x** test 13",
            "-x-test-13"
        );
        test!(
            underscores_asterisks_head_14,
            "_x test 14",
            "_x test 14",
            "_x-test-14"
        );
        test!(
            underscores_asterisks_head_15,
            "x_ test 15",
            "x_ test 15",
            "x_-test-15"
        );
        test!(
            underscores_asterisks_tail_1,
            "1 test_x",
            "1 test_x",
            "1-test_x"
        );
        // TODO: support emphasis/strong sequences in the title text which would more fully support
        // GitHub's extended Markdown format
        //
        // test!(
        //     underscores_asterisks_tail_2,
        //     "2 test _x_",
        //     "2 test _x_",
        //     "2-test-x"
        // );
        test!(
            underscores_asterisks_tail_3,
            "3 test *x*",
            "3 test *x*",
            "3-test-x"
        );
        test!(
            underscores_asterisks_tail_4,
            "4 test _x _",
            "4 test _x _",
            "4-test-_x-_"
        );
        test!(
            underscores_asterisks_tail_5,
            "5 test *x *",
            "5 test *x *",
            "5-test-x-"
        );
        test!(
            underscores_asterisks_tail_6,
            "6 test _ x_",
            "6 test _ x_",
            "6-test-_-x_"
        );
        test!(
            underscores_asterisks_tail_7,
            "7 test * x*",
            "7 test * x*",
            "7-test--x"
        );
        // TODO: support emphasis/strong sequences in the title text which would more fully support
        // GitHub's extended Markdown format
        //
        // test!(
        //     underscores_asterisks_tail_8,
        //     "8 test __x__",
        //     "8 test __x__",
        //     "8-test-x"
        // );
        test!(
            underscores_asterisks_tail_9,
            "9 test **x**",
            "9 test **x**",
            "9-test-x"
        );
        test!(
            underscores_asterisks_tail_10,
            "10 test __x __",
            "10 test __x __",
            "10-test-__x-__"
        );
        test!(
            underscores_asterisks_tail_11,
            "11 test **x **",
            "11 test **x **",
            "11-test-x-"
        );
        test!(
            underscores_asterisks_tail_12,
            "12 test __ x__",
            "12 test __ x__",
            "12-test-__-x__"
        );
        test!(
            underscores_asterisks_tail_13,
            "13 test ** x**",
            "13 test ** x**",
            "13-test--x"
        );
        test!(
            underscores_asterisks_tail_14,
            "14 test _x",
            "14 test _x",
            "14-test-_x"
        );
        test!(
            underscores_asterisks_tail_15,
            "15 test x_",
            "15 test x_",
            "15-test-x_"
        );
        test!(
            underscores_asterisks_middle_1,
            "1_x test",
            "1_x test",
            "1_x-test"
        );
        // TODO: support emphasis/strong sequences in the title text which would more fully support
        // GitHub's extended Markdown format
        //
        // test!(
        //     underscores_asterisks_middle_2,
        //     "2 _x_ test",
        //     "2 _x_ test",
        //     "2-x-test"
        // );
        test!(
            underscores_asterisks_middle_3,
            "3 *x* test",
            "3 *x* test",
            "3-x-test"
        );
        test!(
            underscores_asterisks_middle_4,
            "4 _x _ test",
            "4 _x _ test",
            "4-_x-_-test"
        );
        test!(
            underscores_asterisks_middle_5,
            "5 *x * test",
            "5 *x * test",
            "5-x--test"
        );
        test!(
            underscores_asterisks_middle_6,
            "6 _ x_ test",
            "6 _ x_ test",
            "6-_-x_-test"
        );
        test!(
            underscores_asterisks_middle_7,
            "7 * x* test",
            "7 * x* test",
            "7--x-test"
        );
        // TODO: support emphasis/strong sequences in the title text which would more fully support
        // GitHub's extended Markdown format
        //
        // test!(
        //     underscores_asterisks_middle_8,
        //     "8 __x__ test",
        //     "8 __x__ test",
        //     "8-x-test"
        // );
        test!(
            underscores_asterisks_middle_9,
            "9 **x** test",
            "9 **x** test",
            "9-x-test"
        );
        test!(
            underscores_asterisks_middle_10,
            "10 __x __ test",
            "10 __x __ test",
            "10-__x-__-test"
        );
        test!(
            underscores_asterisks_middle_11,
            "11 **x ** test",
            "11 **x ** test",
            "11-x--test"
        );
        test!(
            underscores_asterisks_middle_12,
            "12 __ x__ test",
            "12 __ x__ test",
            "12-__-x__-test"
        );
        test!(
            underscores_asterisks_middle_13,
            "13 ** x** test",
            "13 ** x** test",
            "13--x-test"
        );
        test!(
            underscores_asterisks_middle_14,
            "14 _x test",
            "14 _x test",
            "14-_x-test"
        );
        test!(
            underscores_asterisks_middle_15,
            "15 x_ test",
            "15 x_ test",
            "15-x_-test"
        );
    }

    // Tests were adapted from the https://github.com/jch/html-pipeline test suite and
    // altered/updated/modified where appropriate.
    //
    // Reference:
    // https://github.com/jch/html-pipeline/blob/f1bbce4858876dc2619c61a8b18637b5d3321b1c/test/html/pipeline/toc_filter_test.rb
    mod jch {
        use super::*;

        test!(
            quotes_1,
            r#""Funky President" by James Brown"#,
            r#""Funky President" by James Brown"#,
            "funky-president-by-james-brown"
        );
        test!(
            quotes_2,
            r#""It's My Thing" by Marva Whitney"#,
            r#""It's My Thing" by Marva Whitney"#,
            "its-my-thing-by-marva-whitney"
        );
        test!(
            quotes_3,
            r#""Boogie Back" by Roy Ayers"#,
            r#""Boogie Back" by Roy Ayers"#,
            "boogie-back-by-roy-ayers"
        );
        test!(
            quotes_4,
            r#""Feel Good" by Fancy"#,
            r#""Feel Good" by Fancy"#,
            "feel-good-by-fancy"
        );
        test!(
            quotes_5,
            r#""Funky Drummer" by James Brown"#,
            r#""Funky Drummer" by James Brown"#,
            "funky-drummer-by-james-brown"
        );
        test!(
            quotes_6,
            r#""Ruthless Villain" by Eazy-E"#,
            r#""Ruthless Villain" by Eazy-E"#,
            "ruthless-villain-by-eazy-e"
        );
        test!(utf8_characters_1, "日本語", "日本語", "日本語");
        test!(
            utf8_characters_2,
            "Русский",
            "Русский",
            "русский"
        );
    }
}
