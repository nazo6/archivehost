/* eslint-disable */
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';
export type Maybe<T> = T | null;
export type InputMaybe<T> = Maybe<T>;
export type Exact<T extends { [key: string]: unknown }> = { [K in keyof T]: T[K] };
export type MakeOptional<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]?: Maybe<T[SubKey]> };
export type MakeMaybe<T, K extends keyof T> = Omit<T, K> & { [SubKey in K]: Maybe<T[SubKey]> };
export type MakeEmpty<T extends { [key: string]: unknown }, K extends keyof T> = { [_ in K]?: never };
export type Incremental<T> = T | { [P in keyof T]?: P extends ' $fragmentName' | '__typename' ? T[P] : never };
/** All built-in and custom scalars, mapped to their actual values */
export type Scalars = {
  ID: { input: string; output: string; }
  String: { input: string; output: string; }
  Boolean: { input: boolean; output: boolean; }
  Int: { input: number; output: number; }
  Float: { input: number; output: number; }
};

export type DownloadGroup = {
  __typename?: 'DownloadGroup';
  downloadType: DownloadType;
  failed?: Maybe<Scalars['String']['output']>;
  from?: Maybe<Scalars['Int']['output']>;
  id: Scalars['String']['output'];
  taskStats: DownloadStats;
  tasks: Array<DownloadTask>;
  to?: Maybe<Scalars['Int']['output']>;
  url: Scalars['String']['output'];
};

export type DownloadQueueQuery = {
  __typename?: 'DownloadQueueQuery';
  groups: Array<DownloadGroup>;
  stats: DownloadStats;
};

export type DownloadStats = {
  __typename?: 'DownloadStats';
  downloading: Scalars['Int']['output'];
  failed: Scalars['Int']['output'];
  pending: Scalars['Int']['output'];
  skipped: Scalars['Int']['output'];
  success: Scalars['Int']['output'];
};

export enum DownloadStatus {
  Downloading = 'DOWNLOADING',
  Failed = 'FAILED',
  Pending = 'PENDING',
  Skipped = 'SKIPPED',
  Success = 'SUCCESS'
}

/**
 * Download task.
 * Corresponds to `download_queue` table.
 */
export type DownloadTask = {
  __typename?: 'DownloadTask';
  downloadStatus: DownloadStatus;
  group: DownloadGroup;
  id: Scalars['String']['output'];
  message?: Maybe<Scalars['String']['output']>;
  mime: Scalars['String']['output'];
  statusCode?: Maybe<Scalars['Int']['output']>;
  timestamp: Scalars['Int']['output'];
  url: Scalars['String']['output'];
};

export enum DownloadType {
  Batch = 'BATCH',
  Single = 'SINGLE'
}

export type MutationRoot = {
  __typename?: 'MutationRoot';
  clearDownloadQueue: Scalars['Boolean']['output'];
  downloadSite: Scalars['Boolean']['output'];
};


export type MutationRootDownloadSiteArgs = {
  from?: InputMaybe<Scalars['String']['input']>;
  to?: InputMaybe<Scalars['String']['input']>;
  urlPart: Scalars['String']['input'];
};

export type QueryRoot = {
  __typename?: 'QueryRoot';
  downloadQueue: DownloadQueueQuery;
  siteList: SiteList;
  sitePaths: Array<SitePath>;
};


export type QueryRootSitePathsArgs = {
  host: Scalars['String']['input'];
  mime?: InputMaybe<Scalars['String']['input']>;
};

export type SiteList = {
  __typename?: 'SiteList';
  hosts: Array<Scalars['String']['output']>;
  totalCount: Scalars['Int']['output'];
};


export type SiteListHostsArgs = {
  limit?: InputMaybe<Scalars['Int']['input']>;
  offset?: InputMaybe<Scalars['Int']['input']>;
};

export type SitePath = {
  __typename?: 'SitePath';
  mime?: Maybe<Scalars['String']['output']>;
  path: Scalars['String']['output'];
  timestamp: Scalars['Int']['output'];
};

export type Subscription = {
  __typename?: 'Subscription';
  downloadQueueGroups: Array<DownloadGroup>;
  downloadQueueStats: DownloadStats;
};

export type DownloadStatsSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type DownloadStatsSubscription = { __typename?: 'Subscription', downloadQueueStats: { __typename?: 'DownloadStats', pending: number, downloading: number, success: number, failed: number, skipped: number } };

export type DownloadSiteMutationVariables = Exact<{
  url: Scalars['String']['input'];
  from?: InputMaybe<Scalars['String']['input']>;
  to?: InputMaybe<Scalars['String']['input']>;
}>;


