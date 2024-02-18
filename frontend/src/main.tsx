import React from "react";
import ReactDOM from "react-dom/client";
import { MantineProvider } from "@mantine/core";
import { Routes } from "@generouted/react-router";
import { Provider } from "urql";

import "./main.css";
import "@mantine/core/styles.css";
import "@mantine/notifications/styles.css";
import "mantine-datatable/styles.layer.css";

import { client } from "./urql-client";
import { Notifications } from "@mantine/notifications";

ReactDOM.createRoot(document.getElementById("root")!).render(
	<React.StrictMode>
		<Provider value={client}>
			<MantineProvider>
				<Notifications />
				<Routes />
			</MantineProvider>
		</Provider>
	</React.StrictMode>,
);
