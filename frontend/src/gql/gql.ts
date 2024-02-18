/* eslint-disable */
import * as types from './graphql';
import { TypedDocumentNode as DocumentNode } from '@graphql-typed-document-node/core';

/**
 * Map of all GraphQL operations in the project.
 *
 * This map has several performance disadvantages:
 * 1. It is not tree-shakeable, so it will include all operations in the project.
 * 2. It is not minifiable, so the string of a GraphQL query will be multiple times inside the bundle.
 * 3. It does not support dead code elimination, so it will add unused operations.
 *
 * Therefore it is highly recommended to use the babel or swc plugin for production.
 */
const documents = {
    "\n  subscription downloadStats {\n    downloadQueueStats {\n      pending\n      downloading\n      success\n      failed\n      skipped\n    }\n  }\n": types.DownloadStatsDocument,
    "\n  mutation downloadSite($url: String!, $from: String, $to: String) {\n    downloadSite(urlPart: $url, from: $from, to: $to)\n  }\n": types.DownloadSiteDocument,
    "\n  mutation clearQueue {\n    clearDownloadQueue\n  }\n": types.ClearQueueDocument,
    "\n  subscription downloadQueueGroups {\n    downloadQueueGroups {\n      url,\n      taskStats {\n        pending,\n        downloading,\n        success,\n        failed,\n        skipped\n      }\n    }\n  }\n": types.DownloadQueueGroupsDocument,
    "\n  query siteList {\n    siteList {\n      totalCount,\n      hosts\n  }\n}": types.SiteListDocument,
    "\n  query sitePaths($host: String!) {\n    sitePaths(host: $host){\n      timestamp\n      path\n      mime\n    }\n  }\n": types.SitePathsDocument,
};

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 *
 *
 * @example
 * ```ts
 * const query = graphql(`query GetUser($id: ID!) { user(id: $id) { name } }`);
 * ```
 *
 * The query argument is unknown!
 * Please regenerate the types.
 */
export function graphql(source: string): unknown;

/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  subscription downloadStats {\n    downloadQueueStats {\n      pending\n      downloading\n      success\n      failed\n      skipped\n    }\n  }\n"): (typeof documents)["\n  subscription downloadStats {\n    downloadQueueStats {\n      pending\n      downloading\n      success\n      failed\n      skipped\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation downloadSite($url: String!, $from: String, $to: String) {\n    downloadSite(urlPart: $url, from: $from, to: $to)\n  }\n"): (typeof documents)["\n  mutation downloadSite($url: String!, $from: String, $to: String) {\n    downloadSite(urlPart: $url, from: $from, to: $to)\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  mutation clearQueue {\n    clearDownloadQueue\n  }\n"): (typeof documents)["\n  mutation clearQueue {\n    clearDownloadQueue\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  subscription downloadQueueGroups {\n    downloadQueueGroups {\n      url,\n      taskStats {\n        pending,\n        downloading,\n        success,\n        failed,\n        skipped\n      }\n    }\n  }\n"): (typeof documents)["\n  subscription downloadQueueGroups {\n    downloadQueueGroups {\n      url,\n      taskStats {\n        pending,\n        downloading,\n        success,\n        failed,\n        skipped\n      }\n    }\n  }\n"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query siteList {\n    siteList {\n      totalCount,\n      hosts\n  }\n}"): (typeof documents)["\n  query siteList {\n    siteList {\n      totalCount,\n      hosts\n  }\n}"];
/**
 * The graphql function is used to parse GraphQL queries into a document that can be used by GraphQL clients.
 */
export function graphql(source: "\n  query sitePaths($host: String!) {\n    sitePaths(host: $host){\n      timestamp\n      path\n      mime\n    }\n  }\n"): (typeof documents)["\n  query sitePaths($host: String!) {\n    sitePaths(host: $host){\n      timestamp\n      path\n      mime\n    }\n  }\n"];

export function graphql(source: string) {
  return (documents as any)[source] ?? {};
}

export type DocumentType<TDocumentNode extends DocumentNode<any, any>> = TDocumentNode extends DocumentNode<  infer TType,  any>  ? TType  : never;