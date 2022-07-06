# EditRepoOption

## Properties
Name | Type | Description | Notes
------------ | ------------- | ------------- | -------------
**allow_manual_merge** | **bool** | either &#x60;true&#x60; to allow mark pr as merged manually, or &#x60;false&#x60; to prevent it. &#x60;has_pull_requests&#x60; must be &#x60;true&#x60;. | [optional] [default to null]
**allow_merge_commits** | **bool** | either &#x60;true&#x60; to allow merging pull requests with a merge commit, or &#x60;false&#x60; to prevent merging pull requests with merge commits. &#x60;has_pull_requests&#x60; must be &#x60;true&#x60;. | [optional] [default to null]
**allow_rebase** | **bool** | either &#x60;true&#x60; to allow rebase-merging pull requests, or &#x60;false&#x60; to prevent rebase-merging. &#x60;has_pull_requests&#x60; must be &#x60;true&#x60;. | [optional] [default to null]
**allow_rebase_explicit** | **bool** | either &#x60;true&#x60; to allow rebase with explicit merge commits (--no-ff), or &#x60;false&#x60; to prevent rebase with explicit merge commits. &#x60;has_pull_requests&#x60; must be &#x60;true&#x60;. | [optional] [default to null]
**allow_rebase_update** | **bool** | either &#x60;true&#x60; to allow updating pull request branch by rebase, or &#x60;false&#x60; to prevent it. &#x60;has_pull_requests&#x60; must be &#x60;true&#x60;. | [optional] [default to null]
**allow_squash_merge** | **bool** | either &#x60;true&#x60; to allow squash-merging pull requests, or &#x60;false&#x60; to prevent squash-merging. &#x60;has_pull_requests&#x60; must be &#x60;true&#x60;. | [optional] [default to null]
**archived** | **bool** | set to &#x60;true&#x60; to archive this repository. | [optional] [default to null]
**autodetect_manual_merge** | **bool** | either &#x60;true&#x60; to enable AutodetectManualMerge, or &#x60;false&#x60; to prevent it. &#x60;has_pull_requests&#x60; must be &#x60;true&#x60;, Note: In some special cases, misjudgments can occur. | [optional] [default to null]
**default_branch** | **String** | sets the default branch for this repository. | [optional] [default to null]
**default_delete_branch_after_merge** | **bool** | set to &#x60;true&#x60; to delete pr branch after merge by default | [optional] [default to null]
**default_merge_style** | **String** | set to a merge style to be used by this repository: \&quot;merge\&quot;, \&quot;rebase\&quot;, \&quot;rebase-merge\&quot;, or \&quot;squash\&quot;. &#x60;has_pull_requests&#x60; must be &#x60;true&#x60;. | [optional] [default to null]
**description** | **String** | a short description of the repository. | [optional] [default to null]
**enable_prune** | **bool** | enable prune - remove obsolete remote-tracking references | [optional] [default to null]
**external_tracker** | [***::models::ExternalTracker**](ExternalTracker.md) |  | [optional] [default to null]
**external_wiki** | [***::models::ExternalWiki**](ExternalWiki.md) |  | [optional] [default to null]
**has_issues** | **bool** | either &#x60;true&#x60; to enable issues for this repository or &#x60;false&#x60; to disable them. | [optional] [default to null]
**has_projects** | **bool** | either &#x60;true&#x60; to enable project unit, or &#x60;false&#x60; to disable them. | [optional] [default to null]
**has_pull_requests** | **bool** | either &#x60;true&#x60; to allow pull requests, or &#x60;false&#x60; to prevent pull request. | [optional] [default to null]
**has_wiki** | **bool** | either &#x60;true&#x60; to enable the wiki for this repository or &#x60;false&#x60; to disable it. | [optional] [default to null]
**ignore_whitespace_conflicts** | **bool** | either &#x60;true&#x60; to ignore whitespace for conflicts, or &#x60;false&#x60; to not ignore whitespace. &#x60;has_pull_requests&#x60; must be &#x60;true&#x60;. | [optional] [default to null]
**internal_tracker** | [***::models::InternalTracker**](InternalTracker.md) |  | [optional] [default to null]
**mirror_interval** | **String** | set to a string like &#x60;8h30m0s&#x60; to set the mirror interval time | [optional] [default to null]
**name** | **String** | name of the repository | [optional] [default to null]
**private** | **bool** | either &#x60;true&#x60; to make the repository private or &#x60;false&#x60; to make it public. Note: you will get a 422 error if the organization restricts changing repository visibility to organization owners and a non-owner tries to change the value of private. | [optional] [default to null]
**template** | **bool** | either &#x60;true&#x60; to make this repository a template or &#x60;false&#x60; to make it a normal repository | [optional] [default to null]
**website** | **String** | a URL with more information about the repository. | [optional] [default to null]

[[Back to Model list]](../README.md#documentation-for-models) [[Back to API list]](../README.md#documentation-for-api-endpoints) [[Back to README]](../README.md)


