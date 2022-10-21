# Play Release Procedure

- [Play Release Procedure](#play-release-procedure)
  - [Before the release](#before-the-release)
    - [Issues and pull request triage](#issues-and-pull-request-triage)
  - [Release tracking issue](#release-tracking-issue)
  - [Intro](#intro)
  - [Prerequisites](#prerequisites)
  - [If something goes wrong](#if-something-goes-wrong)
  - [Releasing Play](#releasing-play)
    - [Step 0 - Release projects that Play depend on (play-json, play-ws, twirl)](#step-0---release-projects-that-play-depends-on-play-json-play-ws-twirl)
    - [Step 1 - Release Play itself](#step-1---release-play-itself)
    - [Step 2 - Release external modules](#step-2---release-external-modules)
    - [Step 3 - Release omnidoc](#step-3---release-omnidoc)
    - [Step 4 - Update `play-samples`](#step-4---update-play-samples)
    - [Step 5 - Update playframework.com](#step-5---update-playframeworkcom)
      - [Update `.version` in `play-generated-docs`](#update-version-in-play-generated-docs)
      - [Update `playReleases.json` and `changelog.md`](#update-playreleasesjson-and-changelogmd)
      - [Deploy the website changes](#deploy-the-website-changes)
    - [Step 6 - Announce](#step-6---announce)
    - [Step 7 - Post release tasks](#step-7---post-release-tasks)

## Before the release

### Issues and pull request triage

See if there are [issues that need triage](https://github.com/issues?utf8=%E2%9C%93&q=label%3Atriage+org%3Aplayframework+archived%3Afalse+) and are possibly related to the upcoming release.
This is mainly important if you are doing a minor or major release.

## Release tracking issue

Create a new [release tracking issue](https://github.com/playframework/play-meta/issues/new?template=z_play-release.md).

## Intro

When cutting a release of Play, you need to make the following decision:

- Does this release of Play require releasing the modules that depend on it, e.g. play-grpc and play-slick.
  For a typical minor release, it would not.  For a major release, and often for pre releases of major releases, the answer is yes.

## Prerequisites

As of 2022 all repositories in the Play Framework organization are released by tag. That means the sonatype credentials are set up at the GitHub organization level.
To release from older branches however you might still have to release from your machine, that's why you have to set up the sonatype credentials locally.
See [here](https://github.com/xerial/sbt-sonatype#homesbtsbt-version-013-or-10sonatypesbt-1) how that is done (either by setting `credentials +=...` or using the env variable).

## If something goes wrong

The release process pushes all artifacts to maven central and "promote" them automatically.

If the build fails during or after the promotion of maven central artifacts, there is no going back.
Published artifacts are immutable, they find their way into CDNs, into caches on developer machines, and if there are two different versions of the same artifact out there, this can cause big problems.
Your only option is to fix the problem, and attempt another release of the next version.  Remember, version numbers are cheap.

If the build failed during or before the publishing of artifacts, but not after maven central promotion, you can drop the maven central staging repository.
This can either be done through their corresponding [web interfaces](https://oss.sonatype.org/), or by using the `sonatypeDrop` sbt commands.

The most common failure is a request timeout. Typically, you can see in the logs that all artifacts have been uploaded and the build fails with a timeout while closing the repository or promoting it. If that's the case, the easiest solution is to close and promote by hand on Sonatype.

## Before you release Play

Check when was the last time scala-steward ran on the repository and consider updating the dependencies yourself. Have a look in the [.scala-steward.conf](https://github.com/playframework/playframework/blob/main/.scala-steward.conf).

Take this with a grain of salt. It's good and nice to have a release with the latest dependencies, but dependency updates at the last minute can come with surprises.

## Releasing Play

### Step 0 - Release projects that Play depends on (play-json, play-ws, twirl)

Prepare the branch for each project:

- Look for PRs that should be merged.
- Look at `status:needs-backport` issues/PRs (including closed ones).
- Look at issues/PRs tagged milestone version (including closed ones).
- Update any dependencies that are needed.
- Update any upstream projects (e.g make sure new `play-ws` is using new `play-json`)

May need to release these projects: `play-json`, `play-ws`, `twirl`

When ready:

- If the project already has set up `sbt-ci-release`:
  - Make sure you are on the latest HEAD of the branch you want to release
  - Tag the version you want to release: `git tag -s x.y.z`
  - Push the tag to the upstream repo.
  - In the GitHub user interface, check "Publish" action, it will publish the release.
- If you release from an older branch that has no `sbt-ci-release` set up yet:
  - Make sure you are using **JDK8** to build and release!
  - Checkout the branch you want to release and check that the commit you want to release has a green build in CI
  - Tag the commit, eg: (`git tag -s x.y.z`). The projects are using `sbt-dynver`, so tagging first is important in order to get the right version number. The projects are NOT using tag prefixes (eg: v1.0.2), they use 1.0.2 instead. Watch-out, don't be mislid by twirl and play-ws. They do have tags prefixed by a `v`, but the dynver config is requiring the new tags to NOT have a prefix.
  - Run `sbt release`
  - At the end of the release, you must push the tag.
  - Check the correcponding release page in GitHub, adapt the release notes as needed and published it.

### Step 1 - Release Play itself

Prepare the branch:

- Look for PRs that should be merged.
- Look at [`status:needs-backport`](https://github.com/playframework/playframework/issues?utf8=%E2%9C%93&q=label%3Astatus%3Aneeds-backport+) issues/PRs (including closed ones). If you are releasing an older version of Play, look at the `status:needs-backport-x.x` label too.
- Look at issues/PRs tagged milestone version (including closed ones).
- Updated any dependencies that are needed (e.g. Dependencies.scala).
- Do a local build or the appropriate snapshot and use [the `local-test.sh` from `play-samples`](https://github.com/playframework/play-samples/blob/2.8.x/local-test.sh) for a final round of tests.

When ready:

- If releasing 2.9 or newer:
  - Tag the version you want to release: `git tag -s x.y.z`
  - Push the tag to the upstream repo.
  - In the GitHub user interface, check "Publish" action, it will publish the release.
- If releasing from `2.8.x`:
  - Make sure you are using **JDK8** to build and release.
  - Ccheckout the branch you want to release and check that the commit you want to release has a green build in CI
  - Tag the commit, eg: (`git tag -s x.y.z`). The projects are using sbt-dynver, so tagging first is important in order to get the right version number. Play is NOT using tag prefixes (eg: v1.0.2), it uses 1.0.2 instead and dynver is configured as such.
  - Run `sbt release`
  - At the end of the release, you must push the tag.
 - Check the corresponding release page in GitHub, adapt the release notes as needed and published it.

Once Play is released, you need to wait 10 minutes or so for a Maven central sync before you can perform any of the remaining tasks.

**Verification**:
You can check that the artifacts are available at Maven Central [under `play_<scalaversion>`](https://repo1.maven.org/maven2/com/typesafe/play/).
You can see the staged artifacts here on OSS Sonatype. E.g. [here is a search](<https://oss.sonatype.org/#nexus-search;gav~com.typesafe.play~~2.8.18~~>) for Play 2.8.18 artifacts.

**Warning**:
After pushing the a tag to GitHub, a GitHub actions workflow will be trigger.

### Step 2 - Release external modules

This includes the modules:

- play-slick
- play-ebean
- scalatestplus-play
- play-grpc

Only release these if they need to be released, generally for minor Play releases, there's no reason to cut a new release of these, these libraries are free to have their own release cycle.

**Note**: since we update omnidoc and the Play templates and seeds people reading the docs or starting a new project will automatically see and use the latest minor versions of all modules, even if we don't patch all modules directly to update dependencies.

You may need to bump the Play version in the external module, do this, commit, and depending on how major the version bump is, push directly to the repo or go through a pull request.
Once that is done, to release:

For `play-slick` and `scalatestplus-play`:

- Make sure you are using **JDK8** to build and release
- Checkout the branch you want to release and check that the commit you want to release has a green build in CI
- Tag the commit, eg: (`git tag -s x.y.z`). The projects are using sbt-dynver, so tagging first is important is important in order to get the right version number. The projects are NOT using tag prefixes (eg: v1.0.2), they use 1.0.2 instead.
- Run `sbt release`
- At the end of the release, you must push the tag.
- Check the correcponding release page in GitHub, adapt the release notes as needed and published it.

TODO: all of the above should soon become obsolete. All that process should be run by the CI build.

Again, you will need to wait 10 minutes or so for a Maven central sync before you can perform any of the remaining tasks.
In the meantime you can see the staged artifacts here on OSS Sonatype. E.g. here is a search for Play 2.8.18 artifacts:
<https://oss.sonatype.org/#nexus-search;gav~com.typesafe.play~~2.8.18~~>

**Verification**:
You can check that the artifacts are available at Maven Central under `<library>_<scalaversion>` or `<sbt-plugin>_2.12_1.0`:

- <https://repo1.maven.org/maven2/com/typesafe/play/>
- <https://repo1.maven.org/maven2/org/scalatestplus/play/>

**Verification**:
when you run sbt new playframework/play-{scala,java}-seed.g8 it should pick up the new version on Maven. Try the templates out. You may need to update them if they don't work with the new version of Play.

### Step 3 - Release omnidoc

**Warning**:
This is a compulsory step and the version X.Y.Z of omnidoc released here must match the version X.Y.Z of Play released in step 1 above.

Omnidoc builds Play's documentation from all the current versions of Play and its modules. To understand what omnidoc really does under the covers, read the [README](https://github.com/playframework/omnidoc/blob/main/README.md). Note that once omnidoc completed, you will have the docs on the machine where you run the command and you still need to push them to `play-generated-docs` (next step).

In the omnidoc build file for the branch of Play that you are releasing:

1. Update the Play version to the version of Play that you just released, and also
2. Update any external modules to their latest version that is compatible with that version of Play.

Here's an example update to the omnidoc 2.8.x branch.

```diff
$ git diff
diff --git a/project/OmnidocBuild.scala b/project/OmnidocBuild.scala
-  val playVersion       = sys.props.getOrElse("play.version",       "2.8.0")
+  val playVersion       = sys.props.getOrElse("play.version",       "2.8.1")
```

Push this changes directly to GitHub (no need for a pull request).

To release omnidoc:

- For Play 2.9 and newer we [set up `sbt-ci-release`](https://github.com/playframework/omnidoc/pull/207):
  - Create a tag for the release either by using `git tag -s` or the GitHub UI. Make sure you are on the correct branch (where you just set the correct versions described above)
  - After the tag was pushed, the GitHub actions ci workflow will do the rest.
- For Play 2.8.x you still have to release by hand:
  - make sure you are using **JDK8** to build and release
  - checkout the branch you want to release and check that the commit you want to release has a green build in CI
  - DO NOT create a tag. Omnidoc does not have a `version.sbt` file and also does not use sbt-dynver. It gets its version from Play.
  - run `sbt release`
  - at the end of the release, the commit will be tagged and you must push the tag.

**Verification**: check that the artifacts are available at Maven Central under `play-omnidoc_<scalaversion>`. It may take a few minutes. <https://repo1.maven.org/maven2/com/typesafe/play/>

Once that is done, you can update the docs on playframework.com, by running:

Checkout https://github.com/playframework/play-generated-docs and switch to the branch you are releasing.

In `play-generated-docs` checkout and after switching the branch, eg: 2.8.x, run the following:

```sh
rm -rf api/java
rm -rf api/scala
rm -rf manual
rm -rf confs
```

Followed by... 

```sh
cp -r <path-to-omnidoc>/target/scala-2.13/omnidoc/javadoc api/java
cp -r <path-to-omnidoc>/target/scala-2.13/omnidoc/scaladoc api/scala                                                                                    
cp -r <path-to-omnidoc>/target/scala-2.13/omnidoc/playdoc/manual manual
cp -r <path-to-omnidoc>/target/scala-2.13/omnidoc/playdoc/confs confs
```

Where `<path-to-omnidoc>` is the path to the omnidoc repo in your machine that you just released.

In `play-generated-docs`

```shell
git add --all
git commit -m "Documentation for <tag>"
git push origin <branch>
git tag -sm "Version <tag>" <tag>
git push origin <tag>
```

Where `<tag>` is the version you are releasing, eg: 2.8.11 and branch is the branch you are updating eg: 2.8.x

Verification: check there is a new tag `<tag>` at <https://github.com/playframework/play-generated-docs> project. It
should be on top of <https://github.com/playframework/play-generated-docs/releases>. The website should pick this
tagged version of the generated docs up to 10 minutes later. You can check that then using the following URL
pattern: `https://www.playframework.com/documentation/<tag>/Home`. For example
<https://www.playframework.com/documentation/2.7.2/Home>.

### Step 4 - Update `play-samples`

Update the Play version (and other released artifacts) in any of the [play-example projects](https://github.com/playframework/play-samples).

**Verification**: The sample repository builds can be seen at <https://github.com/playframework/play-samples/actions/workflows/build-test.yml>. Make sure the build is green and then merge the pull request.
Only continue the release procedure if the samples pass the test! If they don't it's an indicator the relase might break existing applications!

### Step 5 - Update playframework.com

These are the steps to update <https://playframework.com> website.

#### Update `.version` in `play-generated-docs`

If you are releasing a MAJOR version, review the contents of [`.version` in `play-generated-docs`](https://github.com/playframework/play-generated-docs/blob/main/.version) before  updating the site.

#### Update `playReleases.json` and `changelog.md`

Update `playReleases.json` and `changelog.md` in [playframework.com website git repository](https://github.com/playframework/playframework.com/).

Note that the changelog should be updated describing all milestone/release candidates for the same release
collectively. In addition the `playReleases.json` should be updated with the development releases in place i.e.
you don't need to publish information on both an M1 and an M2 release.  If the release is for the latest stable
or development version, upgrade the website itself to use that version.

#### Deploy the website changes

Commit and push your changes.

**NOTE**: you will need a distinct SSH public key for this.  Talk to [Matthias](https://github.com/mkurz) or someone from the steering committee if you don't have access.

To set up your public key:

1. Ask someone else from Play team to give you access the Play secrets.
2. Download the PEM file and log into the machine:

```
ssh -i PlayProd2015.pem ubuntu@ec2-100-25-201-80.compute-1.amazonaws.com
```
3. `cd playframework.com`
4. `git pull`
5. `sbt stage`
6. Restart the linux service: `sudo systemctl restart playframework.service`

**Verification**: Check that <https://www.playframework.com/changelog> contains the new release and also if the previous release moved to <https://www.playframework.com/releases>.

### Step 6 - Announce

1. If the release contains security fixes post an update on <https://groups.google.com/g/play-framework-security>
1. Publish the release on <https://github.com/playframework/playframework/releases>. There should be a release draft already.
   - Make sure to check `[x] Create a discussion for this release` before publishing the release!
1. Tweet about the new release.

**Tip**:
This shouldn't be necessary anymore because release drafter already adds all the author itself to the release notes, which will the display nicely at the bottom of a release. In case you want to list all the authors that contributed to a release you can use:

```bash
git fetch --tags && git shortlog -s 2.8.0..2.8.1 | cut -c8- | sort
```

**Verification**:
* <https://github.com/playframework/playframework/discussions/categories/announcements> - check the announcement!
* <https://twitter.com/playframework> - check the tweet!

### Step 7 - Post release tasks

1. Close the [milestone](https://github.com/playframework/playframework/milestones) for the release you just made
1. Create a [new milestone](https://github.com/playframework/playframework/milestones/new) for the next release
1. Move issues and pull requests from the old milestone to the new one if necessary
