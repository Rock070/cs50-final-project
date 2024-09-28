import { COOKIE_KEY } from '$lib/constant';
import Cookie from 'js-cookie';
import type { FetchOptions } from 'ofetch';

export const useRequestHandler = ((context) => {
	const token = Cookie.get(COOKIE_KEY.TOKEN);
	const { options } = context;

	const additionalHeaders = new Headers(options.headers);
	additionalHeaders.set('Authorization', `Bearer ${token}`);
	options.headers = token
		? {
			...options.headers,
			...additionalHeaders
		}
		: options.headers;
}) as FetchOptions['onRequest'];


export const useResponseHandler = ((context) => {
	const { response } = context;

	// 401 未登入
	if (response.status === 401) {
		Cookie.remove(COOKIE_KEY.TOKEN);
	}
}) as FetchOptions['onResponse'];