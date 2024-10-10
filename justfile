default day-number: (new-day day-number)

copy-template day-number:
	cp -r .template day-{{day-number}}

set-day-number day-number: (copy-template day-number)
	find day-{{day-number}} -type f | xargs sed -i  "s/__DAY_NUMBER__/{{day-number}}/g"

add-to-workspace day-number:
	sed -i "s/^]/    \"day-{{day-number}}\",\n]/g" Cargo.toml

new-day day-number: (set-day-number day-number) (add-to-workspace day-number)

run day-number part-number:
	cargo run -p day-{{day-number}} --bin part{{part-number}} -- day-{{day-number}}/input.txt