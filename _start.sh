#!/bin/sh

Y=2023
C=  # Insert session cookie here.
A="nikola.benes@gmail.com via curl"

if [ "$C" = "" ]; then
	echo "Missing session cookie."
	exit
fi

DAY="$1"
ZDAY=$(printf '%02d' "$1")

if [ "$DAY" = "" ]; then
	echo "Usage: $0 <day_number>"
	exit
fi

cat <<END >> Cargo.toml

[[bin]]
name = "$ZDAY"
path = "src/$ZDAY.rs"
END

cat <<END > "src/$ZDAY.rs"
use aoc::*;

fn main() {

    // println!("{:?}", x);
}
END

curl "https://adventofcode.com/$Y/day/$DAY/input" \
	--cookie "session=$C" --user-agent "$A" > "input$ZDAY"
