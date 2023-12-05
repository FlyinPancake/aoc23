# run tests for the day:

leaderboard_id := `op read "op://Personal/cqib5epgrz55ypzgh36nw5apne/leaderboard id"`

aoc-cli *args:
    #!/bin/fish
    set -x ADVENT_OF_CODE_SESSION "op://Personal/cqib5epgrz55ypzgh36nw5apne/credential"
    op run --no-masking -- aoc {{args}} 


pl:
    just aoc-cli private-leaderboard {{leaderboard_id}}


new_day day:
    #!/bin/bash
    export ADVENT_OF_CODE_SESSION="op://Personal/cqib5epgrz55ypzgh36nw5apne/credential"
    export RUST_BACKTRACE=1
    op run -- cargo run -p common --bin create_day -- --day {{day}} init

update_day day:
    #!/bin/bash
    export ADVENT_OF_CODE_SESSION="op://Personal/cqib5epgrz55ypzgh36nw5apne/credential"
    export RUST_BACKTRACE=1
    op run -- cargo run -p common --bin create_day -- --day {{day}} update-puzzle

test day phase="":
    cargo test -p day_{{day}} {{phase}} --lib

submit day phase:
    #!/bin/bash
    ADVENT_OF_CODE_SOLUTION=$(cargo run --package day_{{day}} --bin task_{{phase}} --release)
    just aoc-cli submit -d {{day}} {{phase}} "$ADVENT_OF_CODE_SOLUTION"