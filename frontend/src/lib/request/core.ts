import { createFetch } from 'ofetch';
import type { Response } from '$lib/type';
import { API_CODE } from '$lib/constant';
import { useRequestHandler, useResponseHandler } from './interceptor';

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
export default async <T = unknown>(url: string, fetchOptions?: FetchOptions): Promise<Response<T>> => {
  // TODO 改為從 env 取得
	const baseURL = 'http://localhost:3000/api'
	fetchInstance = fetchInstance || createFetch();

	return await fetchInstance(url, {
		baseURL,
		retry: 0,
		onResponse: useResponseHandler,
		onRequest: useRequestHandler,
		...fetchOptions,
	})
		.then((response: Response<T>) => {
			if (response?.code !== API_CODE.OK) {
				 
				console.error( `[API ERROR] ${url}`);
				throw response;
			}

			return response;
		});
};
