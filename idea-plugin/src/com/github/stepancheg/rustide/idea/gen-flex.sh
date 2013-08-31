#!/bin/sh -e

IDEA_HOME=$HOME/devel/left/idea

exec $IDEA_HOME/tools/lexer/jflex-1.4/bin/jflex --charat ./Rust.flex --nobak --skel $IDEA_HOME/tools/lexer/idea-flex.skeleton

# vim: set ts=4 sw=4 et:
