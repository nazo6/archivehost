import {
	Client,
	cacheExchange,
	fetchExchange,
	subscriptionExchange,
} from "urql";
import { createClient as createWSClient } from "graphql-ws";

const wsClient = createWSClient({
	url: `ws://${location.host}/graphql/ws`,
});

export const client = new Client({
	url: "/graphql",
	exchanges: [
		cacheExchange,
		fetchExchange,
		subscriptionExchange({
			forwardSubscription(request) {
				const input = { ...request, query: request.query || "" };
				return {
					subscribe(sink) {
						const unsubscribe = wsClient.subscribe(input, sink);
						return { unsubscribe };
					},
				};
			},
		}),
	],
});