export type DownloadSiteMutation = { __typename?: 'MutationRoot', downloadSite: boolean };

export type ClearQueueMutationVariables = Exact<{ [key: string]: never; }>;


export type ClearQueueMutation = { __typename?: 'MutationRoot', clearDownloadQueue: boolean };

export type DownloadQueueGroupsSubscriptionVariables = Exact<{ [key: string]: never; }>;


export type DownloadQueueGroupsSubscription = { __typename?: 'Subscription', downloadQueueGroups: Array<{ __typename?: 'DownloadGroup', url: string, taskStats: { __typename?: 'DownloadStats', pending: number, downloading: number, success: number, failed: number, skipped: number } }> };

export type SiteListQueryVariables = Exact<{ [key: string]: never; }>;


export type SiteListQuery = { __typename?: 'QueryRoot', siteList: { __typename?: 'SiteList', totalCount: number, hosts: Array<string> } };

export type SitePathsQueryVariables = Exact<{
  host: Scalars['String']['input'];
}>;


export type SitePathsQuery = { __typename?: 'QueryRoot', sitePaths: Array<{ __typename?: 'SitePath', timestamp: number, path: string, mime?: string | null }> };


export const DownloadStatsDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"subscription","name":{"kind":"Name","value":"downloadStats"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"downloadQueueStats"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"pending"}},{"kind":"Field","name":{"kind":"Name","value":"downloading"}},{"kind":"Field","name":{"kind":"Name","value":"success"}},{"kind":"Field","name":{"kind":"Name","value":"failed"}},{"kind":"Field","name":{"kind":"Name","value":"skipped"}}]}}]}}]} as unknown as DocumentNode<DownloadStatsSubscription, DownloadStatsSubscriptionVariables>;
export const DownloadSiteDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"downloadSite"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"url"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"from"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}},{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"to"}},"type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"downloadSite"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"urlPart"},"value":{"kind":"Variable","name":{"kind":"Name","value":"url"}}},{"kind":"Argument","name":{"kind":"Name","value":"from"},"value":{"kind":"Variable","name":{"kind":"Name","value":"from"}}},{"kind":"Argument","name":{"kind":"Name","value":"to"},"value":{"kind":"Variable","name":{"kind":"Name","value":"to"}}}]}]}}]} as unknown as DocumentNode<DownloadSiteMutation, DownloadSiteMutationVariables>;
export const ClearQueueDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"mutation","name":{"kind":"Name","value":"clearQueue"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"clearDownloadQueue"}}]}}]} as unknown as DocumentNode<ClearQueueMutation, ClearQueueMutationVariables>;
export const DownloadQueueGroupsDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"subscription","name":{"kind":"Name","value":"downloadQueueGroups"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"downloadQueueGroups"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"url"}},{"kind":"Field","name":{"kind":"Name","value":"taskStats"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"pending"}},{"kind":"Field","name":{"kind":"Name","value":"downloading"}},{"kind":"Field","name":{"kind":"Name","value":"success"}},{"kind":"Field","name":{"kind":"Name","value":"failed"}},{"kind":"Field","name":{"kind":"Name","value":"skipped"}}]}}]}}]}}]} as unknown as DocumentNode<DownloadQueueGroupsSubscription, DownloadQueueGroupsSubscriptionVariables>;
export const SiteListDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"siteList"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"siteList"},"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"totalCount"}},{"kind":"Field","name":{"kind":"Name","value":"hosts"}}]}}]}}]} as unknown as DocumentNode<SiteListQuery, SiteListQueryVariables>;
export const SitePathsDocument = {"kind":"Document","definitions":[{"kind":"OperationDefinition","operation":"query","name":{"kind":"Name","value":"sitePaths"},"variableDefinitions":[{"kind":"VariableDefinition","variable":{"kind":"Variable","name":{"kind":"Name","value":"host"}},"type":{"kind":"NonNullType","type":{"kind":"NamedType","name":{"kind":"Name","value":"String"}}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"sitePaths"},"arguments":[{"kind":"Argument","name":{"kind":"Name","value":"host"},"value":{"kind":"Variable","name":{"kind":"Name","value":"host"}}}],"selectionSet":{"kind":"SelectionSet","selections":[{"kind":"Field","name":{"kind":"Name","value":"timestamp"}},{"kind":"Field","name":{"kind":"Name","value":"path"}},{"kind":"Field","name":{"kind":"Name","value":"mime"}}]}}]}}]} as unknown as DocumentNode<SitePathsQuery, SitePathsQueryVariables>;