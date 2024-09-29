import { createFetch } from 'ofetch';
import type { Response } from '$lib/type';
import { API_CODE } from '$lib/constant';
import { useResponseHandler } from './interceptor';

import type { FetchOptions } from 'ofetch';

/**
 * Singleton fetch instance
 */
let fetchInstance: ReturnType<typeof createFetch>;

/**
 * node server 對後端 api 的請求
 *
 * @template T - 回傳資料的類型
 * @param {string} url - API 的 URL
 * @param {FetchOptions} [fetchOptions] - fetch 的選項
 * @returns {Promise<Response<T>>} - API 回傳的伺服器回應
 */
export default async <T = unknown>(url: string, requestOptions?: RequestOptions): Promise<Response<T>> => {
  // TODO 改為從 env 取得
	const baseURL = 'http://localhost:3000/api'
	fetchInstance = fetchInstance || createFetch();

	const token = requestOptions?.token;
	const headers = new Headers(requestOptions?.headers);
	
	if (token) {
		headers.set('Authorization', `Bearer ${token}`);
	}
	return await fetchInstance(url, {
		baseURL,
		retry: 0,
		onResponse: useResponseHandler,
		headers,
		...requestOptions,
	})
		.then((response: Response<T>) => {
			if (response?.code !== API_CODE.OK) {
				 
				console.error( `[API ERROR] ${url}`);
				throw response;
			}

			return response;
		});
};


export type RequestOptions = FetchOptions & {
	token?: string;
}