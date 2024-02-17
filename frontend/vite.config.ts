import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig({
	plugins: [react()],
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
