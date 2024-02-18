import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";
import generouted from "@generouted/react-router/plugin";
import tsconfigPaths from "vite-tsconfig-paths";

export default defineConfig({
	plugins: [react(), generouted(), tsconfigPaths()],
	server: {
		proxy: {
			"/graphql": "http://localhost:3000",
			"/graphql/ws": {
				target: "ws://localhost:3000",
				ws: true,
			},
			"/web": "http://localhost:3000",
		},
	},
});
