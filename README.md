Coffeelint parser
=================

This is a simple parser for parsing the coffeelint files that CircleCI stores on AWS after coffeelint runs. The format is not particulary machine readable, so we have to do some janky stuff to get the data into a useful format.

Built for Catalyst. If you need this for your own project, please ping me so I can whip it into a more useful shape for you.

This takes the ultra raw stuff directly out of coffeelint. As in, pipe your coffeelint result to a file and load it into a string. Pass that string into `identify_lint_errors` and you're good to go.
