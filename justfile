# run tests for the day:
r day phase:
    cargo run --release -p day_{{day}} -- --file day_{{day}}/tests/test_{{phase}}.txt

# solve for the day:
s day: 
    cargo run --release -p day_{{day}} -- --file day_{{day}}/tests/day_{{day}}.txt
