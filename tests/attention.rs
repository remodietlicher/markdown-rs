extern crate micromark;
use micromark::{micromark, micromark_with_options, Options};

const DANGER: &Options = &Options {
    allow_dangerous_html: true,
    allow_dangerous_protocol: true,
    default_line_ending: None,
};

#[test]
fn attention() {
    // Rule 1.
    assert_eq!(
        micromark("*foo bar*"),
        "<p><em>foo bar</em></p>",
        "should support emphasis w/ `*`"
    );

    assert_eq!(
        micromark("a * foo bar*"),
        "<p>a * foo bar*</p>",
        "should not support emphasis if the opening is not left flanking (1)"
    );

    assert_eq!(
        micromark("a*\"foo\"*"),
        "<p>a*&quot;foo&quot;*</p>",
        "should not support emphasis if the opening is not left flanking (2b)"
    );

    assert_eq!(
        micromark("* a *"),
        "<p>* a *</p>",
        "should not support emphasis unicode whitespace either"
    );

    assert_eq!(
        micromark("foo*bar*"),
        "<p>foo<em>bar</em></p>",
        "should support intraword emphasis w/ `*` (1)"
    );

    assert_eq!(
        micromark("5*6*78"),
        "<p>5<em>6</em>78</p>",
        "should support intraword emphasis w/ `*` (2)"
    );

    // Rule 2.
    assert_eq!(
        micromark("_foo bar_"),
        "<p><em>foo bar</em></p>",
        "should support emphasis w/ `_`"
    );

    assert_eq!(
        micromark("_ foo bar_"),
        "<p>_ foo bar_</p>",
        "should not support emphasis if the opening is followed by whitespace"
    );

    assert_eq!(
        micromark("a_\"foo\"_"),
        "<p>a_&quot;foo&quot;_</p>",
        "should not support emphasis if the opening is preceded by something else and followed by punctuation"
    );

    assert_eq!(
        micromark("foo_bar_"),
        "<p>foo_bar_</p>",
        "should not support intraword emphasis (1)"
    );

    assert_eq!(
        micromark("5_6_78"),
        "<p>5_6_78</p>",
        "should not support intraword emphasis (2)"
    );

    assert_eq!(
        micromark("пристаням_стремятся_"),
        "<p>пристаням_стремятся_</p>",
        "should not support intraword emphasis (3)"
    );

    assert_eq!(
       micromark("aa_\"bb\"_cc"),
        "<p>aa_&quot;bb&quot;_cc</p>",
        "should not support emphasis if the opening is right flanking and the closing is left flanking"
    );

    assert_eq!(
        micromark("foo-_(bar)_"),
        "<p>foo-<em>(bar)</em></p>",
        "should support emphasis if the opening is both left and right flanking, if it’s preceded by punctuation"
    );

    // Rule 3.
    assert_eq!(
        micromark("_foo*"),
        "<p>_foo*</p>",
        "should not support emphasis if opening and closing markers don’t match"
    );

    assert_eq!(
        micromark("*foo bar *"),
        "<p>*foo bar *</p>",
        "should not support emphasis w/ `*` if the closing markers are preceded by whitespace"
    );

    assert_eq!(
        micromark("*foo bar\n*"),
        "<p>*foo bar\n*</p>",
        "should not support emphasis w/ `*` if the closing markers are preceded by a line break (also whitespace)"
    );

    assert_eq!(
        micromark("*(*foo)"),
        "<p>*(*foo)</p>",
        "should not support emphasis w/ `*` if the closing markers are not right flanking"
    );

    assert_eq!(
        micromark("*(*foo*)*"),
        "<p><em>(<em>foo</em>)</em></p>",
        "should support nested emphasis"
    );

    // Rule 4.
    assert_eq!(
        micromark("_foo bar _"),
        "<p>_foo bar _</p>",
        "should not support emphasis if the closing `_` is preceded by whitespace"
    );

    assert_eq!(
        micromark("_(_foo)"),
        "<p>_(_foo)</p>",
        "should not support emphasis w/ `_` if the closing markers are not right flanking"
    );

    assert_eq!(
        micromark("_(_foo_)_"),
        "<p><em>(<em>foo</em>)</em></p>",
        "should support nested emphasis w/ `_`"
    );

    assert_eq!(
        micromark("_foo_bar"),
        "<p>_foo_bar</p>",
        "should not support intraword emphasis w/ `_` (1)"
    );

    assert_eq!(
        micromark("_пристаням_стремятся"),
        "<p>_пристаням_стремятся</p>",
        "should not support intraword emphasis w/ `_` (2)"
    );

    assert_eq!(
        micromark("_foo_bar_baz_"),
        "<p><em>foo_bar_baz</em></p>",
        "should not support intraword emphasis w/ `_` (3)"
    );

    assert_eq!(
        micromark("_(bar)_."),
        "<p><em>(bar)</em>.</p>",
        "should support emphasis if the opening is both left and right flanking, if it’s followed by punctuation"
    );

    // Rule 5.
    assert_eq!(
        micromark("**foo bar**"),
        "<p><strong>foo bar</strong></p>",
        "should support strong emphasis"
    );

    assert_eq!(
        micromark("** foo bar**"),
        "<p>** foo bar**</p>",
        "should not support strong emphasis if the opening is followed by whitespace"
    );

    assert_eq!(
    micromark("a**\"foo\"**"),
    "<p>a**&quot;foo&quot;**</p>",
    "should not support strong emphasis if the opening is preceded by something else and followed by punctuation"
  );

    assert_eq!(
        micromark("foo**bar**"),
        "<p>foo<strong>bar</strong></p>",
        "should support strong intraword emphasis"
    );

    // Rule 6.
    assert_eq!(
        micromark("__foo bar__"),
        "<p><strong>foo bar</strong></p>",
        "should support strong emphasis w/ `_`"
    );

    assert_eq!(
        micromark("__ foo bar__"),
        "<p>__ foo bar__</p>",
        "should not support strong emphasis if the opening is followed by whitespace"
    );

    assert_eq!(
        micromark("__\nfoo bar__"),
        "<p>__\nfoo bar__</p>",
        "should not support strong emphasis if the opening is followed by a line ending (also whitespace)"
    );

    assert_eq!(
        micromark("a__\"foo\"__"),
        "<p>a__&quot;foo&quot;__</p>",
        "should not support strong emphasis if the opening is preceded by something else and followed by punctuation"
    );

    assert_eq!(
        micromark("foo__bar__"),
        "<p>foo__bar__</p>",
        "should not support strong intraword emphasis w/ `_` (1)"
    );

    assert_eq!(
        micromark("5__6__78"),
        "<p>5__6__78</p>",
        "should not support strong intraword emphasis w/ `_` (2)"
    );

    assert_eq!(
        micromark("пристаням__стремятся__"),
        "<p>пристаням__стремятся__</p>",
        "should not support strong intraword emphasis w/ `_` (3)"
    );

    assert_eq!(
        micromark("__foo, __bar__, baz__"),
        "<p><strong>foo, <strong>bar</strong>, baz</strong></p>",
        "should support nested strong emphasis"
    );

    assert_eq!(
        micromark("foo-__(bar)__"),
        "<p>foo-<strong>(bar)</strong></p>",
        "should support strong emphasis if the opening is both left and right flanking, if it’s preceded by punctuation"
    );

    // Rule 7.
    assert_eq!(
        micromark("**foo bar **"),
        "<p>**foo bar **</p>",
        "should not support strong emphasis w/ `*` if the closing is preceded by whitespace"
    );

    assert_eq!(
        micromark("**(**foo)"),
        "<p>**(**foo)</p>",
        "should not support strong emphasis w/ `*` if the closing is preceded by punctuation and followed by something else"
    );

    assert_eq!(
        micromark("*(**foo**)*"),
        "<p><em>(<strong>foo</strong>)</em></p>",
        "should support strong emphasis in emphasis"
    );

    assert_eq!(
        micromark(
            "**Gomphocarpus (*Gomphocarpus physocarpus*, syn.\n*Asclepias physocarpa*)**"
        ),
        "<p><strong>Gomphocarpus (<em>Gomphocarpus physocarpus</em>, syn.\n<em>Asclepias physocarpa</em>)</strong></p>",
        "should support emphasis in strong emphasis (1)"
    );

    assert_eq!(
        micromark("**foo \"*bar*\" foo**"),
        "<p><strong>foo &quot;<em>bar</em>&quot; foo</strong></p>",
        "should support emphasis in strong emphasis (2)"
    );

    assert_eq!(
        micromark("**foo**bar"),
        "<p><strong>foo</strong>bar</p>",
        "should support strong intraword emphasis"
    );

    // Rule 8.
    assert_eq!(
        micromark("__foo bar __"),
        "<p>__foo bar __</p>",
        "should not support strong emphasis w/ `_` if the closing is preceded by whitespace"
    );

    assert_eq!(
        micromark("__(__foo)"),
        "<p>__(__foo)</p>",
        "should not support strong emphasis w/ `_` if the closing is preceded by punctuation and followed by something else"
    );

    assert_eq!(
        micromark("_(__foo__)_"),
        "<p><em>(<strong>foo</strong>)</em></p>",
        "should support strong emphasis w/ `_` in emphasis"
    );

    assert_eq!(
        micromark("__foo__bar"),
        "<p>__foo__bar</p>",
        "should not support strong intraword emphasis w/ `_` (1)"
    );

    assert_eq!(
        micromark("__пристаням__стремятся"),
        "<p>__пристаням__стремятся</p>",
        "should not support strong intraword emphasis w/ `_` (2)"
    );

    assert_eq!(
        micromark("__foo__bar__baz__"),
        "<p><strong>foo__bar__baz</strong></p>",
        "should not support strong intraword emphasis w/ `_` (3)"
    );

    assert_eq!(
        micromark("__(bar)__."),
        "<p><strong>(bar)</strong>.</p>",
        "should support strong emphasis if the opening is both left and right flanking, if it’s followed by punctuation"
    );

    // Rule 9.
    assert_eq!(
        micromark("*foo [bar](/url)*"),
        "<p><em>foo <a href=\"/url\">bar</a></em></p>",
        "should support content in emphasis"
    );

    assert_eq!(
        micromark("*foo\nbar*"),
        "<p><em>foo\nbar</em></p>",
        "should support line endings in emphasis"
    );

    assert_eq!(
        micromark("_foo __bar__ baz_"),
        "<p><em>foo <strong>bar</strong> baz</em></p>",
        "should support nesting emphasis and strong (1)"
    );

    assert_eq!(
        micromark("_foo _bar_ baz_"),
        "<p><em>foo <em>bar</em> baz</em></p>",
        "should support nesting emphasis and strong (2)"
    );

    assert_eq!(
        micromark("__foo_ bar_"),
        "<p><em><em>foo</em> bar</em></p>",
        "should support nesting emphasis and strong (3)"
    );

    assert_eq!(
        micromark("*foo *bar**"),
        "<p><em>foo <em>bar</em></em></p>",
        "should support nesting emphasis and strong (4)"
    );

    assert_eq!(
        micromark("*foo **bar** baz*"),
        "<p><em>foo <strong>bar</strong> baz</em></p>",
        "should support nesting emphasis and strong (5)"
    );

    assert_eq!(
        micromark("*foo**bar**baz*"),
        "<p><em>foo<strong>bar</strong>baz</em></p>",
        "should support nesting emphasis and strong (6)"
    );

    assert_eq!(
        micromark("*foo**bar*"),
        "<p><em>foo**bar</em></p>",
        "should not support adjacent emphasis in certain cases"
    );

    assert_eq!(
        micromark("***foo** bar*"),
        "<p><em><strong>foo</strong> bar</em></p>",
        "complex (1)"
    );
    assert_eq!(
        micromark("*foo **bar***"),
        "<p><em>foo <strong>bar</strong></em></p>",
        "complex (2)"
    );
    assert_eq!(
        micromark("*foo**bar***"),
        "<p><em>foo<strong>bar</strong></em></p>",
        "complex (3)"
    );

    assert_eq!(
        micromark("foo***bar***baz"),
        "<p>foo<em><strong>bar</strong></em>baz</p>",
        "complex (a)"
    );
    assert_eq!(
        micromark("foo******bar*********baz"),
        "<p>foo<strong><strong><strong>bar</strong></strong></strong>***baz</p>",
        "complex (b)"
    );

    assert_eq!(
        micromark("*foo **bar *baz* bim** bop*"),
        "<p><em>foo <strong>bar <em>baz</em> bim</strong> bop</em></p>",
        "should support indefinite nesting of emphasis (1)"
    );

    assert_eq!(
        micromark("*foo [*bar*](/url)*"),
        "<p><em>foo <a href=\"/url\"><em>bar</em></a></em></p>",
        "should support indefinite nesting of emphasis (2)"
    );

    assert_eq!(
        micromark("** is not an empty emphasis"),
        "<p>** is not an empty emphasis</p>",
        "should not support empty emphasis"
    );

    assert_eq!(
        micromark("**** is not an empty emphasis"),
        "<p>**** is not an empty emphasis</p>",
        "should not support empty strong emphasis"
    );

    // Rule 10.
    assert_eq!(
        micromark("**foo [bar](/url)**"),
        "<p><strong>foo <a href=\"/url\">bar</a></strong></p>",
        "should support content in strong emphasis"
    );

    assert_eq!(
        micromark("**foo\nbar**"),
        "<p><strong>foo\nbar</strong></p>",
        "should support line endings in emphasis"
    );

    assert_eq!(
        micromark("__foo _bar_ baz__"),
        "<p><strong>foo <em>bar</em> baz</strong></p>",
        "should support nesting emphasis and strong (1)"
    );

    assert_eq!(
        micromark("__foo __bar__ baz__"),
        "<p><strong>foo <strong>bar</strong> baz</strong></p>",
        "should support nesting emphasis and strong (2)"
    );

    assert_eq!(
        micromark("____foo__ bar__"),
        "<p><strong><strong>foo</strong> bar</strong></p>",
        "should support nesting emphasis and strong (3)"
    );

    assert_eq!(
        micromark("**foo **bar****"),
        "<p><strong>foo <strong>bar</strong></strong></p>",
        "should support nesting emphasis and strong (4)"
    );

    assert_eq!(
        micromark("**foo *bar* baz**"),
        "<p><strong>foo <em>bar</em> baz</strong></p>",
        "should support nesting emphasis and strong (5)"
    );

    assert_eq!(
        micromark("**foo*bar*baz**"),
        "<p><strong>foo<em>bar</em>baz</strong></p>",
        "should support nesting emphasis and strong (6)"
    );

    assert_eq!(
        micromark("***foo* bar**"),
        "<p><strong><em>foo</em> bar</strong></p>",
        "should support nesting emphasis and strong (7)"
    );

    assert_eq!(
        micromark("**foo *bar***"),
        "<p><strong>foo <em>bar</em></strong></p>",
        "should support nesting emphasis and strong (8)"
    );

    assert_eq!(
        micromark("**foo *bar **baz**\nbim* bop**"),
        "<p><strong>foo <em>bar <strong>baz</strong>\nbim</em> bop</strong></p>",
        "should support indefinite nesting of emphasis (1)"
    );

    assert_eq!(
        micromark("**foo [*bar*](/url)**"),
        "<p><strong>foo <a href=\"/url\"><em>bar</em></a></strong></p>",
        "should support indefinite nesting of emphasis (2)"
    );

    assert_eq!(
        micromark("__ is not an empty emphasis"),
        "<p>__ is not an empty emphasis</p>",
        "should not support empty emphasis"
    );

    assert_eq!(
        micromark("____ is not an empty emphasis"),
        "<p>____ is not an empty emphasis</p>",
        "should not support empty strong emphasis"
    );

    // Rule 11.
    assert_eq!(
        micromark("foo ***"),
        "<p>foo ***</p>",
        "should not support emphasis around the same marker"
    );

    assert_eq!(
        micromark("foo *\\**"),
        "<p>foo <em>*</em></p>",
        "should support emphasis around an escaped marker"
    );

    assert_eq!(
        micromark("foo *_*"),
        "<p>foo <em>_</em></p>",
        "should support emphasis around the other marker"
    );

    assert_eq!(
        micromark("foo *****"),
        "<p>foo *****</p>",
        "should not support strong emphasis around the same marker"
    );

    assert_eq!(
        micromark("foo **\\***"),
        "<p>foo <strong>*</strong></p>",
        "should support strong emphasis around an escaped marker"
    );

    assert_eq!(
        micromark("foo **_**"),
        "<p>foo <strong>_</strong></p>",
        "should support strong emphasis around the other marker"
    );

    assert_eq!(
        micromark("**foo*"),
        "<p>*<em>foo</em></p>",
        "should support a superfluous marker at the start of emphasis"
    );

    assert_eq!(
        micromark("*foo**"),
        "<p><em>foo</em>*</p>",
        "should support a superfluous marker at the end of emphasis"
    );

    assert_eq!(
        micromark("***foo**"),
        "<p>*<strong>foo</strong></p>",
        "should support a superfluous marker at the start of strong"
    );

    assert_eq!(
        micromark("****foo*"),
        "<p>***<em>foo</em></p>",
        "should support multiple superfluous markers at the start of strong"
    );

    assert_eq!(
        micromark("**foo***"),
        "<p><strong>foo</strong>*</p>",
        "should support a superfluous marker at the end of strong"
    );

    assert_eq!(
        micromark("*foo****"),
        "<p><em>foo</em>***</p>",
        "should support multiple superfluous markers at the end of strong"
    );

    // Rule 12.
    assert_eq!(
        micromark("foo ___"),
        "<p>foo ___</p>",
        "should not support emphasis around the same marker"
    );

    assert_eq!(
        micromark("foo _\\__"),
        "<p>foo <em>_</em></p>",
        "should support emphasis around an escaped marker"
    );

    assert_eq!(
        micromark("foo _X_"),
        "<p>foo <em>X</em></p>",
        "should support emphasis around the other marker"
    );

    assert_eq!(
        micromark("foo _____"),
        "<p>foo _____</p>",
        "should not support strong emphasis around the same marker"
    );

    assert_eq!(
        micromark("foo __\\___"),
        "<p>foo <strong>_</strong></p>",
        "should support strong emphasis around an escaped marker"
    );

    assert_eq!(
        micromark("foo __X__"),
        "<p>foo <strong>X</strong></p>",
        "should support strong emphasis around the other marker"
    );

    assert_eq!(
        micromark("__foo_"),
        "<p>_<em>foo</em></p>",
        "should support a superfluous marker at the start of emphasis"
    );

    assert_eq!(
        micromark("_foo__"),
        "<p><em>foo</em>_</p>",
        "should support a superfluous marker at the end of emphasis"
    );

    assert_eq!(
        micromark("___foo__"),
        "<p>_<strong>foo</strong></p>",
        "should support a superfluous marker at the start of strong"
    );

    assert_eq!(
        micromark("____foo_"),
        "<p>___<em>foo</em></p>",
        "should support multiple superfluous markers at the start of strong"
    );

    assert_eq!(
        micromark("__foo___"),
        "<p><strong>foo</strong>_</p>",
        "should support a superfluous marker at the end of strong"
    );

    assert_eq!(
        micromark("_foo____"),
        "<p><em>foo</em>___</p>",
        "should support multiple superfluous markers at the end of strong"
    );

    // Rule 13.
    assert_eq!(
        micromark("**foo**"),
        "<p><strong>foo</strong></p>",
        "should support strong w/ `*`"
    );

    assert_eq!(
        micromark("*_foo_*"),
        "<p><em><em>foo</em></em></p>",
        "should support emphasis directly in emphasis w/ `_` in `*`"
    );

    assert_eq!(
        micromark("__foo__"),
        "<p><strong>foo</strong></p>",
        "should support strong w/ `_`"
    );

    assert_eq!(
        micromark("_*foo*_"),
        "<p><em><em>foo</em></em></p>",
        "should support emphasis directly in emphasis w/ `*` in `_`"
    );

    assert_eq!(
        micromark("****foo****"),
        "<p><strong><strong>foo</strong></strong></p>",
        "should support strong emphasis directly in strong emphasis w/ `*`"
    );

    assert_eq!(
        micromark("____foo____"),
        "<p><strong><strong>foo</strong></strong></p>",
        "should support strong emphasis directly in strong emphasis w/ `_`"
    );

    assert_eq!(
        micromark("******foo******"),
        "<p><strong><strong><strong>foo</strong></strong></strong></p>",
        "should support indefinite strong emphasis"
    );

    // Rule 14.
    assert_eq!(
        micromark("***foo***"),
        "<p><em><strong>foo</strong></em></p>",
        "should support strong directly in emphasis w/ `*`"
    );

    assert_eq!(
        micromark("___foo___"),
        "<p><em><strong>foo</strong></em></p>",
        "should support strong directly in emphasis w/ `_`"
    );

    // Rule 15.
    assert_eq!(
        micromark("*foo _bar* baz_"),
        "<p><em>foo _bar</em> baz_</p>",
        "should not support mismatched emphasis"
    );

    assert_eq!(
        micromark("*foo __bar *baz bim__ bam*"),
        "<p><em>foo <strong>bar *baz bim</strong> bam</em></p>",
        "should not support mismatched strong emphasis"
    );

    // Rule 16.
    assert_eq!(
        micromark("**foo **bar baz**"),
        "<p>**foo <strong>bar baz</strong></p>",
        "should not shortest strong possible"
    );

    assert_eq!(
        micromark("*foo *bar baz*"),
        "<p>*foo <em>bar baz</em></p>",
        "should not shortest emphasis possible"
    );

    // Rule 17.
    assert_eq!(
        micromark("*[bar*](/url)"),
        "<p>*<a href=\"/url\">bar*</a></p>",
        "should not mismatch inside links (1)"
    );

    assert_eq!(
        micromark("_[bar_](/url)"),
        "<p>_<a href=\"/url\">bar_</a></p>",
        "should not mismatch inside links (1)"
    );

    assert_eq!(
        micromark_with_options("*<img src=\"foo\" title=\"*\"/>", DANGER),
        "<p>*<img src=\"foo\" title=\"*\"/></p>",
        "should not end inside HTML"
    );

    assert_eq!(
        micromark_with_options("*<img src=\"foo\" title=\"*\"/>", DANGER),
        "<p>*<img src=\"foo\" title=\"*\"/></p>",
        "should not end emphasis inside HTML"
    );

    assert_eq!(
        micromark_with_options("**<a href=\"**\">", DANGER),
        "<p>**<a href=\"**\"></p>",
        "should not end strong inside HTML (1)"
    );

    assert_eq!(
        micromark_with_options("__<a href=\"__\">", DANGER),
        "<p>__<a href=\"__\"></p>",
        "should not end strong inside HTML (2)"
    );

    assert_eq!(
        micromark("*a `*`*"),
        "<p><em>a <code>*</code></em></p>",
        "should not end emphasis inside code (1)"
    );

    assert_eq!(
        micromark("_a `_`_"),
        "<p><em>a <code>_</code></em></p>",
        "should not end emphasis inside code (2)"
    );

    assert_eq!(
        micromark("**a<http://foo.bar/?q=**>"),
        "<p>**a<a href=\"http://foo.bar/?q=**\">http://foo.bar/?q=**</a></p>",
        "should not end strong emphasis inside autolinks (1)"
    );

    assert_eq!(
        micromark("__a<http://foo.bar/?q=__>"),
        "<p>__a<a href=\"http://foo.bar/?q=__\">http://foo.bar/?q=__</a></p>",
        "should not end strong emphasis inside autolinks (2)"
    );

    // To do: turning things off.
    // assert_eq!(
    //     micromark("*a*", {extensions: [{disable: {null: ["attention"]}}]}),
    //     "<p>*a*</p>",
    //     "should support turning off attention"
    // );
}
