# sbt-web plugin release procedure

## Prerequisites

### Github

1. You'll need the write access to the sbt-web plugin's Github repo.
2. The easiest way to do this is to be added to the sbt-web team: https://github.com/orgs/sbt/teams/sbt-web

### !! OUTDATED !! Bintray is closed since May 1st 2021

1. You'll need an account in bintray.org
1. You'll need to be a member of the sbt-web organisation on Bintray
    * To get access, contact one of the owners - James Roper
    * You'll get an invitation through Bintray's internal messaging system which you'll need to accept
1. Your project will need a location to publish to on Bintray under the sbt-web organisation. E.g.
   https://bintray.com/sbt-web/sbt-plugin-releases/sbt-coffeescript
1. It will need to be linked from the sbt organisation on Bintray - You'll need to email
   sbt-repo-admins@googlegroups.com to get this sorted out. E.g. see the Linked to section on this page:
   https://bintray.com/sbt-web/sbt-plugin-releases/sbt-coffeescript. It links to
   https://bintray.com/sbt/sbt-plugin-releases?filterByPkgName=sbt-coffeescript.
1. Ignore the "imported" Bintray repos like
   https://bintray.com/sbt/sbt-plugin-releases/sbt-coffeescript-imported. I think these come from when we moved
   to Bintray from Artifactory.

### PGP key

1. You'll need to set up a PGP key so that sbt release can sign the artifacts. Signing is done with sbt-pgp.
   Read the docs there if you need more info.

### Process

1. Run sbt release
1. Answer all prompts with defaults - you'll need to provide your PGP passphrase too
1. After publish has finished update the project's README.md to show the latest version. You can do this with
   Github's online editor and push th
