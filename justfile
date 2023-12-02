# run tests for the day:

leaderboard_id := `op read "op://Personal/cqib5epgrz55ypzgh36nw5apne/leaderboard id"`

r day phase:
    cargo run --release -p day_{{day}} -- --file day_{{day}}/tests/test_{{phase}}.txt

# solve for the day:
s day: 
    cargo run --release -p day_{{day}} -- --file day_{{day}}/tests/day_{{day}}.txt

aoc-cli *args:
    #!/bin/fish
    set -x ADVENT_OF_CODE_SESSION "op://Personal/cqib5epgrz55ypzgh36nw5apne/credential"
    op run -- aoc {{args}} 


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

test day:
    cargo test -p day_{{day}}