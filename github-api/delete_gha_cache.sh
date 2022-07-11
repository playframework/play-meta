#!/usr/bin/env bash

# Personal access token with scopes:
# - public_repo (Needed for the DELETE request)
github_token="ghp_XXX"
repo="playframework"

# To list all repos that currently have an existing cache (needs read:org permission!)
#curl -H 'Authorization: token $github_token' https://api.github.com/orgs/playframework/actions/cache/usage-by-repository | jq ".repository_cache_usages[].full_name" 

echo "Starting to delete all caches of repo $repo"
json_response=$(curl -s -H "Authorization: token $github_token" https://api.github.com/repos/playframework/$repo/actions/caches)
jq -n "$json_response | .actions_caches[].id" | while read id; do
  echo "Deleting cache with id $id..."
  curl -X DELETE -H "Authorization: token $github_token" https://api.github.com/repos/playframework/$repo/actions/caches/$id
done
echo
echo "Cache of repo $repo after deletion:"
curl -H "Authorization: token $github_token" https://api.github.com/repos/playframework/$repo/actions/caches
