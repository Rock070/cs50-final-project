import { COOKIE_KEY } from '$lib/constant';
import Cookie from 'js-cookie';
import { clearUserStore } from '$lib/store/user';
import type { FetchOptions } from 'ofetch';

export const useResponseHandler = ((context) => {
	const { response } = context;
	// 401 未登入
	if (response.status === 401) {
		Cookie.remove(COOKIE_KEY.TOKEN);
		clearUserStore();
	}
}) as FetchOptions['onResponse'];