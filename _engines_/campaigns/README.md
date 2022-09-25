# campaigns

The root of this folder handles the base to run general campaigns with different names

## inputs

All inputs live in this level of the project and must be adjusted to run a seperate campaign.

## find patterns

The list of patterns are defined to operate as REGX, allowing for expressive finds on the `patterns.txt`

## custom [TODO]

In order to run a custom list of campaigns per create a campaign list within a campaign folder.

## running

You can run all the campaigns directly by using the following env var `ENGINE_FD`.

`ENGINE_FD=true cargo run -r`

**todo**

In order to run direct campaigns pass the campaign as an arg with the env variable.

You can also trigger campaigns using the webserver following the API commands.
