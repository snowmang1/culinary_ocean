#! /bin/bash

if [[ $1 == "dev" ]]
then
	# fast dev build
	npx tailwind -c ./tailwind.config.js -o output.css
	# NEEDS ERROR CHECKING
fi

if [[ $1 ==  "build" ]]
then
	# production build, for optimizing tailwinds
	NODE_ENV=production tailwindcss -c ./tailwind.config.js -o ./tailwind.css --minify
fi

if [[ $1 == "serve" ]]
then
	# this will provide an inclusive hotreload for yew + tailwindcss
	# using trunk
	cargo watch -s "bash tailcomp.sh dev"
fi
