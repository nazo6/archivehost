import { Card, Table, Title } from "@mantine/core";
import { graphql } from "@/gql";
import { useSubscription } from "urql";
import { DownloadQueueGroupsSubscription } from "@/gql/graphql";

const downloadQueueGroupSubscription = graphql(`
  subscription downloadQueueGroups {
    downloadQueueGroups {
      url,
      taskStats {
        pending,
        downloading,
        success,
        failed,
        skipped
      }
    }
  }
`);

export default function DownloadQueue() {
	let [res] = useSubscription({ query: downloadQueueGroupSubscription });

	return (
		<div>
			<Title order={3}>Download queue</Title>
			<div className="grid grid-cols-1 md:grid-cols-2 xl:grid-cols-3 gap-3">
				{res.data &&
					res.data.downloadQueueGroups.map((g) => (
						<DownloadGroup key={g.url} group={g} />
					))}
			</div>
		</div>
	);
}

function DownloadGroup(props: {
	group: DownloadQueueGroupsSubscription["downloadQueueGroups"][0];
}) {
	return (
		<Card shadow="sm" radius="md" withBorder>
			<Title order={4}>{props.group.url}</Title>
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
						<Table.Td>{props.group.taskStats.pending}</Table.Td>
					</Table.Tr>
					<Table.Tr>
						<Table.Td>Downloading</Table.Td>
						<Table.Td>{props.group.taskStats.downloading}</Table.Td>
					</Table.Tr>
					<Table.Tr>
						<Table.Td>Success</Table.Td>
						<Table.Td>{props.group.taskStats.success}</Table.Td>
					</Table.Tr>
					<Table.Tr>
						<Table.Td>Failed</Table.Td>
						<Table.Td>{props.group.taskStats.failed}</Table.Td>
					</Table.Tr>
					<Table.Tr>
						<Table.Td>Skipped</Table.Td>
						<Table.Td>{props.group.taskStats.skipped}</Table.Td>
					</Table.Tr>
				</Table.Tbody>
			</Table>
		</Card>
	);
}
