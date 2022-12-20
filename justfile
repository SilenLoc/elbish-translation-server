@_list:
	just --list --unsorted

alias c := check

alias r := run

bt := '0'


log := "warn"

export JUST_LOG := log

check:
    cargo verify

watch:
    cargo watch -c

run:
    cargo run