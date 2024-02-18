import { Button, Card, Input, Table, Title } from "@mantine/core";
import { graphql } from "@/gql";
import { useMutation, useSubscription } from "urql";
import { useState } from "react";
import { notifications } from "@mantine/notifications";

export default function Downloads() {
	return (
		<div>
			<Title order={3}>Downloads</Title>
			<div className="flex flex-col gap-3">
				<DownloadStats />
				<ClearQueue />
				<DownloadSite />
			</div>
		</div>
	);
}

const downloadStatsSubscription = graphql(/* GraphQL */ `
  subscription downloadStats {
    downloadQueueStats {
      pending
      downloading
      success
      failed
      skipped
    }
  }
`);

function DownloadStats() {
	let [res] = useSubscription({ query: downloadStatsSubscription });

	return (
		<Card shadow="sm" radius="md" withBorder>
			<Title order={4}>Download Stats</Title>
			<Table>
				<Table.Thead>
					<Table.Tr>
						<Table.Th>Type</Table.Th>
						<Table.Th>Value</Table.Th>
					</Table.Tr>
				</Table.Thead>
				<Table.Tbody>
					<Table.Tr>
						<Table.Td>Pending</Table.Td>
						<Table.Td>{res.data?.downloadQueueStats.pending ?? "-"}</Table.Td>
					</Table.Tr>
					<Table.Tr>
						<Table.Td>Downloading</Table.Td>
						<Table.Td>
							{res.data?.downloadQueueStats.downloading ?? "-"}
						</Table.Td>
					</Table.Tr>
					<Table.Tr>
						<Table.Td>Success</Table.Td>
						<Table.Td>{res.data?.downloadQueueStats.success ?? "-"}</Table.Td>
					</Table.Tr>
					<Table.Tr>
						<Table.Td>Failed</Table.Td>
						<Table.Td>{res.data?.downloadQueueStats.failed ?? "-"}</Table.Td>
					</Table.Tr>
					<Table.Tr>
						<Table.Td>Skipped</Table.Td>
						<Table.Td>{res.data?.downloadQueueStats.skipped ?? "-"}</Table.Td>
					</Table.Tr>
				</Table.Tbody>
			</Table>
		</Card>
	);
}

const downloadSiteMutation = graphql(/* GraphQL */ `
  mutation downloadSite($url: String!, $from: String, $to: String) {
    downloadSite(urlPart: $url, from: $from, to: $to)
  }
`);

function DownloadSite() {
	const [, downloadSite] = useMutation(downloadSiteMutation);
	const [url, setUrl] = useState("");
	const [from, setFrom] = useState("");
	const [to, setTo] = useState("");
	return (
		<Card shadow="sm" radius="md" withBorder>
			<Title order={4}>Site downloader</Title>
			<div className="flex flex-col gap-2">
				<Input
					className="flex-grow"
					value={url}
					onChange={(e) => setUrl(e.currentTarget.value)}
					placeholder="https://example.com"
				/>
				<div className="flex gap-2">
					<Input
						value={from}
						onChange={(e) => setFrom(e.currentTarget.value)}
						placeholder="From"
					/>
					<Input
						value={to}
						onChange={(e) => setTo(e.currentTarget.value)}
						placeholder="To"
					/>
				</div>
				<Button
					onClick={async () => {
						let fromArg = from || null;
						let toArg = to || null;
						let res = await downloadSite({ url, from: fromArg, to: toArg });
						if (res.data?.downloadSite) {
							notifications.show({
								title: "Download started",
								message: `Download of ${url} started`,
							});
						} else {
							notifications.show({
								title: "Download failed",
								message: `Download of ${url} failed`,
								color: "red",
							});
						}
					}}
				>
					Download
				</Button>
			</div>
		</Card>
	);
}

const clearQueueMutation = graphql(/* GraphQL */ `
  mutation clearQueue {
    clearDownloadQueue
  }
`);

function ClearQueue() {
	const [, clearQueue] = useMutation(clearQueueMutation);
	return (
		<Card shadow="sm" radius="md" withBorder>
			<Title className="pb-3" order={4}>
				Clear queue
			</Title>
			<Button
				color="red"
				variant="outline"
				onClick={async () => {
					let res = await clearQueue({});
					if (res.data?.clearDownloadQueue) {
						notifications.show({
							message: "Queue cleared",
						});
					} else {
						notifications.show({
							message:
								"Failed to clear queue" +
								(res.error ? `: ${res.error.message}` : ""),
						});
					}
				}}
			>
				Clear
			</Button>
		</Card>
	);
}
