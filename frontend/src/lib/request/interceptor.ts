import { COOKIE_KEY } from '$lib/constant';

import type { FetchOptions } from 'ofetch';

export const useResponseHandler = (({ request, response }) => {
	const data = response._data;

}) as FetchOptions['onResponse'];

export const useRequestHandler = ((context) => {
	// const event = useEvent();
	// const token = getCookie(event, COOKIE_KEY.TOKEN);
	// const { options } = context;

	// options.headers = token
	// 	? {
	// 		...options.headers,
	// 		Authorization: `Bearer ${token}`,
	// 	}
	// 	: options.headers;
}) as FetchOptions['onRequest'];
