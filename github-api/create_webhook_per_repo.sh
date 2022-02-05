#!/bin/bash

github_token="ghp_XXX"
webhook_url="https://discord.com/api/webhooks/XXX/github"

# To fetch all the public repos:
# curl https://api.github.com/orgs/playframework/repos?per_page=100 | grep -E '^    "name"' | cut -c 14- | cut -d'"' -f1  | sort

declare -a repos=(
  'anorm'
  'cachecontrol'
  '.github'
  'interplay'
# 'jnotify'
# 'modules.playframework.com'
# 'modules.playframework.org'
  'netty-reactive-streams'
  'omnidoc'
# 'play1'
  'play-doc'
  'play-ebean'
# 'play-enhancer'
  'play-file-watch'
  'playframework'
  'playframework.com'
  'play-generated-docs'
# 'play-glassfish'
  'play-grpc'
# 'play-iteratees'
# 'play-java-chatroom-example'
# 'play-java-compile-di-example'
# 'play-java-dagger2-example'
# 'play-java-ebean-example'
# 'play-java-fileupload-example'
# 'play-java-forms-example'
# 'play-java-grpc-example'
# 'play-java-hello-world-tutorial'
# 'play-java-jpa-example'
# 'play-java-rest-api-example'
  'play-java-seed.g8'
# 'play-java-starter-example'
# 'play-java-streaming-example'
# 'play-java-websocket-example'
  'play-json'
  'play-mailer'
  'play-meta'
# 'play-native-loader'
# 'play-plugins'
# 'play-quota-java-example'
# 'play-quota-scala-example'
  'play-samples'
# 'play-scala-anorm-example'
# 'play-scala-chatroom-example'
# 'play-scala-compile-di-example'
# 'play-scala-fileupload-example'
# 'play-scala-forms-example'
# 'play-scala-grpc-example'
# 'play-scala-hello-world-tutorial'
# 'play-scala-isolated-slick-example'
# 'play-scala-log4j2-example'
# 'play-scala-macwire-di-example'
# 'play-scala-rest-api-example'
# 'play-scala-secure-session-example'
  'play-scala-seed.g8'
# 'play-scala-slick-example'
# 'play-scala-starter-example'
# 'play-scala-streaming-example'
# 'play-scala-tls-example'
# 'play-scala-websocket-example'
  'play-slick'
  'play-soap'
  'play-socket.io'
# 'play-spring-loader'
  'play-webgoat'
  'play-ws'
# 'prune'
# 'sbt-coffeescript'
  'scalatestplus-play'
# 'templatecontrol'
  'twirl'
)

for repo in "${repos[@]}"
do
  curl \
    -X POST \
    -H "Authorization: token $github_token" \
    -H "Accept: application/vnd.github.v3+json" \
    https://api.github.com/repos/playframework/$repo/hooks \
    -d '{"name":"web", "config":{"url": "'$webhook_url'", "content_type":"json"},  "events": [ "code_scanning_alert", "commit_comment", "deployment", "discussion", "discussion_comment", "gollum", "issues", "issue_comment", "milestone", "project_card", "project_column", "pull_request", "pull_request_review", "pull_request_review_comment", "pull_request_review_thread", "push", "release", "repository", "repository_import", "repository_vulnerability_alert", "secret_scanning_alert" ]}'
done
