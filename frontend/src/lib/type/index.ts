/**
 * API 回傳的伺服器回應
 */
export type Response<TData = unknown> = {
	data: TData;
	code: string;
	message: boolean;
};


export enum Status {
	Info,
	Success,
	Error
}
