# sbt-web plugins release procedure

## Prerequisites

### Github

You'll need the write access to the sbt-web plugin's Github repo.
The easiest way to do this is to be added to the sbt-web team: https://github.com/orgs/sbt/teams/sbt-web.
You can then see all the repos you can publish to (with the `playframework` sonatype user): https://github.com/orgs/sbt/teams/sbt-web/repositories

In addtion the `playframework` sonatype user can also publish releases for https://github.com/sbt/sbt-eclipse.
You have to join the `sbteclipse` team for that: https://github.com/orgs/sbt/teams/sbteclipse

### PGP key

1. You'll need to set up a PGP key so that sbt release can sign the artifacts. Signing is done with sbt-pgp.
   Read the docs there if you need more info.

### Process

1. Run `sbt release`
1. Answer all prompts with defaults - you'll need to provide your PGP passphrase too
1. After publish has finished update the project's `README.md` to show the latest version. You can do this with Github's online editor and push it.
