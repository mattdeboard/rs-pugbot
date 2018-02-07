# 0.1

Minimum viable bot.

Features:
* Administration via chat commands of all aspects of custom match configuration:
  - user enrollment in draft pool
  - random assignment of team captains
  - round-robin player drafting by team captains
  - random map selection to be used when the team captains set up the match

# 1.0

Feature-complete release.

## Features
* Administration via chat commands of all aspects of custom match configuration:
  - user enrollment in draft pool
  - random assignment of team captains
  - round-robin player drafting by team captains
  - random map selection to be used when the team captains set up the match
* Match result persistence
  - reporting of match results by team captains
  - possible feature: "call a human" when there is a dispute of any kind over match results. This may come after 1.0
* Persistent player rating tracking
  - adjust player scores after each match
  - persist those scores to the database
  - provide skill rankings so players can see "Top n" players, as well as their own position on the ladder
