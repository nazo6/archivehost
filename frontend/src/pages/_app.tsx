import {
	AppShell,
	Burger,
	Group,
	NavLink as NavLinkUi,
	Title,
} from "@mantine/core";
import { useDisclosure } from "@mantine/hooks";
import { NavLink, Outlet } from "react-router";
import { Link, Path } from "../router";

export default function App() {
	const [opened, { toggle }] = useDisclosure();

	return (
		<AppShell
			header={{ height: 60 }}
			navbar={{
				width: 250,
				breakpoint: "sm",
				collapsed: { mobile: !opened, desktop: !opened },
			}}
			padding="xs"
		>
			<AppShell.Header>
				<Group h="100%" px="md">
					<Burger opened={opened} onClick={toggle} size="sm" />
					<Link to="/">
						<Title order={2}>Archivehost</Title>
					</Link>
				</Group>
			</AppShell.Header>
			<AppShell.Navbar p="xs">
				<NLink to="/" label="Home" />
				<NLink to="/downloads" label="Downloads" />
				<NLink to="/queue" label="Download Queue" />
				<NLink to="/sites" label="Sites" />
			</AppShell.Navbar>
			<AppShell.Main>
				<Outlet />
			</AppShell.Main>
		</AppShell>
	);
}

function NLink(props: { to: Path; label: string }) {
	return (
		<NavLink to={props.to}>
			{({ isActive }) => (
				<NavLinkUi component="div" active={isActive} label={props.label} />
			)}
		</NavLink>
	);
}
