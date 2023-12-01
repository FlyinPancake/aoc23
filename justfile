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

init-day:
    

pl:
    just aoc-cli private-leaderboard {{leaderboard_id}}

