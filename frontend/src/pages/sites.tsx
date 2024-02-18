import { graphql } from "@/gql";
import { ActionIcon, Button, Title } from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { IconChevronUp, IconChevronDown } from "@tabler/icons-react";
import { DataTable } from "mantine-datatable";
import { useQuery } from "urql";

const siteListQuery = graphql(`
  query siteList {
    siteList {
      totalCount,
      hosts
  }
}`);

const sitePathsQuery = graphql(`
  query sitePaths($host: String!) {
    sitePaths(host: $host){
      timestamp
      path
      mime
    }
  }
`);

export default function Sites() {
	const [siteListRes] = useQuery({ query: siteListQuery });
	return (
		<div>
			<Title order={3}>Sites</Title>
			<div className="flex flex-col gap-3">
				{siteListRes.data?.siteList.hosts.map((host: string) => (
					<SiteInfo host={host} key={host} />
				))}
			</div>
		</div>
	);
}

function SiteInfo(props: { host: string }) {
	const [opened, { toggle }] = useDisclosure();
	const [sitePathsRes] = useQuery({
		query: sitePathsQuery,
		variables: { host: props.host },
		pause: !opened,
	});
	return (
		<div className="flex flex-col">
			<div className="flex gap-2">
				<ActionIcon onClick={toggle} variant="outline">
					{opened ? <IconChevronUp /> : <IconChevronDown />}
				</ActionIcon>
				{props.host}
				<Button
					component="a"
					href={`/web/latest/${props.host}`}
					target="_blank"
					rel="noreferrer"
					size="xs"
					variant="outline"
				>
					Latest archive
				</Button>
			</div>
			<div className={opened ? "" : "hidden"}>
				{sitePathsRes.data && (
					<DataTable
						withTableBorder
						borderRadius="sm"
						withColumnBorders
						striped
						highlightOnHover
						records={sitePathsRes.data.sitePaths}
						idAccessor={(r) => r.timestamp + r.path}
						columns={[
							{ accessor: "path" },
							{ accessor: "timestamp" },
							{ accessor: "mime" },
						]}
						height={400}
					/>
				)}
			</div>
		</div>
	);
}
