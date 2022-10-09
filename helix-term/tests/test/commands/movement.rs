use super::*;

#[tokio::test(flavor = "multi_thread")]
async fn test_move_parent_node_end() -> anyhow::Result<()> {
    let tests = vec![
        // single cursor stays single cursor, first goes to end of current
        // node, then parent
        (
            indoc! {r##"
                fn foo() {
                    let result = if true {
                        "yes"
                    } else {
                        "no#["|]#
                    }
                }
            "##},
            "<A-e>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"#[\n|]#
                    }
                }
            "},
        ),
        (
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"#[\n|]#
                    }
                }
            "},
            "<A-e>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"
                    }#[\n|]#
                }
            "},
        ),
        // select mode extends
        (
            indoc! {r##"
                fn foo() {
                    let result = if true {
                        "yes"
                    } else {
                        #["no"|]#
                    }
                }
            "##},
            "v<A-e><A-e>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        #[\"no\"
                    }\n|]#
                }
            "},
        ),
    ];

    for test in tests {
        test_with_config(AppBuilder::new().with_file("foo.rs", None), test).await?;
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_move_parent_node_start() -> anyhow::Result<()> {
    let tests = vec![
        // single cursor stays single cursor, first goes to end of current
        // node, then parent
        (
            indoc! {r##"
                fn foo() {
                    let result = if true {
                        "yes"
                    } else {
                        "no#["|]#
                    }
                }
            "##},
            "<A-b>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        #[\"|]#no\"
                    }
                }
            "},
        ),
        (
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"#[\n|]#
                    }
                }
            "},
            "<A-b>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else #[{|]#
                        \"no\"
                    }
                }
            "},
        ),
        (
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else #[{|]#
                        \"no\"
                    }
                }
            "},
            "<A-b>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } #[e|]#lse {
                        \"no\"
                    }
                }
            "},
        ),
        // select mode extends
        (
            indoc! {r##"
                fn foo() {
                    let result = if true {
                        "yes"
                    } else {
                        #["no"|]#
                    }
                }
            "##},
            "v<A-b><A-b>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else #[|{
                        ]#\"no\"
                    }
                }
            "},
        ),
        (
            indoc! {r##"
                fn foo() {
                    let result = if true {
                        "yes"
                    } else {
                        #["no"|]#
                    }
                }
            "##},
            "v<A-b><A-b><A-b>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } #[|else {
                        ]#\"no\"
                    }
                }
            "},
        ),
    ];

    for test in tests {
        test_with_config(AppBuilder::new().with_file("foo.rs", None), test).await?;
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn test_smart_tab_move_parent_node_end() -> anyhow::Result<()> {
    let tests = vec![
        // single cursor stays single cursor, first goes to end of current
        // node, then parent
        (
            indoc! {r##"
                fn foo() {
                    let result = if true {
                        "yes"
                    } else {
                        "no#["|]#
                    }
                }
            "##},
            "i<tab>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"#[|\n]#
                    }
                }
            "},
        ),
        (
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"#[\n|]#
                    }
                }
            "},
            "i<tab>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"
                    }#[|\n]#
                }
            "},
        ),
        // appending to the end of a line should still look at the current
        // line, not the next one
        (
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no#[\"|]#
                    }
                }
            "},
            "a<tab>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"
                    }#[\n|]#
                }
            "},
        ),
        // before cursor is all whitespace, so insert tab
        (
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        #[\"no\"|]#
                    }
                }
            "},
            "i<tab>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                            #[|\"no\"]#
                    }
                }
            "},
        ),
        // if selection spans multiple lines, it should still only look at the
        // line on which the head is
        (
            indoc! {"\
                fn foo() {
                    let result = if true {
                        #[\"yes\"
                    } else {
                        \"no\"|]#
                    }
                }
            "},
            "a<tab>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    } else {
                        \"no\"
                    }#[\n|]#
                }
            "},
        ),
        (
            indoc! {"\
                fn foo() {
                    let result = if true {
                        #[\"yes\"
                    } else {
                        \"no\"|]#
                    }
                }
            "},
            "i<tab>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                            #[|\"yes\"
                    } else {
                        \"no\"]#
                    }
                }
            "},
        ),
        (
            indoc! {"\
                fn foo() {
                    #[l|]#et result = if true {
                        #(\"yes\"
                    } else {
                        \"no\"|)#
                    }
                }
            "},
            "i<tab>",
            indoc! {"\
                fn foo() {
                        #[|l]#et result = if true {
                            #(|\"yes\"
                    } else {
                        \"no\")#
                    }
                }
            "},
        ),
        (
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"#[\n|]#
                    } else {
                        \"no\"#(\n|)#
                    }
                }
            "},
            "i<tab>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    }#[| ]#else {
                        \"no\"
                    }#(|\n)#
                }
            "},
        ),
        (
            indoc! {"\
                fn foo() {
                    let result = if true {
                        #[\"yes\"|]#
                    } else {
                        #(\"no\"|)#
                    }
                }
            "},
            "i<tab>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                            #[|\"yes\"]#
                    } else {
                            #(|\"no\")#
                    }
                }
            "},
        ),
        // if any cursors are not preceded by all whitespace, then do the
        // smart_tab action
        (
            indoc! {"\
                fn foo() {
                    let result = if true {
                        #[\"yes\"\n|]#
                    } else {
                        \"no#(\"\n|)#
                    }
                }
            "},
            "i<tab>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                        \"yes\"
                    }#[| ]#else {
                        \"no\"
                    }#(|\n)#
                }
            "},
        ),
        // Ctrl-tab always inserts a tab
        (
            indoc! {"\
                fn foo() {
                    let result = if true {
                        #[\"yes\"\n|]#
                    } else {
                        \"no#(\"\n|)#
                    }
                }
            "},
            "i<S-tab>",
            indoc! {"\
                fn foo() {
                    let result = if true {
                            #[|\"yes\"\n]#
                    } else {
                        \"no    #(|\"\n)#
                    }
                }
            "},
        ),
    ];

    for test in tests {
        test_with_config(AppBuilder::new().with_file("foo.rs", None), test).await?;
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn expand_shrink_selection() -> anyhow::Result<()> {
    let tests = vec![
        // single range
        (
            indoc! {r##"
                Some(#[thing|]#)
            "##},
            "<A-o><A-o>",
            indoc! {r##"
                #[Some(thing)|]#
            "##},
        ),
        // multi range
        (
            indoc! {r##"
                Some(#[thing|]#)
                Some(#(other_thing|)#)
            "##},
            "<A-o>",
            indoc! {r##"
                Some#[(thing)|]#
                Some#((other_thing)|)#
            "##},
        ),
        // multi range collision merges
        (
            indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##},
            "<A-o><A-o><A-o>",
            indoc! {r##"
                #[(
                    Some(thing),
                    Some(other_thing),
                )|]#
            "##},
        ),
        // multi range collision merges, then shrinks back to original
        (
            indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##},
            "<A-o><A-o><A-o><A-i>",
            indoc! {r##"
                (
                    #[Some(thing)|]#,
                    #(Some(other_thing)|)#,
                )
            "##},
        ),
        (
            indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##},
            "<A-o><A-o><A-o><A-i><A-i>",
            indoc! {r##"
                (
                    Some#[(thing)|]#,
                    Some#((other_thing)|)#,
                )
            "##},
        ),
        (
            indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##},
            "<A-o><A-o><A-o><A-i><A-i><A-i>",
            indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##},
        ),
        // shrink with no expansion history defaults to first child
        (
            indoc! {r##"
                #[(
                    Some(thing),
                    Some(other_thing),
                )|]#
            "##},
            "<A-i>",
            indoc! {r##"
                (
                    #[Some(thing)|]#,
                    Some(other_thing),
                )
            "##},
        ),
        // any movement cancels selection history and falls back to first child
        (
            indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )

            "##},
            "<A-o><A-o><A-o>jkvkkk<A-i>",
            indoc! {r##"
                (
                    #[|Some(thing)]#,
                    Some(other_thing),
                )

            "##},
        ),
    ];

    for test in tests {
        test_with_config(AppBuilder::new().with_file("foo.rs", None), test).await?;
    }

    Ok(())
}

#[tokio::test(flavor = "multi_thread")]
async fn expand_selection_around() -> anyhow::Result<()> {
    let tests = vec![
        // single cursor stays single cursor, first goes to end of current
        // node, then parent
        (
            indoc! {r##"
                Some(#[thing|]#)
            "##},
            "<A-O><A-O>",
            indoc! {r##"
                #[Some(|]#thing#()|)#
            "##},
        ),
        // shrinking restores previous selection
        (
            indoc! {r##"
                Some(#[thing|]#)
            "##},
            "<A-O><A-O><A-i><A-i>",
            indoc! {r##"
                Some(#[thing|]#)
            "##},
        ),
        // multi range collision merges expand as normal, except with the
        // original selection removed from the result
        (
            indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##},
            "<A-O><A-O><A-O>",
            indoc! {r##"
                #[(
                    Some(|]#thing#(),
                    Some(|)#other_thing#(),
                )|)#
            "##},
        ),
        (
            indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##},
            "<A-O><A-O><A-O><A-i><A-i><A-i>",
            indoc! {r##"
                (
                    Some(#[thing|]#),
                    Some(#(other_thing|)#),
                )
            "##},
        ),
    ];

    for test in tests {
        test_with_config(AppBuilder::new().with_file("foo.rs", None), test).await?;
    }

    Ok(())
}
