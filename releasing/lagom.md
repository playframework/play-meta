# Releasing Lagom

* [Release tracking issue](#release-tracking-issue)
* [First-time Setup](#first-time-setup)
* [Review the Changes](#review-the-changes)
* [Before You Release](#before-you-release)
* [Publish the Artifacts](#publish-the-artifacts)
* [Update the Web Site](#update-the-web-site)
* [Update the g8 templates and example projects](#update-the-g8-templates-and-example-projects)
* [Announce the Release](#announce-the-release)
* [After-Release Cleanup](#after-release-cleanup)

## Release tracking issue

Create a new [release tracking issue][].

[release tracking issue]: https://github.com/playframework/play-meta/issues/new?template=z_lagom-release.md

## First-time Setup

 * Get access to vegemite
 * Read the [vegemite docs][]

[vegemite docs]: https://github.com/lightbend/vegemite/blob/master/README.md

## Review the Changes

Before doing anything else, it's a good idea to review the changes in the release branch.

On the root of the project you can find a script named `changelog.sh`. You can run it to produce a list of
commits starting from a given tag and up to head or between to tags.

    bin/changelog.sh 1.3.8         # print change log starting from tag 1.3.8 up to HEAD
    bin/changelog.sh 1.3.7 1.3.8   # print change log starting from tag 1.3.7 up to 1.3.8

This prints a list of changes reformatted to turn issue references into Markdown-formatted links to GitHub. This
makes it convenient to copy and paste when writing a changelog.  Note that `changelog.sh` will not print a valid
list if you are working on an `M1` release. You'll have to manually diff changes listed in `master` and `1.4.x`,
or most recent stable branch, but you can use changelog to create the lists to diff from.

    bin/authors.pl 1.3.7..1.3.8   # print change log starting from tag 1.3.7 up to 1.3.8

This prints the authors (without headers on the columns). You'll need this on the release notes. The
`authors.pl` is useless in M1 releases.

Some lines may be missing GitHub issue references. You'll need to decide for each one whether to find an issue
to link it to in the changelog, include it in the changelog without an issue link, or omit it from the changelog
entirely. Merge commits and automated commits that only change the version number should always be left out.

*Bonus points*: once you have a clean list of issues with the appropriate link to GitHub and issue number, use
sort -r to have the list sorted in reverse numerical order so it's easier to locate a particular issue.

### Manual verifications (optional)

If there's been changes in the build or bumped versions of sbt plugins, you may want to have a look at
[https://github.com/lagom/lagom/issues/1496#issuecomment-408398508](https://github.com/lagom/lagom/issues/1496#issuecomment-408398508)

## Before You Release

* Email Marketing to let them know about the planned release date.
    * Let them know whether it's a major, minor or patch release
    * For a patch release, a day or two of notice is sufficient
    * Minor releases will need at least a week or two of notice... we should keep them apprised of the release
      candidate process
    * Major releases will need more notice
* Check the appropriate [GitHub issues milestone](https://github.com/lagom/lagom/milestones) to be sure that
  there aren't any unresolved issues scheduled for the release, and close the milestone if it's complete
* Compare the GitHub milestone with the commits on the branch you are releasing using the instructions in
  "Reviewing the Changes" above. This way you can make sure the GH milestone lists the correct information.
* Ensure that the branch has a green build in [Travis CI](https://travis-ci.org/lagom/lagom/branches)
* Create a pull request against the project with changes for the release:
    * Write a release blog post with highlights, contributions, etc. (Use the authors list you created above)
    * Update `currentLagomVersion`
    * If this is a MAJOR or MINOR version bump (RC or final), also update `currentDocsVersion`
    * If this is a MILESTONE, update `previewVersions`
* Copy the release announcement Markdown into [draft release notes on GitHub](https://github.com/lagom/lagom/releases).

## Publish the Artifacts

ssh into `vegemite` and run:

    screen -r # try reconnecting to an existing, detached session; to force detach add -d, otherwise:
    screen sudo su - play # if you get disconnected, you can reconnect with screen -r
    deploy/release --project lagom --branch <releaseBranch>

You'll be prompted to:

* Confirm the release
* Set the release version
* Set the next SNAPSHOT version
* Confirm whether to push the updated version to GitHub (Yes)

The artifacts are deployed to the Sonatype OSS Nexus repository and automatically promoted. This will be
automatically synced to [Maven Central](https://repo1.maven.org/maven2/com/lightbend/lagom/).

Before proceeding with the public release, test the artifacts by using them in an example project. If the
artifacts are not yet in Maven Central, you can temporarily add the repository to your sbt session by adding
this to your build definition:

    // in project/plugins.sbt
    resolvers += Resolver.sonatypeRepo("releases") 
    
    // in build.sbt
    resolvers in ThisBuild += Resolver.sonatypeRepo("releases") 

*NOTE*: that Sonatype doesn't include the `lagom-sbt-plugin` which is only published to bintray. Until bintray
propagates changes you won't be able to test.

Verify the release:

* <https://bintray.com/lagom/sbt-plugin-releases/lagom-sbt-plugin/> must contain your version and be deployed using
  the **SBT_LAYOUT** (such as `scala_{versions}/sbt_{versions}/1.4.7`)
* <https://repo1.maven.org/maven2/com/lightbend/lagom/> must contain all the other artifacts for the new version.

## Update the Web Site

Merge your release pull request and deploy the updated website from `vegemite`. Check the
`~/bin/nightlies` script for the command to run.

*NOTE*: the command logs its output to `~/logs/lagom-nightly-deploy-master-1532020306.log` or similar. You need
to find the most recent file to `tail` if you can to see what is happening.

## Update the g8 templates and example projects

* Chirper (Maven & sbt):
    * <https://github.com/lagom/lagom-java-sbt-chirper-example/blob/1.5.x/project/plugins.sbt#L1>
    * <https://github.com/lagom/lagom-java-maven-chirper-example/blob/1.5.x/pom.xml#L186>
* <https://github.com/lagom/lagom-scala.g8/blob/master/src/main/g8/project/plugins.sbt>
* <https://github.com/lagom/lagom-java.g8/blob/master/src/main/g8/project/plugins.sbt>
* <https://github.com/lagom/online-auction-java/blob/1.5.x/project/plugins.sbt>
* <https://github.com/lagom/online-auction-scala/blob/1.5.x/project/plugins.sbt>
* Reactive Platform "supported modules" page (sbt plugin & artifacts):
    * <https://github.com/lightbend/reactive-platform-docs/blob/master/build.sbt#L92>
    * <https://github.com/lightbend/reactive-platform-docs/blob/master/src/main/paradox/supported-modules/index.md>
* We also have examples in <https://github.com/lagom/ibm-integration-examples> and
  <https://github.com/lagom/lagom-recipes>. We will not try to update these on every release, but may choose to do
  so on a case-by-case basis for more significant releases.

## Announce the Release

Send a link to the release blog post around internally...

* Slack #applied-play and share the post into #eng
* Email eng-updates

...and publicly

* Gitter
* https://discuss.lightbend.com (Use the blog post write up to copy/paste into a new discuss topic)
* Twitter (usually handled by marketing... be sure to let them know when the blog post is published)
* Publish release notes on GitHub

## After-Release Cleanup

* For major or minor releases, be sure to add the new version to the MiMa checks (for example, see #1298)
* If there is a new release branch, update the nightlies script on `vegemite` to ensure the documentation is deployed
    * The GitHub repository is the source of truth, so change it in a GitHub pull request first
    * After it is merged, ssh to `vegemite` and git pull the latest version
    * It's a good idea to test running the command manually rather than waiting for the next nightly run
